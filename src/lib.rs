use std::path::Path;
use anyhow::{anyhow, Result};

pub mod windows;
use crate::windows::*;

pub fn upgrade<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref().to_str();
    if path.is_none() {
        return Err(anyhow!("Invalid path"));
    }
    let path = path.unwrap();
    create_process(path, "./")?;
    todo!()
    // Ok(())
}

#[cfg(test)]
mod test {
    use super::upgrade;

    #[test]
    fn test() {
        upgrade("./run/upgrade.exe").unwrap();
    }
}
