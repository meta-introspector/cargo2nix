use super::cargo_command::CargoCommand;
use super::is_git_ignored::is_git_ignored;
use anyhow::Result;
use anyhow::Context;
use git_wrapper_lib::git_traits::Execv;
use std::{ffi::OsStr, fs::File, io::Write, path::Path, process::Output, sync::Arc};

// Struct for the 'cargo vendor' command
pub struct CargoVendorCommand;

impl CargoCommand for CargoVendorCommand {
    fn needs_execution(
        &self,
        current_dir: &Path,
        executor: Arc<dyn Execv + Send + Sync>,
    ) -> anyhow::Result<bool> {
        let cargo_lock_path = current_dir.join("Cargo.lock");
        if !cargo_lock_path.exists() {
            anyhow::bail!("Cargo.lock not found at {:?}", cargo_lock_path);
        }

        // Check if Cargo.lock is git-ignored
        if is_git_ignored(current_dir, &cargo_lock_path, executor.clone())? {
            anyhow::bail!(
                "Error: Cargo.lock at {:?} is ignored by Git. Please unignore it to ensure proper dependency management.",
                cargo_lock_path
            );
        }

        println!("'cargo vendor' will always run to ensure consistency.");
        Ok(true)
    }

    fn execute(
        &self,
        current_dir: &Path,
        log_file: &mut File,
        executor: Arc<dyn Execv + Send + Sync>,
    ) -> anyhow::Result<Output> {
        writeln!(
            log_file,
            "[COMMAND_START] cargo vendor in {:?}",
            current_dir
        )
        .context("Failed to write to log file")?;
        let output = executor
            .execv(
                OsStr::new("cargo"),
                &[OsStr::new("vendor")],
                Some(current_dir),
            )
            .context("Failed to execute cargo vendor")?;

        if output.status.success() {
            writeln!(log_file, "[COMMAND_STATUS] cargo vendor succeeded.")
                .context("Failed to write to log file")?;
            Ok(output)
        } else {
            let stdout_str = String::from_utf8_lossy(&output.stdout);
            let stderr_str = String::from_utf8_lossy(&output.stderr);
            writeln!(log_file, "[COMMAND_STATUS] cargo vendor failed.")
                .context("Failed to write to log file")?;
            writeln!(
                log_file,
                "[ERROR] Stdout: {}
Stderr: {}",
                stdout_str, stderr_str
            )
            .context("Failed to write to log file")?;
            anyhow::bail!(
                "'cargo vendor' failed:\nStdout: {}
Stderr: {}",
                stdout_str, stderr_str
            )
        }
    }

    fn dry_run(
        &self,
        current_dir: &Path,
        log_file: &mut File,
        executor: Arc<dyn Execv + Send + Sync>,
    ) -> anyhow::Result<()> {
        let command_str = format!("cargo vendor");
        writeln!(
            log_file,
            "[DRY_RUN_COMMAND] Would execute command: '{}' in directory: {:?}",
            command_str, current_dir
        )
        .context("Failed to write to log file")?;
        println!(
            "[DRY_RUN_COMMAND] Would execute command: '{}' in directory: {:?}",
            command_str, current_dir
        );
        writeln!(log_file, "[DRY_RUN_STATUS] cargo vendor dry run completed.")
            .context("Failed to write to log file")?;
        Ok(())
    }
}
