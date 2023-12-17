mod windows;

use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;
use anyhow::Result;
use crate::windows::windows::create_process;

pub fn get_default_temp_file() -> &'static str {
    "./upgrader.exe"
}

pub fn call_upgrader(temp: &str, source: &str, target: &str, runtime: &str, delete: bool, args: &Vec<&str>) -> Result<()> {
    let mut upgrader = OpenOptions::new().write(true).create(true).truncate(true).open(temp)?;
    upgrader.write_all(include_bytes!(env!("UPGRADER_WINDOWS_EXE")))?;
    upgrader.flush()?;
    drop(upgrader);
    create_process(&Command::new(temp).arg(source).arg(target).arg(runtime)
        .arg(if delete { "1" } else { "0" }).args(args), "./")?;
    Ok(())
}
