use std::env::{current_dir, current_exe};
use std::path::Path;
use anyhow::{anyhow, Result};

pub mod windows;
pub(crate) use crate::windows::*;

pub mod builder;

fn get_target<P: AsRef<Path>>(path: P) -> Result<String> {
    let target = path.as_ref().to_str();
    if target.is_none() {
        return Err(anyhow!("Invalid target path: {:?}", path.as_ref()));
    }
    Ok(target.unwrap().to_string())
}

fn get_current_exe() -> Result<String> {
    let current_exe = current_exe()?;
    let source = current_exe.to_str();
    if source.is_none() {
        return Err(anyhow!("Invalid current exe: {:?}", current_exe));
    }
    Ok(source.unwrap().to_string())
}

fn get_current_dir() -> Result<String> {
    let current_dir = current_dir()?;
    let runtime = current_dir.to_str();
    if runtime.is_none() {
        return Err(anyhow!("Invalid current dir: {:?}", current_dir));
    }
    Ok(runtime.unwrap().to_string())
}

/// Replace the current exe with the param path.
///
/// You should exit the program as soon as possible after it returns Ok.
pub fn upgrade<P: AsRef<Path>>(path: P) -> Result<()> {
    let target = get_target(path)?;
    let source = get_current_exe()?;
    let runtime = get_current_dir()?;
    call_upgrader(&get_default_temp_file(), &source, &target, &runtime, true, &Vec::new())
}
