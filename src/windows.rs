use std::ffi::CString;
use std::fs::OpenOptions;
use std::io::{Error, ErrorKind, Write};
use std::mem::size_of;
use anyhow::{Context, Result};
use encoding_rs::GB18030;
use windows::core::{PCSTR, PSTR};
use windows::Win32::System::Threading::{CREATE_NEW_CONSOLE, CreateProcessA, PROCESS_INFORMATION, STARTUPINFOA};

fn convert_to_ansi(input: &str) -> Result<CString> {
    let (converted, _, had_errors) = GB18030.encode(input);
    if had_errors {
        Err(Error::new(ErrorKind::InvalidData, "Failed to convert string to GBK.").into())
    } else {
        Ok(CString::new(converted).with_context(|| "Failed to convert string to GBK.")?)
    }
}

pub fn create_process(commandline: &str, runtime_directory: &str) -> Result<()> {
    let commandline = convert_to_ansi(commandline)?;
    let runtime_directory = convert_to_ansi(runtime_directory)?;
    let mut _process_info = PROCESS_INFORMATION::default();
    let mut _startup_info = STARTUPINFOA::default();
    _startup_info.cb = size_of::<STARTUPINFOA>() as u32;
    unsafe {
        CreateProcessA(
            PCSTR::null(),
            PSTR(commandline.as_ptr() as *mut u8),
            None,
            None,
            false,
            CREATE_NEW_CONSOLE,
            None,
            PCSTR(runtime_directory.as_ptr() as *const u8),
            &mut _startup_info,
            &mut _process_info,
        )
    }.with_context(|| format!("Failed to create process: {:?}", commandline))?;
    Ok(())
}

pub(crate) fn call_upgrader(source: &str, target: &str, runtime: &str, delete: bool, args: &Vec<&str>) -> Result<()> {
    let mut upgrader = OpenOptions::new().write(true).create(true).truncate(true).open("upgrader.exe")?;
    upgrader.write_all(include_bytes!("../windows-upgrader/target/release/windows-upgrader.exe"))?;
    upgrader.flush()?;
    drop(upgrader);
    let commandline = format!("./upgrader.exe \"{}\" \"{}\" \"{}\" {} {}", source, target, runtime, if delete { 1 } else { 0 }, args.join(" "));
    create_process(&commandline, "./")?;
    Ok(())
}
