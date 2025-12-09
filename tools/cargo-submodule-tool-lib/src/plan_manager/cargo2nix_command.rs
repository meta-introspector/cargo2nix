use super::cargo_command::CargoCommand;
use anyhow::Result;
use anyhow::Context;
use git_wrapper_lib::Execv;
use std::{
    ffi::OsStr,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    process::Output,
    sync::Arc,
};
#[cfg(feature = "toml_edit_enabled")]
use toml_edit;
#[cfg(feature = "walkdir_enabled")]
use walkdir;

// Struct for the 'cargo2nix' command
pub struct Cargo2NixCommand;

impl CargoCommand for Cargo2NixCommand {
    fn needs_execution(
        &self,
        current_dir: &Path,
        executor: Arc<dyn Execv + Send + Sync>,
    ) -> anyhow::Result<bool> {
        let cargo_toml_path = current_dir.join("Cargo.toml");
        let cargo_lock_path = current_dir.join("Cargo.lock");
        let cargo_nix_path = current_dir.join("Cargo.nix");
        let src_dir = current_dir.join("src");

        if !cargo_toml_path.exists() {
            anyhow::bail!("Cargo.toml not found at {:?}", cargo_toml_path);
        }
        if !cargo_lock_path.exists() {
            anyhow::bail!("Cargo.lock not found at {:?}", cargo_lock_path);
        }

        if !cargo_nix_path.exists() {
            println!("Cargo.nix not found, 'cargo2nix' is needed.");
            return Ok(true);
        }

        let cargo_toml_modified = fs::metadata(&cargo_toml_path)
            .context("Failed to get metadata for Cargo.toml")?
            .modified()
            .context("Failed to get modified time for Cargo.toml")?;
        let cargo_lock_modified = fs::metadata(&cargo_lock_path)
            .context("Failed to get metadata for Cargo.lock")?
            .modified()
            .context("Failed to get modified time for Cargo.lock")?;
        let cargo_nix_modified = fs::metadata(&cargo_nix_path)
            .context("Failed to get metadata for Cargo.nix")?
            .modified()
            .context("Failed to get modified time for Cargo.nix")?;

        if cargo_toml_modified > cargo_nix_modified || cargo_lock_modified > cargo_nix_modified {
            println!("Cargo.toml or Cargo.lock is newer than Cargo.nix, 'cargo2nix' is needed.");
            return Ok(true);
        }

        // Check if any Rust source files are newer than Cargo.nix
        #[cfg(feature = "walkdir_enabled")]
        if src_dir.exists() {
            for entry in walkdir::WalkDir::new(&src_dir) {
                let entry = entry.context("Error walking src directory")?;
                if entry.file_type().is_file()
                    && entry.path().extension().map_or(false, |ext| ext == "rs")
                {
                    let src_file_modified = fs::metadata(entry.path())
                        .context(format!("Failed to get metadata for {:?}", entry.path()))?
                        .modified()
                        .context(format!("Failed to get modified time for {:?}", entry.path()))?;
                    if src_file_modified > cargo_nix_modified {
                        println!(
                            "Rust source file {:?} is newer than Cargo.nix, 'cargo2nix' is needed.",
                            entry.path()
                        );
                        return Ok(true);
                    }
                }
            }
        }
        #[cfg(not(feature = "walkdir_enabled"))]
        {
            // If walkdir is not enabled, we cannot check source files, so we assume Cargo.nix is up to date
            // or that the check is not critical for a minimal build.
            println!("Walkdir feature not enabled, skipping source file check.");
        }

        println!("Cargo.nix appears up to date, 'cargo2nix' might not be strictly needed.");
        Ok(false)
    }

