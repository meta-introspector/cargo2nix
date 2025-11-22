use std::{
    fs::File,
    io::Write,
    path::Path,
    process::Output,
    ffi::OsStr,
    sync::Arc,
};
use anyhow::Result;
use git_wrapper_lib::git_traits::Execv;
use super::cargo_command::CargoCommand;
use super::is_git_ignored::is_git_ignored;

// Struct for the 'cargo vendor' command
pub struct CargoVendorCommand;

impl CargoCommand for CargoVendorCommand {
    fn needs_execution(&self, current_dir: &Path, executor: Arc<dyn Execv + Send + Sync>) -> Result<bool, String> {
        let cargo_lock_path = current_dir.join("Cargo.lock");
        if !cargo_lock_path.exists() {
            return Err(format!("Cargo.lock not found at {:?}", cargo_lock_path));
        }

        // Check if Cargo.lock is git-ignored
        if is_git_ignored(current_dir, &cargo_lock_path, executor.clone())? {
            return Err(format!(
                "Error: Cargo.lock at {:?} is ignored by Git. Please unignore it to ensure proper dependency management.",
                cargo_lock_path
            ));
        }

        println!("'cargo vendor' will always run to ensure consistency.");
        Ok(true)
    }

    fn execute(&self, current_dir: &Path, log_file: &mut File, executor: Arc<dyn Execv + Send + Sync>) -> Result<Output, String> {
        writeln!(log_file, "[COMMAND_START] cargo vendor in {:?}", current_dir)
            .map_err(|e| format!("Failed to write to log file: {}", e))?;
        let output = executor.execv(
            OsStr::new("cargo"),
            &[OsStr::new("vendor")],
            Some(current_dir),
        ).map_err(|e| format!("Failed to execute cargo vendor: {}", e))?;

        if output.status.success() {
            writeln!(log_file, "[COMMAND_STATUS] cargo vendor succeeded.")
                .map_err(|e| format!("Failed to write to log file: {}", e))?;
            Ok(output)
        } else {
            let stdout_str = String::from_utf8_lossy(&output.stdout);
            let stderr_str = String::from_utf8_lossy(&output.stderr);
            writeln!(log_file, "[COMMAND_STATUS] cargo vendor failed.")
                .map_err(|e| format!("Failed to write to log file: {}", e))?;
            writeln!(log_file, "[ERROR] Stdout: {}
Stderr: {}", stdout_str, stderr_str)
                .map_err(|e| format!("Failed to write to log file: {}", e))?;
            Err(format!(
                "'cargo vendor' failed:\nStdout: {}
Stderr: {}",
                stdout_str,
                stderr_str
            ))
        }
    }

    fn dry_run(&self, current_dir: &Path, log_file: &mut File, executor: Arc<dyn Execv + Send + Sync>) -> Result<(), String> {
        let command_str = format!("cargo vendor");
        writeln!(log_file, "[DRY_RUN_COMMAND] Would execute command: '{}' in directory: {:?}", command_str, current_dir)
            .map_err(|e| format!("Failed to write to log file: {}", e))?;
        println!("[DRY_RUN_COMMAND] Would execute command: '{}' in directory: {:?}", command_str, current_dir);
        writeln!(log_file, "[DRY_RUN_STATUS] cargo vendor dry run completed.")
            .map_err(|e| format!("Failed to write to log file: {}", e))?;
        Ok(())
    }
}
