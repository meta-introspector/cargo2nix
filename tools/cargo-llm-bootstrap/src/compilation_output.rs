use std::path::{Path, PathBuf};
use std::process::Output;
use std::time::Duration;

use chrono::Utc;
use sha2::{Digest, Sha256};
use crate::error::AppError;
use crate::results::CompilationResult;
use crate::config::CompilerConfig;

pub fn process_rustc_output(
    command_output: Output,
    crate_root_path: &Path,
    config: &CompilerConfig,
    crate_name: &str,
    source_checksum: String,
    duration: Duration,
) -> Result<CompilationResult, AppError> {
    let stdout = String::from_utf8_lossy(&command_output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&command_output.stderr).to_string();
    let exit_code = command_output.status.code();

    // Get rustc version
    let rustc_version_output = Command::new(config.rustc_path.as_ref().ok_or_else(|| AppError::Custom("rustc_path is not provided in config.".to_string()))?)
        .arg("--version")
        .output()
        .map_err(AppError::Io)?;
    let rustc_version = String::from_utf8_lossy(&rustc_version_output.stdout).trim().to_string();

    let mut compiled_checksum: Option<String> = None;

    // Determine the output directory
    let actual_output_dir = if let Some(out_dir) = &config.output_dir {
        out_dir.clone()
    } else {
        // Default to target/debug if no output_dir is specified
        crate_root_path.join("target/debug")
    };

    // Construct the expected .rlib path
    let rlib_file_name = format!("lib{}.rlib", crate_name.replace("-", "_")); // Rustc replaces hyphens with underscores
    let expected_rlib_path = actual_output_dir.join(rlib_file_name);

    if expected_rlib_path.exists() {
        compiled_checksum = Some(expected_rlib_path.to_string_lossy().to_string());
    } else {
        println!("Warning: Expected .rlib file not found at {:?}", expected_rlib_path);
    }

    Ok(CompilationResult {
        success: exit_code.unwrap_or(-1) == 0,
        output: stdout.clone(),
        error: if stderr.is_empty() { None } else { Some(stderr.clone()) },
        file_path: crate_root_path.to_path_buf(), // Now stores crate_root_path
        rustc_version,
        rustc_flags: vec![], // No specific flags passed yet, can be extended
        stdout,
        stderr,
        exit_code: exit_code.unwrap_or(-1),
        duration_ms: duration.as_millis(),
        source_checksum,
        compiled_checksum, // Now stores the actual .rlib path
        timestamp: Utc::now(),
    })
}
