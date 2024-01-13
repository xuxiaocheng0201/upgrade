use std::env::current_exe;
use std::fs::remove_file;
use std::path::Path;
use std::process::Command;
use anyhow::{anyhow, Result};
use self_replace::self_replace;

pub mod builder;

pub extern crate anyhow;
pub extern crate self_replace;

/// Please consider using `upgrade::builder::Builder`
#[doc(hidden)]
pub fn run_upgrade<P: AsRef<Path>>(source: P, delete: bool, args: &Vec<&str>) -> Result<()> {
    let source = source.as_ref();
    self_replace(source)?;
    let current_exe = current_exe()?;
    let current_exe = current_exe.to_str()
        .ok_or(anyhow!("Invalid current exe: {:?}", current_exe))?;
    Command::new(current_exe).args(args).spawn()?;
    if delete { remove_file(source)?; }
    Ok(())
}

/// Replace the current exe with the param path.
///
/// You may exit the program as soon as possible after it returns Ok.
pub fn upgrade<P: AsRef<Path>>(path: P) -> Result<()> {
    run_upgrade(path, true, &Vec::new())
}
