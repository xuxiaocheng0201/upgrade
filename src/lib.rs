use std::env::current_exe;
use std::fs::remove_file;
use std::path::Path;
use anyhow::{anyhow, Result};
use self_replace::self_replace;

pub mod windows;
pub use crate::windows::new_process;

pub mod builder;

pub extern crate anyhow;
pub extern crate self_replace;

fn run_upgrade(source: &str, delete: bool, args: &Vec<&str>) -> Result<()> {
    self_replace(source)?;
    let current_exe = current_exe()?;
    new_process(current_exe.to_str()
                    .ok_or(anyhow!("Invalid current exe: {:?}", current_exe))?,
                args)?;
    if delete {
        remove_file(source)?;
    }
    Ok(())
}

/// Replace the current exe with the param path.
///
/// You should exit the program as soon as possible after it returns Ok.
pub fn upgrade<P: AsRef<Path>>(path: P) -> Result<()> {
    run_upgrade(path.as_ref().to_str()
                    .ok_or(anyhow!("Invalid path: {:?}", path.as_ref()))?,
                true,
                &Vec::new())
}
