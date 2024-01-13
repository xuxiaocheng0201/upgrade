use std::path::{Path, PathBuf};
use std::process::exit;
use anyhow::Result;
use crate::run_upgrade;

/// A builder to config how to upgrade.
/// ```no_run
/// use upgrader::builder::Builder;
///
/// fn main() {
///     Builder::create().unwrap()
///         .source(&"./upgrade.exe")
///         .upgrade().unwrap();
/// }
/// ```
pub struct Builder<'a> {
    source: Option<PathBuf>,
    args: Vec<&'a str>,
    delete: bool,
    exit: Option<i32>,
}

impl<'a> Builder<'a> {
    pub fn create() -> Result<Builder<'a>> {
        Ok(Builder {
            source: None,
            delete: true,
            args: Vec::new(),
            exit: None,
        })
    }

    /// Set the new version file.
    pub fn source<P: AsRef<Path>>(&mut self, source: P) -> &mut Builder<'a> {
        self.source = Some(source.as_ref().to_path_buf());
        self
    }

    /// Set the args when calling new version file.
    pub fn args(&mut self, args: Vec<&'a str>) -> &mut Builder<'a> {
        self.args = args;
        self
    }

    /// Set whether to delete the new version file after upgrading.
    pub fn delete(&mut self, delete: bool) -> &mut Builder<'a> {
        self.delete = delete;
        self
    }

    /// Set the exit code after calling upgrade.
    pub fn exit(&mut self, exit: i32) -> &mut Builder<'a> {
        self.exit = Some(exit);
        self
    }

    /// Run the upgrade process.
    /// # Panic
    /// If you don't set the source file.
    pub fn upgrade(&self) -> Result<()> {
        if self.source.is_none() {
            panic!("No upgrade source specified.");
        }
        run_upgrade(self.source.as_ref().unwrap(), self.delete, &self.args)?;
        if let Some(code) = self.exit {
            exit(code);
        }
        Ok(())
    }
}