    fn execute(
        &self,
        current_dir: &Path,
        log_file: &mut File,
        executor: Arc<dyn Execv + Send + Sync>,
    ) -> anyhow::Result<Output> {
        #[cfg(feature = "toml_edit_enabled")]
        let cargo_toml_content = fs::read_to_string(current_dir.join("Cargo.toml"))
            .context("Failed to read Cargo.toml")?;
        #[cfg(feature = "toml_edit_enabled")]
        let cargo_toml = cargo_toml_content
            .parse::<toml_edit::Document<String>>()
            .context("Failed to parse Cargo.toml with toml_edit")?;

        #[cfg(feature = "toml_edit_enabled")]
        let cargo2nix_path_str = cargo_toml
            .get("package")
            .and_then(|p: &toml_edit::Item| p.as_table())
            .and_then(|p: &toml_edit::Table| p.get("metadata"))
            .and_then(|m: &toml_edit::Item| m.as_table())
            .and_then(|m: &toml_edit::Table| m.get("cargo2nix"))
            .and_then(|c: &toml_edit::Item| c.as_table())
            .and_then(|c: &toml_edit::Table| c.get("cargo2nix_path"))
            .and_then(|p: &toml_edit::Item| p.as_str())
            .context("cargo2nix_path not found in Cargo.toml metadata")?;

        #[cfg(not(feature = "toml_edit_enabled"))]
        let cargo2nix_path_str = {
            anyhow::bail!("toml_edit_enabled feature is required for cargo2nix command execution.");
        };

        let cargo2nix_path = PathBuf::from(cargo2nix_path_str);

        writeln!(
            log_file,
            "[COMMAND_START] {} -o Cargo.nix --git-srcs vendor in {:?}",
            cargo2nix_path_str, current_dir
        )
        .context("Failed to write to log file")?;

        // --- Improved error handling starts here ---
        if !cargo2nix_path.exists() {
            writeln!(log_file, "[COMMAND_STATUS] cargo2nix failed.")
                .context("Failed to write to log file")?;
            writeln!(
                log_file,
                "[ERROR] cargo2nix executable not found at the specified path: {:?}",
                cargo2nix_path
            )
            .context("Failed to write to log file")?;
            anyhow::bail!(
                "Error: cargo2nix executable not found at the specified path: {:?}",
                cargo2nix_path
            );
        }

        // Check if it's executable (basic check, might not cover all OS nuances)
        #[cfg(unix)] // This check is primarily for Unix-like systems
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = fs::metadata(&cargo2nix_path)
                .context(format!("Failed to get metadata for {:?}", cargo2nix_path))?;
            let permissions = metadata.permissions();
            if permissions.mode() & 0o111 == 0 {
                // Check for any execute bit
                writeln!(log_file, "[COMMAND_STATUS] cargo2nix failed.")
                    .context("Failed to write to log file")?;
                writeln!(
                    log_file,
                    "[ERROR] cargo2nix executable at {:?} does not have execute permissions.",
                    cargo2nix_path
                )
                .context("Failed to write to log file")?;
                anyhow::bail!(
                    "Error: cargo2nix executable at {:?} does not have execute permissions.",
                    cargo2nix_path
                );
            }
        }
        // --- Improved error handling ends here ---

        let output = executor
            .execv(
                cargo2nix_path.as_os_str(),
                &[
                    OsStr::new("-o"),
                    OsStr::new("Cargo.nix"),
                    OsStr::new("--git-srcs"),
                    OsStr::new("vendor"),
                ],
                Some(current_dir),
            )
            .context(format!("Failed to execute cargo2nix at {:?}", cargo2nix_path))?;

        if output.status.success() {
            writeln!(log_file, "[COMMAND_STATUS] cargo2nix succeeded.")
                .context("Failed to write to log file")?;
            Ok(output)
        } else {
            let stdout_str = String::from_utf8_lossy(&output.stdout);
            let stderr_str = String::from_utf8_lossy(&output.stderr);
            writeln!(log_file, "[COMMAND_STATUS] cargo2nix failed.")
                .context("Failed to write to log file")?;
            writeln!(
                log_file,
                "[ERROR] Stdout: {}\nStderr: {}",
                stdout_str, stderr_str
            )
            .context("Failed to write to log file")?;
            anyhow::bail!(
                "'cargo2nix' failed:\nStdout: {}\nStderr: {}",
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
        #[cfg(feature = "toml_edit_enabled")]
        let cargo_toml_content = fs::read_to_string(current_dir.join("Cargo.toml"))
            .context("Failed to read Cargo.toml")?;
        #[cfg(feature = "toml_edit_enabled")]
        let cargo_toml = cargo_toml_content
            .parse::<toml_edit::Document<String>>()
            .context("Failed to parse Cargo.toml with toml_edit")?;
        #[cfg(feature = "toml_edit_enabled")]
        let cargo2nix_path_str = cargo_toml
            .get("package")
            .and_then(|p: &toml_edit::Item| p.as_table())
            .and_then(|p: &toml_edit::Table| p.get("metadata"))
            .and_then(|m: &toml_edit::Item| m.as_table())
            .and_then(|m: &toml_edit::Table| m.get("cargo2nix"))
            .and_then(|c: &toml_edit::Item| c.as_table())
            .and_then(|c: &toml_edit::Table| c.get("cargo2nix_path"))
            .and_then(|p: &toml_edit::Item| p.as_str())
            .context("cargo2nix_path not found in Cargo.toml metadata")?;
        #[cfg(not(feature = "toml_edit_enabled"))]
        let cargo2nix_path_str = {
            anyhow::bail!("toml_edit_enabled feature is required for cargo2nix dry run.");
        };

        let command_str = format!("{} -o Cargo.nix --git-srcs vendor", cargo2nix_path_str);
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
        writeln!(log_file, "[DRY_RUN_STATUS] cargo2nix dry run completed.")
            .context("Failed to write to log file")?;
        Ok(())
    }
}
