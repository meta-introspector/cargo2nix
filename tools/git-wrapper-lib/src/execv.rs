#[cfg(feature = "anyhow_enabled")]
use anyhow::Result;
#[cfg(not(feature = "anyhow_enabled"))]
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>; // Fallback for Result

use std::ffi::OsStr;
use std::path::Path;
use std::process::{Command, Output};

use crate::git_traits::Execv;

pub struct RealExecv;

impl Execv for RealExecv {
    fn execv(
        &self,
        program: &OsStr,
        args: &[&OsStr],
        current_dir: Option<&Path>,
    ) -> Result<Output> {
        let mut command = Command::new(program);
        command.args(args);
        if let Some(dir) = current_dir {
            command.current_dir(dir);
        }
        Ok(command.output()?)
    }
}

pub mod mock_execv;
