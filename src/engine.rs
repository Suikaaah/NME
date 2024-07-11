use std::ffi::c_void;
use windows::Win32::System::Diagnostics::Debug as windbg;
use windows::Win32::UI::WindowsAndMessaging as winmsg;
use windows::Win32::System::Threading as winthr;
use windows::Win32::Foundation::HANDLE;
use windows::core::PCSTR;

pub mod misc;
pub mod address;
pub mod skill;
pub mod data;

use data::*;

pub struct Engine {
  proc: HANDLE
}

impl Engine {
  pub fn new() -> Result<Self, String> {
    let read = std::fs::read_to_string("./window_name.txt")
      .map_err(|_| "Could not find window_name.txt.")?
      .trim()
      .to_owned();

    let hwnd = unsafe {
      winmsg::FindWindowA(PCSTR::null(), misc::pcstr(read))
    }.map_err(|_| "Could not find the window specified.
                  \nEdit window_name.txt so it matches your emulator's window name.
                  \n(case sensitive, 2 spaces between \"PCSX2\" and \"1.x.x\")")?;

    let proc_id = unsafe {
      let mut proc_id: u32 = 0;
      winmsg::GetWindowThreadProcessId(hwnd, Some(&mut proc_id));
      if proc_id == 0 { Err("GetWindowThreadProcessId failed.") } else { Ok(proc_id) }
    }?;

    let proc = unsafe {
      winthr::OpenProcess(winthr::PROCESS_ALL_ACCESS, false, proc_id)
    }.map_err(|e| e.to_string())?;

    Ok(Self { proc })
  }

  pub fn bulk_read(&self, lock: &Lock, data_prev: &Data) -> Result<Data, String> {
    macro_rules! m {
      ($t: ty, $field: tt) => {
        if lock.$field {
          data_prev.$field
        } else {
          self.read::<$t>(address::$field())? as i32
        }
      };
    }

    let mut skills: [i32; data::SKILL_SLOT_COUNT] = Default::default();
    for (idx, skill) in skills.iter_mut().enumerate() {
      *skill = self.read::<SkillType>(address::skill_address(idx)?)? as i32;
    }

    Ok(Data {
      macca : m!(MaccaType, macca),
      hp    : m!(HpType   , hp),
      max_hp: m!(MaxHpType, max_hp),
      mp    : m!(MpType   , mp),
      max_mp: m!(MaxMpType, max_mp),
      exp   : m!(ExpType  , exp),
      level : m!(LevelType, level),
      st    : m!(StType   , st),
      ma    : m!(MaType   , ma),
      vi    : m!(ViType   , vi),
      ag    : m!(AgType   , ag),
      lu    : m!(LuType   , lu),
      skills
    })
  }

  pub fn bulk_write(&self, data: &Data) -> Result<(), String> {
    self.write(data.macca  as MaccaType, address::macca())?;
    self.write(data.hp     as HpType   , address::hp())?;
    self.write(data.max_hp as MaxHpType, address::max_hp())?;
    self.write(data.mp     as MpType   , address::mp())?;
    self.write(data.max_mp as MaxMpType, address::max_mp())?;
    self.write(data.exp    as ExpType  , address::exp())?;
    self.write(data.level  as LevelType, address::level())?;
    self.write(data.st     as StType   , address::st())?;
    self.write(data.ma     as MaType   , address::ma())?;
    self.write(data.vi     as ViType   , address::vi())?;
    self.write(data.ag     as AgType   , address::ag())?;
    self.write(data.lu     as LuType   , address::lu())?;

    self.write(data.skill_availability(), address::skill_availability())?;

    for (idx, &skill) in data.skills.iter().enumerate() {
      self.write(skill as SkillType, address::skill_address(idx)?)?;
    }

    Ok(())
  }

  fn read<T: Default>(&self, addr: u64) -> Result<T, String> {
    let addr = addr as *const c_void;

    let mut buffer = T::default();
    let buffer_ptr = &mut buffer as *mut T as *mut c_void;

    unsafe {
      windbg::ReadProcessMemory(self.proc, addr, buffer_ptr, std::mem::size_of::<T>(), None)
    }.map_err(|e| e.to_string())?;

    Ok(buffer)
  }

  fn write<T>(&self, val: T, addr: u64) -> Result<(), String> {
    let addr       = addr as *const c_void;
    let buffer_ptr = &val as *const T as *const c_void;

    unsafe {
      windbg::WriteProcessMemory(self.proc, addr, buffer_ptr, std::mem::size_of::<T>(), None)
    }.map_err(|e| e.to_string())?;

    Ok(())
  }
}