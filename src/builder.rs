use std::process::exit;
use anyhow::Result;
use crate::{get_current_dir, get_current_exe};
use crate::windows::call_upgrader;

pub struct Builder<'a> {
    source: Option<&'a str>,
    target: String,
    runtime: String,
    delete: bool,
    args: Vec<&'a str>,
    exit: Option<i32>,
}

impl<'a> Builder<'a> {
    pub fn new() -> Result<Builder<'a>> {
        Ok(Builder {
            source: None,
            target: get_current_exe()?,
            runtime: get_current_dir()?,
            delete: true,
            args: Vec::new(),
            exit: None,
        })
    }

    pub fn source(&mut self, source: &'a str) -> &mut Builder<'a> {
        self.source = Some(source);
        self
    }

    pub fn target(&mut self, target: &str) -> &mut Builder<'a> {
        self.target = target.to_string();
        self
    }

    pub fn runtime(&mut self, runtime: &str) -> &mut Builder<'a> {
        self.runtime = runtime.to_string();
        self
    }

    pub fn delete(&mut self, delete: bool) -> &mut Builder<'a> {
        self.delete = delete;
        self
    }

    pub fn args(&mut self, args: Vec<&'a str>) -> &mut Builder<'a> {
        self.args = args;
        self
    }

    pub fn exit(&mut self, exit: i32) -> &mut Builder<'a> {
        self.exit = Some(exit);
        self
    }

    pub fn upgrade(&self) -> Result<()> {
        if self.source.is_none() {
            panic!("No upgrade source specified.");
        }
        call_upgrader(self.source.unwrap(), &self.target, &self.runtime, self.delete, &self.args)?;
        if let Some(code) = self.exit {
            exit(code);
        }
        Ok(())
    }
}
