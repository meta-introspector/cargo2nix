use super::cargo_command::CargoCommand;
use anyhow::Result;
use anyhow::Context;
use git_wrapper_lib::git_traits::Execv;
use std::{
    ffi::OsStr,
    fs::{self, File},
    io::Write,
    os::unix::process::ExitStatusExt, // Still needed for ExitStatus::from_raw
    path::Path,
    process::Output,
    sync::Arc,
};
use walkdir;

// Struct for the 'remove rust version' command
pub struct RemoveRustVersionCommand;

impl CargoCommand for RemoveRustVersionCommand {
    fn needs_execution(
        &self,
        current_dir: &Path,
        executor: Arc<dyn Execv + Send + Sync>,
    ) -> anyhow::Result<bool> {
        // This command should always run if there are any uncommented rust-version lines
        // We can check this by grepping for uncommented lines.
        let output = executor
            .execv(
                OsStr::new("grep"),
                &[
                    OsStr::new("-r"),
                    OsStr::new("-e"),
                    OsStr::new("^rust-version = \"[0-9.]+\"$"),
                    OsStr::new("--include"),
                    OsStr::new("Cargo.toml"),
                    current_dir.as_os_str(),
                ],
                None,
            )
            .context("Failed to execute grep")?;

        Ok(!output.stdout.is_empty())
    }

    fn execute(
        &self,
        current_dir: &Path,
        log_file: &mut File,
        executor: Arc<dyn Execv + Send + Sync>,
    ) -> anyhow::Result<Output> {
        writeln!(
            log_file,
            "[COMMAND_START] Removing rust-version constraints in {:?}",
            current_dir
        )
        .context("Failed to write to log file")?;

        // Find all Cargo.toml files
        let mut cargo_tomls_to_process = Vec::new();
        for entry in walkdir::WalkDir::new(current_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file() && e.file_name() == "Cargo.toml")
        {
            cargo_tomls_to_process.push(entry.path().to_path_buf());
        }

        let mut changed_files = 0;
        for cargo_toml_path in cargo_tomls_to_process {
            let content = fs::read_to_string(&cargo_toml_path).context(format!(
                "Failed to read Cargo.toml at {:?}",
                cargo_toml_path
            ))?;

            let new_content = content
                .lines()
                .map(|line| {
                    if line.trim_start().starts_with("rust-version = ")
                        && !line.trim_start().starts_with("#")
                    {
                        format!("# {}", line)
                    } else {
                        line.to_string()
                    }
                })
                .collect::<Vec<String>>()
                .join("\n");

            if new_content != content {
                fs::write(&cargo_toml_path, new_content).context(format!(
                    "Failed to write to Cargo.toml at {:?}",
                    cargo_toml_path
                ))?;
                writeln!(
                    log_file,
                    "  Commented out rust-version in {:?}",
                    cargo_toml_path
                )
                .context("Failed to write to log file")?;
                changed_files += 1;
            }
        }

        if changed_files > 0 {
            Ok(Output {
                status: std::process::ExitStatus::from_raw(0),
                stdout: Vec::new(),
                stderr: Vec::new(),
            })
        } else {
            Ok(Output {
                status: std::process::ExitStatus::from_raw(0),
                stdout: Vec::new(),
                stderr: Vec::new(),
            })
        }
    }

    fn dry_run(
        &self,
        current_dir: &Path,
        log_file: &mut File,
        executor: Arc<dyn Execv + Send + Sync>,
    ) -> anyhow::Result<()> {
        let command_str = format!("Remove rust-version constraints");
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
        writeln!(
            log_file,
            "[DRY_RUN_STATUS] Remove rust-version constraints dry run completed."
        )
        .context("Failed to write to log file")?;
        Ok(())
    }
}
