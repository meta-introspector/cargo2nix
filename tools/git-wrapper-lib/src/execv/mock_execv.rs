#[cfg(feature = "anyhow_enabled")]
use anyhow::Result;
#[cfg(not(feature = "anyhow_enabled"))]
use std::error::Error;
use std::os::unix::process::ExitStatusExt; // Added for from_raw // For fallback Result
#[cfg(not(feature = "anyhow_enabled"))]
type Result<T> = std::result::Result<T, Box<dyn Error>>; // Fallback for Result
use std::ffi::OsStr;
use std::path::Path;
use std::process::Output;

use crate::git_traits::Execv;

pub struct MockExecv;

impl Execv for MockExecv {
    fn execv(
        &self,
        _program: &OsStr,
        _args: &[&OsStr],
        _current_dir: Option<&Path>,
    ) -> Result<Output> {
        // Mock implementation: return a dummy successful output
        Ok(Output {
            status: std::process::ExitStatus::from_raw(0), // Success
            stdout: b"mock stdout".to_vec(),
            stderr: b"mock stderr".to_vec(),
        })
    }
}
