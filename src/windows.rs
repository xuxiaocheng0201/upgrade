use std::ffi::OsString;
use std::iter::once;
use std::mem::{size_of, zeroed};
use std::os::windows::ffi::OsStrExt;
use std::process::Command;
use std::ptr::{null, null_mut};
use anyhow::{anyhow, Result};
use widestring::U16String;
use windows_sys::core::PWSTR;
use windows_sys::Win32::Foundation::{GetLastError, HLOCAL, LocalFree, WIN32_ERROR};
use windows_sys::Win32::System::Diagnostics::Debug::{FORMAT_MESSAGE_ALLOCATE_BUFFER, FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS, FormatMessageW};
use windows_sys::Win32::System::Threading::{CREATE_NEW_CONSOLE, CreateProcessW, PROCESS_INFORMATION, STARTUPINFOW};

pub unsafe fn get_error_message(code: WIN32_ERROR) -> Result<String> {
    let mut buffer = null_mut();
    let strlen = FormatMessageW(
        FORMAT_MESSAGE_ALLOCATE_BUFFER | FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
        null(),
        code,
        0,
        (&mut buffer as *mut PWSTR) as PWSTR,
        0,
        null_mut()
    );
    let message = U16String::from_ptr(buffer, strlen as usize).to_string();
    LocalFree(buffer as HLOCAL);
    Ok(message?)
}

pub fn new_process(executable: &str, arguments: &Vec<&str>) -> Result<()> {
    let mut command = Command::new(executable);
    let command = command.args(arguments);
    let program = command.get_program().encode_wide().chain(once(0)).collect::<Vec<_>>();
    let mut args = Vec::new();
    command.get_args().for_each(|arg| args.extend_from_slice(&OsString::from(&" ").encode_wide().chain(arg.encode_wide()).collect::<Vec<_>>()));
    args.extend_from_slice(&[0u16]);
    unsafe {
        let mut _process_info: PROCESS_INFORMATION = zeroed();
        let mut _startup_info: STARTUPINFOW = zeroed();
        _startup_info.cb = size_of::<STARTUPINFOW>() as u32;
        if CreateProcessW(
            program.as_ptr(),
            args.as_mut_ptr(),
            null_mut(),
            null_mut(),
            false.into(),
            CREATE_NEW_CONSOLE,
            null_mut(),
            null(),
            &_startup_info,
            &mut _process_info,
        ) != 0 {
            Ok(())
        } else {
            let message = get_error_message(GetLastError())?;
            Err(anyhow!("Failed to create process (exe=\"{}\" args={:?}): {}", executable, arguments, message))
        }
    }
}
