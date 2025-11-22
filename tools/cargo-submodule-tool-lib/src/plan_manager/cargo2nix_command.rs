use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    process::Output,
    ffi::OsStr,
    sync::Arc,
};
use anyhow::Result;
use git_wrapper_lib::git_traits::Execv;
use super::cargo_command::CargoCommand;
use toml_edit;
use walkdir;

// Struct for the 'cargo2nix' command
pub struct Cargo2NixCommand;

impl CargoCommand for Cargo2NixCommand {
    fn needs_execution(&self, current_dir: &Path, executor: Arc<dyn Execv + Send + Sync>) -> Result<bool, String> {
        let cargo_toml_path = current_dir.join("Cargo.toml");
        let cargo_lock_path = current_dir.join("Cargo.lock");
        let cargo_nix_path = current_dir.join("Cargo.nix");
        let src_dir = current_dir.join("src");

        if !cargo_toml_path.exists() {
            return Err(format!("Cargo.toml not found at {:?}", cargo_toml_path));
        }
        if !cargo_lock_path.exists() {
            return Err(format!("Cargo.lock not found at {:?}", cargo_lock_path));
        }

        if !cargo_nix_path.exists() {
            println!("Cargo.nix not found, 'cargo2nix' is needed.");
            return Ok(true);
        }

        let cargo_toml_modified = fs::metadata(&cargo_toml_path)
            .map_err(|e| format!("Failed to get metadata for Cargo.toml: {}", e))?.modified()
            .map_err(|e| format!("Failed to get modified time for Cargo.toml: {}", e))?;
        let cargo_lock_modified = fs::metadata(&cargo_lock_path)
            .map_err(|e| format!("Failed to get metadata for Cargo.lock: {}", e))?.modified()
            .map_err(|e| format!("Failed to get modified time for Cargo.lock: {}", e))?;
        let cargo_nix_modified = fs::metadata(&cargo_nix_path)
            .map_err(|e| format!("Failed to get metadata for Cargo.nix: {}", e))?.modified()
            .map_err(|e| format!("Failed to get modified time for Cargo.nix: {}", e))?;

        if cargo_toml_modified > cargo_nix_modified || cargo_lock_modified > cargo_nix_modified {
            println!("Cargo.toml or Cargo.lock is newer than Cargo.nix, 'cargo2nix' is needed.");
            return Ok(true);
        }

        // Check if any Rust source files are newer than Cargo.nix
        if src_dir.exists() {
            for entry in walkdir::WalkDir::new(&src_dir) {
                let entry = entry.map_err(|e| format!("Error walking src directory: {}", e))?;
                if entry.file_type().is_file() && entry.path().extension().map_or(false, |ext| ext == "rs") {
                    let src_file_modified = fs::metadata(entry.path())
                        .map_err(|e| format!("Failed to get metadata for {:?}: {}", entry.path(), e))?.modified()
                        .map_err(|e| format!("Failed to get modified time for {:?}: {}", entry.path(), e))?;
                    if src_file_modified > cargo_nix_modified {
                        println!("Rust source file {:?} is newer than Cargo.nix, 'cargo2nix' is needed.", entry.path());
                        return Ok(true);
                    }
                }
            }
        }

        println!("Cargo.nix appears up to date, 'cargo2nix' might not be strictly needed.");
        Ok(false)
    }

