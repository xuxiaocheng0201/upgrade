use std::env::{current_dir, current_exe};
use std::path::Path;
use anyhow::{anyhow, Result};

pub mod windows;
use crate::windows::*;

pub fn upgrade<P: AsRef<Path>>(path: P) -> Result<()> {
    let target = path.as_ref().to_str();
    if target.is_none() {
        return Err(anyhow!("Invalid target path: {:?}", path.as_ref()));
    }
    let target = target.unwrap();
    let current_exe = current_exe()?;
    let source = current_exe.to_str();
    if source.is_none() {
        return Err(anyhow!("Invalid current exe: {:?}", current_exe));
    }
    let source = source.unwrap();
    let current_dir = current_dir()?;
    let runtime = current_dir.to_str();
    if runtime.is_none() {
        return Err(anyhow!("Invalid current dir: {:?}", current_dir));
    }
    let runtime = runtime.unwrap();
    call_upgrader(source, target, runtime, true, &Vec::new())
}

#[cfg(test)]
mod test {
    use super::upgrade;

    #[test]
    fn test() {
        upgrade("./upgrade.exe").unwrap();
    }
}
