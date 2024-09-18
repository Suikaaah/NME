use anyhow::{anyhow, bail, Result};
use std::ffi::{c_void, CString};
use std::fmt::Debug;
use windows::Win32::Foundation::HWND;
use windows::Win32::System::Diagnostics::Debug as windbg;
use windows::Win32::System::Threading as winthr;
use windows::Win32::UI::WindowsAndMessaging as winmsg;
use windows::{core::PCSTR, Win32::Foundation::HANDLE};

pub fn pcstr(string: impl AsRef<str>) -> Result<(PCSTR, CString)> {
    let c_string = CString::new(string.as_ref())?;
    Ok((PCSTR::from_raw(c_string.as_bytes().as_ptr()), c_string))
}

pub fn get_process(window_name: impl AsRef<str>) -> Result<HANDLE> {
    let process_id = {
        let mut process_id: u32 = 0;

        unsafe {
            let hwnd = winmsg::FindWindowA(PCSTR::null(), pcstr(window_name)?.0)
                .map_err(|_| anyhow!("Failed to find the window specified."))?;

            winmsg::GetWindowThreadProcessId(hwnd, Some(&mut process_id));
        }

        if process_id == 0 {
            bail!("Failed to get process ID.");
        } else {
            process_id
        }
    };

    let process = unsafe { winthr::OpenProcess(winthr::PROCESS_ALL_ACCESS, false, process_id) }
        .map_err(|_| anyhow!("Failed to open the process specified."))?;

    Ok(process)
}

pub fn read<T>(process: HANDLE, value: &mut T, address: u64) -> Result<()> {
    let address = address as *const c_void;
    let buffer_ptr = value as *mut T as *mut c_void;

    unsafe {
        windbg::ReadProcessMemory(process, address, buffer_ptr, std::mem::size_of::<T>(), None)
    }
    .map_err(|_| anyhow!("Failed to read memory.\nPlease make sure the game is running."))?;

    Ok(())
}

pub fn write<T>(process: HANDLE, value: &T, address: u64) -> Result<()> {
    let address = address as *const c_void;
    let buffer_ptr = value as *const T as *const c_void;

    unsafe {
        windbg::WriteProcessMemory(process, address, buffer_ptr, std::mem::size_of::<T>(), None)
    }
    .map_err(|_| anyhow!("Failed to write to memory.\nPlease make sure the game is running."))?;

    Ok(())
}

pub fn parse_address(text: impl AsRef<str>) -> Result<u64> {
    let text = text.as_ref().trim_start_matches("0x").trim();

    Ok(u64::from_str_radix(text, 16)?)
}

pub fn message_box<T, M>(title: T, message: M)
where
    T: AsRef<str>,
    M: AsRef<str>,
{
    unsafe {
        winmsg::MessageBoxA(
            HWND::default(),
            pcstr(message).unwrap().0,
            pcstr(title).unwrap().0,
            winmsg::MB_ICONERROR,
        );
    }
}

pub fn panic_window(message: impl AsRef<str>) -> ! {
    message_box("Panic", message.as_ref());
    panic!("{}", message.as_ref());
}

pub trait FancyUnwrap<T> {
    fn fancy_unwrap(self) -> T;
}

impl<T, E: Debug> FancyUnwrap<T> for std::result::Result<T, E> {
    fn fancy_unwrap(self) -> T {
        self.unwrap_or_else(|error| panic_window(format!("{error:?}")))
    }
}