    fn execute(&self, current_dir: &Path, log_file: &mut File, executor: Arc<dyn Execv + Send + Sync>) -> Result<Output, String> {
        let cargo_toml_content = fs::read_to_string(current_dir.join("Cargo.toml"))
            .map_err(|e| format!("Failed to read Cargo.toml: {}", e))?;
        
        let cargo_toml = cargo_toml_content.parse::<toml_edit::Document<String>>()
            .map_err(|e| format!("Failed to parse Cargo.toml with toml_edit: {}", e))?;

        let cargo2nix_path_str = cargo_toml
            .get("package")
            .and_then(|p| p.as_table())
            .and_then(|p| p.get("metadata"))
            .and_then(|m| m.as_table())
            .and_then(|m| m.get("cargo2nix"))
            .and_then(|c| c.as_table())
            .and_then(|c| c.get("cargo2nix_path"))
            .and_then(|p| p.as_str())
            .ok_or_else(|| "cargo2nix_path not found in Cargo.toml metadata".to_string())?;

        let cargo2nix_path = PathBuf::from(cargo2nix_path_str);

        writeln!(log_file, "[COMMAND_START] {} -o Cargo.nix --git-srcs vendor in {:?}", cargo2nix_path_str, current_dir)
            .map_err(|e| format!("Failed to write to log file: {}", e))?;

        // --- Improved error handling starts here ---
        if !cargo2nix_path.exists() {
            writeln!(log_file, "[COMMAND_STATUS] cargo2nix failed.").map_err(|e| e.to_string())?;
            writeln!(log_file, "[ERROR] cargo2nix executable not found at the specified path: {:?}", cargo2nix_path).map_err(|e| e.to_string())?;
            return Err(format!(
                "Error: cargo2nix executable not found at the specified path: {:?}",
                cargo2nix_path
            ));
        }

        // Check if it's executable (basic check, might not cover all OS nuances)
        #[cfg(unix)] // This check is primarily for Unix-like systems
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = fs::metadata(&cargo2nix_path)
                .map_err(|e| format!("Failed to get metadata for {:?}: {}", cargo2nix_path, e))?;
            let permissions = metadata.permissions();
            if permissions.mode() & 0o111 == 0 { // Check for any execute bit
                writeln!(log_file, "[COMMAND_STATUS] cargo2nix failed.").map_err(|e| e.to_string())?;
                writeln!(log_file, "[ERROR] cargo2nix executable at {:?} does not have execute permissions.", cargo2nix_path).map_err(|e| e.to_string())?;
                return Err(format!(
                    "Error: cargo2nix executable at {:?} does not have execute permissions.",
                    cargo2nix_path
                ));
            }
        }
        // --- Improved error handling ends here ---

        let output = executor.execv(
            cargo2nix_path.as_os_str(),
            &[OsStr::new("-o"), OsStr::new("Cargo.nix"), OsStr::new("--git-srcs"), OsStr::new("vendor")],
            Some(current_dir),
        ).map_err(|e| format!("Failed to execute cargo2nix at {:?}: {}", cargo2nix_path, e))?;

        if output.status.success() {
            writeln!(log_file, "[COMMAND_STATUS] cargo2nix succeeded.")
                .map_err(|e| format!("Failed to write to log file: {}", e))?;
            Ok(output)
        } else {
            let stdout_str = String::from_utf8_lossy(&output.stdout);
            let stderr_str = String::from_utf8_lossy(&output.stderr);
            writeln!(log_file, "[COMMAND_STATUS] cargo2nix failed.")
                .map_err(|e| format!("Failed to write to log file: {}", e))?;
            writeln!(log_file, "[ERROR] Stdout: {}\nStderr: {}", stdout_str, stderr_str)
                .map_err(|e| format!("Failed to write to log file: {}", e))?;
            Err(format!(
                "'cargo2nix' failed:\nStdout: {}\nStderr: {}",
                stdout_str,
                stderr_str
            ))
        }
    }

    fn dry_run(&self, current_dir: &Path, log_file: &mut File, executor: Arc<dyn Execv + Send + Sync>) -> Result<(), String> {
        let cargo_toml_content = fs::read_to_string(current_dir.join("Cargo.toml"))
            .map_err(|e| format!("Failed to read Cargo.toml: {}", e))?;
        
        let cargo_toml = cargo_toml_content.parse::<toml_edit::Document<String>>()
            .map_err(|e| format!("Failed to parse Cargo.toml with toml_edit: {}", e))?;

        let cargo2nix_path_str = cargo_toml
            .get("package")
            .and_then(|p| p.as_table())
            .and_then(|p| p.get("metadata"))
            .and_then(|m| m.as_table())
            .and_then(|m| m.get("cargo2nix"))
            .and_then(|c| c.as_table())
            .and_then(|c| c.get("cargo2nix_path"))
            .and_then(|p| p.as_str())
            .ok_or_else(|| "cargo2nix_path not found in Cargo.toml metadata".to_string())?;

        let command_str = format!("{} -o Cargo.nix --git-srcs vendor", cargo2nix_path_str);
        writeln!(log_file, "[DRY_RUN_COMMAND] Would execute command: '{}' in directory: {:?}", command_str, current_dir)
            .map_err(|e| format!("Failed to write to log file: {}", e))?;
        println!("[DRY_RUN_COMMAND] Would execute command: '{}' in directory: {:?}", command_str, current_dir);
        writeln!(log_file, "[DRY_RUN_STATUS] cargo2nix dry run completed.")
            .map_err(|e| format!("Failed to write to log file: {}", e))?;
        Ok(())
    }
}
