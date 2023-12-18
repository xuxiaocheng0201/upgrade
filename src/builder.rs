use std::process::exit;
use anyhow::Result;
use crate::run_upgrade;

/// A builder to config how to upgrade.
/// ```no-run
/// use upgrader::builder::Builder;
///
/// fn main() {
///     Builder::create().unwrap()
///         .source(&"./upgrade.exe")
///         .upgrade().unwrap();
/// }
/// ```
pub struct Builder<'a> {
    source: Option<&'a str>,
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
    pub fn source(&mut self, source: &'a str) -> &mut Builder<'a> {
        self.source = Some(source);
        self
    }

    /// Set the args when calling new version file.
    pub fn args(&mut self, args: Vec<&'a str>) -> &mut Builder<'a> {
        self.args = args;
        self
    }

    /// Set whether to delete the new version file.
    pub fn delete(&mut self, delete: bool) -> &mut Builder<'a> {
        self.delete = delete;
        self
    }

    /// Set the exit code after call upgrade.
    pub fn exit(&mut self, exit: i32) -> &mut Builder<'a> {
        self.exit = Some(exit);
        self
    }

    /// Run the upgrade process.
    pub fn upgrade(&self) -> Result<()> {
        if self.source.is_none() {
            panic!("No upgrade source specified.");
        }
        run_upgrade(self.source.unwrap(), self.delete, &self.args)?;
        if let Some(code) = self.exit {
            exit(code);
        }
        Ok(())
    }
}
