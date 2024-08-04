use std::ffi::c_void;
use windows::core::PCSTR;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Diagnostics::Debug as windbg;
use windows::Win32::System::Threading as winthr;
use windows::Win32::UI::WindowsAndMessaging as winmsg;

pub mod address;
pub mod data;
pub mod misc;
pub mod skill;

use data::*;

pub struct Engine {
    proc: HANDLE,
}

impl Engine {
    pub fn new() -> Result<Self, String> {
        let read = std::fs::read_to_string("./window_name.txt")
            .map_err(|_| "Could not find window_name.txt.")?
            .trim()
            .to_owned();

        let hwnd =
            unsafe { winmsg::FindWindowA(PCSTR::null(), misc::pcstr(read)) }.map_err(|_| {
                "Could not find the window specified.\nOpen and edit window_name.txt so it matches your emulator's window name."
            })?;

        let proc_id = unsafe {
            let mut proc_id: u32 = 0;
            winmsg::GetWindowThreadProcessId(hwnd, Some(&mut proc_id));
            if proc_id == 0 {
                Err("GetWindowThreadProcessId failed.")
            } else {
                Ok(proc_id)
            }
        }?;

        let proc = unsafe { winthr::OpenProcess(winthr::PROCESS_ALL_ACCESS, false, proc_id) }
            .map_err(|e| e.to_string())?;

        Ok(Self { proc })
    }

    pub fn bulk_read(&self, mut data: Data) -> Result<Data, String> {
        macro_rules! write_if_unlocked {
            ($field: tt) => {
                data.$field.write_if_unlocked(|x| {
                    *x = self.read(address::$field())?;
                    Ok(())
                })
            };
        }

        write_if_unlocked!(macca)?;
        write_if_unlocked!(hp)?;
        write_if_unlocked!(max_hp)?;
        write_if_unlocked!(mp)?;
        write_if_unlocked!(max_mp)?;
        write_if_unlocked!(exp)?;
        write_if_unlocked!(level)?;
        write_if_unlocked!(st)?;
        write_if_unlocked!(ma)?;
        write_if_unlocked!(vi)?;
        write_if_unlocked!(ag)?;
        write_if_unlocked!(lu)?;

        // skills don't have locks
        for (idx, skill) in data.skills.iter_mut().enumerate() {
            skill.write(|x| {
                *x = self.read(address::skill_address(idx)?)?;
                Ok(())
            })?;
        }

        Ok(data)
    }

    pub fn bulk_write(&self, data: &Data) -> Result<(), String> {
        macro_rules! write {
            ($field: tt) => {
                if let Some(x) = data.$field.read() {
                    self.write(x, address::$field())?;
                }
            };
        }

        write!(macca);
        write!(hp);
        write!(max_hp);
        write!(mp);
        write!(max_mp);
        write!(exp);
        write!(level);
        write!(st);
        write!(ma);
        write!(vi);
        write!(ag);
        write!(lu);

        self.write(data.skill_availability(), address::skill_availability())?;

        for (idx, skill) in data.skills.iter().enumerate() {
            if let Some(id) = skill.read() {
                self.write(id, address::skill_address(idx)?)?;
            }
        }

        Ok(())
    }

    /// ignores skills
    pub fn write_locked_only(&self, data: &Data) -> Result<(), String> {
        macro_rules! write_if_locked {
            ($field: tt) => {
                if let Some(x) = data.$field.read_if_locked() {
                    self.write(x, address::$field())?;
                }
            };
        }

        write_if_locked!(macca);
        write_if_locked!(hp);
        write_if_locked!(max_hp);
        write_if_locked!(mp);
        write_if_locked!(max_mp);
        write_if_locked!(exp);
        write_if_locked!(level);
        write_if_locked!(st);
        write_if_locked!(ma);
        write_if_locked!(vi);
        write_if_locked!(ag);
        write_if_locked!(lu);

        if data.random {
            self.write(data.skill_availability(), address::skill_availability())?;
            for (idx, skill) in data.skills.iter().enumerate() {
                if let Some(x) = skill.read() {
                    self.write(x, address::skill_address(idx)?)?;
                }
            }
        }

        Ok(())
    }

    fn read<T: Default>(&self, addr: u64) -> Result<T, String> {
        let addr = addr as *const c_void;

        let mut buffer = T::default();
        let buffer_ptr = &mut buffer as *mut T as *mut c_void;

        unsafe {
            windbg::ReadProcessMemory(self.proc, addr, buffer_ptr, std::mem::size_of::<T>(), None)
        }
        .map_err(|_| "Could not read from memory.\nPlease make sure the game is running.")?;

        Ok(buffer)
    }

    fn write<T>(&self, val: T, addr: u64) -> Result<(), String> {
        let addr = addr as *const c_void;
        let buffer_ptr = &val as *const T as *const c_void;

        unsafe {
            windbg::WriteProcessMemory(self.proc, addr, buffer_ptr, std::mem::size_of::<T>(), None)
        }
        .map_err(|_| "Could not write to memory.\nPlease make sure the game is running.")?;

        Ok(())
    }
}
