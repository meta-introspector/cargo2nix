use anyhow::{Result, Context};
use std::path::Path;
use std::process::{Command, Output};
use std::ffi::OsStr;

use crate::git_traits::Execv;

/// A concrete implementation of `Execv` that executes commands using `std::process::Command`.
pub struct SystemExecv;

impl Execv for SystemExecv {
    fn execv(&self, program: &OsStr, args: &[&OsStr], current_dir: Option<&Path>) -> Result<Output> {
        let mut command = Command::new(program);

        if let Some(dir) = current_dir {
            command.current_dir(dir);
        }

        command.args(args);

        let command_str = format!("{:?} {:?}", program, args);
        println!("[SystemExecv] Executing command: {}", command_str);

        command.output()
            .with_context(|| format!("Failed to execute command: {}", command_str))
    }
}
