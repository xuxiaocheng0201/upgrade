use std::process::exit;
use anyhow::Result;
use crate::{call_upgrader, get_current_dir, get_current_exe, get_default_temp_file};

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
    target: String,
    runtime: String,
    delete: bool,
    args: Vec<&'a str>,
    temp: String,
    exit: Option<i32>,
}

impl<'a> Builder<'a> {
    pub fn create() -> Result<Builder<'a>> {
        Ok(Builder {
            source: None,
            target: get_current_exe()?,
            runtime: get_current_dir()?,
            delete: true,
            args: Vec::new(),
            temp: get_default_temp_file().to_string(),
            exit: None,
        })
    }

    /// Set the new version file.
    pub fn source(&mut self, source: &'a str) -> &mut Builder<'a> {
        self.source = Some(source);
        self
    }

    /// Set the current version file.
    pub fn target(&mut self, target: &str) -> &mut Builder<'a> {
        self.target = target.to_string();
        self
    }

    /// Set the runtime directory of the new version file.
    pub fn runtime(&mut self, runtime: &str) -> &mut Builder<'a> {
        self.runtime = runtime.to_string();
        self
    }

    /// Set whether to delete the new version file.
    pub fn delete(&mut self, delete: bool) -> &mut Builder<'a> {
        self.delete = delete;
        self
    }

    /// Set the args when calling new version file.
    pub fn args(&mut self, args: Vec<&'a str>) -> &mut Builder<'a> {
        self.args = args;
        self
    }

    /// Set the temp upgrader file path.
    pub fn temp(&mut self, temp: &str) -> &mut Builder<'a> {
        self.temp = temp.to_string();
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
        call_upgrader(&self.temp, self.source.unwrap(), &self.target, &self.runtime, self.delete, &self.args)?;
        if let Some(code) = self.exit {
            exit(code);
        }
        Ok(())
    }
}
