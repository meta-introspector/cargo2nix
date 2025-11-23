use std::path::Path;
use std::process::Command;
use std::time::Instant;

use chrono::Utc;
use sha2::{Digest, Sha256};
use crate::error::AppError;
use crate::results::CompilationResult;
use crate::traits::Compiler;
use crate::config::CompilerConfig; // Import CompilerConfig

pub struct RustcCompilerImpl;

impl Compiler for RustcCompilerImpl {
    fn compile_file(&self, file_path: &Path, rustc_path: &Path, config: &CompilerConfig) -> Result<CompilationResult, AppError> {
        let file_content = std::fs::read(file_path).map_err(AppError::Io)?;
        let mut hasher = Sha256::new();
        hasher.update(&file_content);
        let source_checksum = format!("{:x}", hasher.finalize());

        let start_time = Instant::now();
        let mut command = Command::new(rustc_path);
        command.arg(file_path)
            .arg("--edition")
            .arg("2024")
            .arg("--crate-type")
            .arg("lib");

        if config.rustc_options.unpretty_expanded {
            command.arg("-Zunpretty=expanded");
        }

        let command_output = command.output().map_err(AppError::Io)?;
        let duration = start_time.elapsed();

        let stdout = String::from_utf8_lossy(&command_output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&command_output.stderr).to_string();
        let exit_code = command_output.status.code();

        // Get rustc version
        let rustc_version_output = Command::new(rustc_path)
            .arg("--version")
            .output()
            .map_err(AppError::Io)?;
        let rustc_version = String::from_utf8_lossy(&rustc_version_output.stdout).trim().to_string();

        Ok(CompilationResult {
            file_path: file_path.to_path_buf(),
            rustc_version,
            rustc_flags: vec![], // No specific flags passed yet, can be extended
            stdout,
            stderr,
            exit_code,
            duration_ms: duration.as_millis(),
            source_checksum,
            compiled_checksum: None, // For now, compiled_checksum is None
            timestamp: Utc::now(),
        })
    }
}
