pub extern crate self_replace;

use std::env::current_exe;
use std::ffi::OsStr;
use std::fs::remove_file;
use std::io::Result;
use std::path::Path;
use std::process::Command;

use self_replace::self_replace;

/// Replace the current exe with the source path.
///
/// You may exit the program as soon as possible after it returns Ok.
///
/// See: [`Command`] and [`Command::args`]
pub fn run_upgrade<P: AsRef<Path>, S: AsRef<OsStr>, A: IntoIterator<Item=S>>(source: P, delete: bool, args: A) -> Result<()> {
    let source = source.as_ref();
    self_replace(source)?;
    let current_exe = current_exe()?;
    Command::new(&current_exe).args(args).spawn()?;
    if delete { remove_file(source)?; }
    Ok(())
}

/// A shortcut of [`run_upgrade`] without args.
///
/// You may exit the program as soon as possible after it returns Ok.
pub fn upgrade<P: AsRef<Path>>(path: P) -> Result<()> {
    run_upgrade::<_, &str, _>(path, true, [])
}
