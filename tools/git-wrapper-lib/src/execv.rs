#[cfg(feature = "anyhow_enabled")]
use anyhow::Result;
#[cfg(not(feature = "anyhow_enabled"))]
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>; // Fallback for Result

use std::ffi::OsStr;
use std::path::Path;
use std::os::unix::process::ExitStatusExt;
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

pub mod mock_execv; // Keep this, even if it's empty for now

pub struct DummyExecv;

impl Execv for DummyExecv {
    fn execv(
        &self,
        _program: &OsStr,
        _args: &[&OsStr],
        _current_dir: Option<&Path>,
    ) -> Result<Output> {
        // Return a dummy output for testing or when actual execution is not needed
        Ok(Output {
            status: std::process::ExitStatus::from_raw(0), // Success
            stdout: "dummy stdout".as_bytes().to_vec(),
            stderr: "dummy stderr".as_bytes().to_vec(),
        })
    }
}
