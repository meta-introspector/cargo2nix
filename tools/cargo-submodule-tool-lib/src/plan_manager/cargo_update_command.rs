use super::cargo_command::CargoCommand;
use super::is_git_ignored::is_git_ignored;
use anyhow::Result;
use git_wrapper_lib::git_traits::Execv;
use std::{ffi::OsStr, fs::File, io::Write, path::Path, process::Output, sync::Arc};

// Struct for the 'cargo update' command
pub struct CargoUpdateCommand;

impl CargoCommand for CargoUpdateCommand {
    fn needs_execution(
        &self,
        current_dir: &Path,
        executor: Arc<dyn Execv + Send + Sync>,
    ) -> Result<bool, String> {
        let cargo_toml_path = current_dir.join("Cargo.toml");
        let cargo_lock_path = current_dir.join("Cargo.lock");

        if !cargo_toml_path.exists() {
            return Err(format!("Cargo.toml not found at {:?}", cargo_toml_path));
        }
        if !cargo_lock_path.exists() {
            println!("Cargo.lock not found, 'cargo update' is needed.");
            return Ok(true);
        }

        // Check if Cargo.lock is git-ignored
        if is_git_ignored(current_dir, &cargo_lock_path, executor.clone())? {
            return Err(format!(
                "Error: Cargo.lock at {:?} is ignored by Git. Please unignore it to ensure proper dependency management.",
                cargo_lock_path
            ));
        }

        println!("'cargo update' will always run to ensure consistency.");
        Ok(true)
    }

    fn execute(
        &self,
        current_dir: &Path,
        log_file: &mut File,
        executor: Arc<dyn Execv + Send + Sync>,
    ) -> Result<Output, String> {
        writeln!(
            log_file,
            "[COMMAND_START] cargo update in {:?}",
            current_dir
        )
        .map_err(|e| format!("Failed to write to log file: {}", e))?;
        let output = executor
            .execv(
                OsStr::new("cargo"),
                &[OsStr::new("update")],
                Some(current_dir),
            )
            .map_err(|e| format!("Failed to execute cargo update: {}", e))?;

        if output.status.success() {
            writeln!(log_file, "[COMMAND_STATUS] cargo update succeeded.")
                .map_err(|e| format!("Failed to write to log file: {}", e))?;
            Ok(output)
        } else {
            let stdout_str = String::from_utf8_lossy(&output.stdout);
            let stderr_str = String::from_utf8_lossy(&output.stderr);
            writeln!(
                log_file,
                "[ERROR] Stdout: {}\nStderr: {}",
                stdout_str, stderr_str
            )
            .map_err(|e| format!("Failed to write to log file: {}", e))?;
            Err(format!(
                "'cargo update' failed:\nStdout: {}\nStderr: {}",
                stdout_str, stderr_str
            ))
        }
    }

    fn dry_run(
        &self,
        current_dir: &Path,
        log_file: &mut File,
        executor: Arc<dyn Execv + Send + Sync>,
    ) -> Result<(), String> {
        let command_str = format!("cargo update");
        writeln!(
            log_file,
            "[DRY_RUN_COMMAND] Would execute command: '{}' in directory: {:?}",
            command_str, current_dir
        )
        .map_err(|e| format!("Failed to write to log file: {}", e))?;
        println!(
            "[DRY_RUN_COMMAND] Would execute command: '{}' in directory: {:?}",
            command_str, current_dir
        );
        writeln!(log_file, "[DRY_RUN_STATUS] cargo update dry run completed.")
            .map_err(|e| format!("Failed to write to log file: {}", e))?;
        Ok(())
    }
}
