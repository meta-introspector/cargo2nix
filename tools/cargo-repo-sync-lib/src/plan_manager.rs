use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::Path,
    process::{Command, Output},
    os::unix::process::ExitStatusExt,
};

use serde::{Deserialize, Serialize};
use toml_edit;
use walkdir;
use git2;

// Define a struct to represent a single task from the TOML files
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Task {
    pub name: String,
    pub description: String,
    pub status: String,
    #[serde(default)]
    pub depends_on: Vec<String>,
    #[serde(default)]
    pub command: Option<String>, // Command to execute for this task
    #[serde(default)]
    pub path: Option<String>,    // Path where the command should be executed
}

// Define a struct to represent the overall plan
#[derive(Debug, Deserialize, Serialize)]
pub struct Plan {
    pub tasks: HashMap<String, Task>, // Using HashMap for easy lookup by task name
}

// Define a trait for cargo commands
pub trait CargoCommand {
    fn needs_execution(&self, current_dir: &Path) -> Result<bool, String>;
    fn execute(&self, current_dir: &Path, log_file: &mut File) -> Result<Output, String>;
    fn dry_run(&self, current_dir: &Path, log_file: &mut File) -> Result<(), String>; // New method
}

// Helper function to map command strings to CargoCommand trait objects
pub fn get_cargo_command(command_str: &str) -> Option<Box<dyn CargoCommand>> {
    match command_str {
        "cargo update" => Some(Box::new(CargoUpdateCommand)),
        "cargo vendor" => Some(Box::new(CargoVendorCommand)),
        "cargo2nix" => Some(Box::new(Cargo2NixCommand)),
        "remove rust version constraints" => Some(Box::new(RemoveRustVersionCommand)), // Add this line
        _ => None, // Unknown command
    }
}

// Helper function to check if a file is git-ignored
fn is_git_ignored(repo_path: &Path, file_path: &Path) -> Result<bool, String> {
    let repo = git2::Repository::open(repo_path)
        .map_err(|e| format!("Failed to open Git repository at {:?}: {}", repo_path, e))?;

    let relative_path = file_path.strip_prefix(repo_path)
        .map_err(|e| format!("Failed to get relative path for {:?} from {:?}: {}", file_path, repo_path, e))?;

    repo.is_path_ignored(relative_path)
        .map_err(|e| format!("Failed to check if path {:?} is ignored: {}", relative_path, e))
}

// Struct for the 'cargo update' command
pub struct CargoUpdateCommand;

impl CargoCommand for CargoUpdateCommand {
    fn needs_execution(&self, current_dir: &Path) -> Result<bool, String> {
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
        if is_git_ignored(current_dir, &cargo_lock_path)? {
            return Err(format!(
                "Error: Cargo.lock at {:?} is ignored by Git. Please unignore it to ensure proper dependency management.",
                cargo_lock_path
            ));
        }

        println!("'cargo update' will always run to ensure consistency.");
        Ok(true)
    }

    fn execute(&self, current_dir: &Path, log_file: &mut File) -> Result<Output, String> {
        writeln!(log_file, "[COMMAND_START] cargo update in {:?}", current_dir)
            .map_err(|e| format!("Failed to write to log file: {}", e))?;
        let output = Command::new("cargo")
            .arg("update")
            .current_dir(current_dir)
            .stdout(log_file.try_clone().map_err(|e| format!("Failed to clone log file handle: {}", e))?)
            .stderr(log_file.try_clone().map_err(|e| format!("Failed to clone log file handle: {}", e))?)
            .output()
            .map_err(|e| format!("Failed to execute cargo update: {}", e))?;

        if output.status.success() {
            writeln!(log_file, "[COMMAND_STATUS] cargo update succeeded.")
                .map_err(|e| format!("Failed to write to log file: {}", e))?;
            Ok(output)
        } else {
            let stdout_str = String::from_utf8_lossy(&output.stdout);
            let stderr_str = String::from_utf8_lossy(&output.stderr);
            writeln!(log_file, "[ERROR] Stdout: {}\nStderr: {}", stdout_str, stderr_str)
                .map_err(|e| format!("Failed to write to log file: {}", e))?;
            Err(format!(
                "'cargo update' failed:\nStdout: {{}}\nStderr: {{}}{}{}",
                stdout_str,
                stderr_str
            ))
        }
    }

    fn dry_run(&self, current_dir: &Path, log_file: &mut File) -> Result<(), String> {
        let command_str = format!("cargo update");
        writeln!(log_file, "[DRY_RUN_COMMAND] Would execute command: '{}' in directory: {:?}", command_str, current_dir)
            .map_err(|e| format!("Failed to write to log file: {}", e))?;
        println!("[DRY_RUN_COMMAND] Would execute command: '{}' in directory: {:?}", command_str, current_dir);
        writeln!(log_file, "[DRY_RUN_STATUS] cargo update dry run completed.")
            .map_err(|e| format!("Failed to write to log file: {}", e))?;
        Ok(())
    }
}

// Struct for the 'cargo vendor' command
pub struct CargoVendorCommand;

impl CargoCommand for CargoVendorCommand {
    fn needs_execution(&self, current_dir: &Path) -> Result<bool, String> {
        let cargo_lock_path = current_dir.join("Cargo.lock");
        if !cargo_lock_path.exists() {
            return Err(format!("Cargo.lock not found at {:?}", cargo_lock_path));
        }

        // Check if Cargo.lock is git-ignored
        if is_git_ignored(current_dir, &cargo_lock_path)? {
            return Err(format!(
                "Error: Cargo.lock at {:?} is ignored by Git. Please unignore it to ensure proper dependency management.",
                cargo_lock_path
            ));
        }

        println!("'cargo vendor' will always run to ensure consistency.");
        Ok(true)
    }

    fn execute(&self, current_dir: &Path, log_file: &mut File) -> Result<Output, String> {
        writeln!(log_file, "[COMMAND_START] cargo vendor in {:?}", current_dir)
            .map_err(|e| format!("Failed to write to log file: {}", e))?;
        let output = Command::new("cargo")
            .arg("vendor")
            .current_dir(current_dir)
            .stdout(log_file.try_clone().map_err(|e| format!("Failed to clone log file handle: {}", e))?)
            .stderr(log_file.try_clone().map_err(|e| format!("Failed to clone log file handle: {}", e))?)
            .output()
            .map_err(|e| format!("Failed to execute cargo vendor: {}", e))?;

        if output.status.success() {
            writeln!(log_file, "[COMMAND_STATUS] cargo vendor succeeded.")
                .map_err(|e| format!("Failed to write to log file: {}", e))?;
            Ok(output)
        } else {
            let stdout_str = String::from_utf8_lossy(&output.stdout);
            let stderr_str = String::from_utf8_lossy(&output.stderr);
            writeln!(log_file, "[COMMAND_STATUS] cargo vendor failed.")
                .map_err(|e| format!("Failed to write to log file: {}", e))?;
            writeln!(log_file, "[ERROR] Stdout: {}\nStderr: {}", stdout_str, stderr_str)
                .map_err(|e| format!("Failed to write to log file: {}", e))?;
            Err(format!(
                "'cargo vendor' failed:\nStdout: {}\nStderr: {}",
                stdout_str,
                stderr_str
            ))
        }
    }

    fn dry_run(&self, current_dir: &Path, log_file: &mut File) -> Result<(), String> {
        let command_str = format!("cargo vendor");
        writeln!(log_file, "[DRY_RUN_COMMAND] Would execute command: '{}' in directory: {:?}", command_str, current_dir)
            .map_err(|e| format!("Failed to write to log file: {}", e))?;
        println!("[DRY_RUN_COMMAND] Would execute command: '{}' in directory: {:?}", command_str, current_dir);
        writeln!(log_file, "[DRY_RUN_STATUS] cargo vendor dry run completed.")
            .map_err(|e| format!("Failed to write to log file: {}", e))?;
        Ok(())
    }
}

// Struct for the 'cargo2nix' command
pub struct Cargo2NixCommand;

impl CargoCommand for Cargo2NixCommand {
    fn needs_execution(&self, current_dir: &Path) -> Result<bool, String> {
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

    fn execute(&self, current_dir: &Path, log_file: &mut File) -> Result<Output, String> {
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

        let cargo2nix_path = Path::new(cargo2nix_path_str);

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

        let output = Command::new(&cargo2nix_path)
            .arg("-o")
            .arg("Cargo.nix")
            .arg("--git-srcs") // Add this argument
            .arg("vendor")     // And this argument
            .current_dir(current_dir)
            .stdout(log_file.try_clone().map_err(|e| format!("Failed to clone log file handle: {}", e))?)
            .stderr(log_file.try_clone().map_err(|e| format!("Failed to clone log file handle: {}", e))?)
            .output()
            .map_err(|e| format!("Failed to execute cargo2nix at {:?}: {}", cargo2nix_path, e))?;

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

    fn dry_run(&self, current_dir: &Path, log_file: &mut File) -> Result<(), String> {
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

// Struct for the 'remove rust version' command
pub struct RemoveRustVersionCommand;

impl CargoCommand for RemoveRustVersionCommand {
    fn needs_execution(&self, current_dir: &Path) -> Result<bool, String> {
        // This command should always run if there are any uncommented rust-version lines
        // We can check this by grepping for uncommented lines.
        let output = Command::new("grep")
            .arg("-r")
            .arg("-e")
            .arg("^rust-version = \"[0-9.]+\"$")
            .arg("--include")
            .arg("Cargo.toml")
            .arg(current_dir)
            .output()
            .map_err(|e| format!("Failed to execute grep: {}", e))?;

        Ok(!output.stdout.is_empty())
    }

    fn execute(&self, current_dir: &Path, log_file: &mut File) -> Result<Output, String> {
        writeln!(log_file, "[COMMAND_START] Removing rust-version constraints in {:?}", current_dir)
            .map_err(|e| format!("Failed to write to log file: {}", e))?;

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
            let content = fs::read_to_string(&cargo_toml_path)
                .map_err(|e| format!("Failed to read Cargo.toml at {:?}: {}", cargo_toml_path, e))?;
            
            let new_content = content.lines()
                .map(|line| {
                    if line.trim_start().starts_with("rust-version = ") && !line.trim_start().starts_with("#") {
                        format!("# {}", line)
                    } else {
                        line.to_string()
                    }
                })
                .collect::<Vec<String>>()
                .join("\n");

            if new_content != content {
                fs::write(&cargo_toml_path, new_content)
                    .map_err(|e| format!("Failed to write to Cargo.toml at {:?}: {}", cargo_toml_path, e))?;
                writeln!(log_file, "  Commented out rust-version in {:?}", cargo_toml_path)
                    .map_err(|e| format!("Failed to write to log file: {}", e))?;
                changed_files += 1;
            }
        }

        if changed_files > 0 {
            Ok(Output { status: std::process::ExitStatus::from_raw(0), stdout: Vec::new(), stderr: Vec::new() })
        } else {
            Ok(Output { status: std::process::ExitStatus::from_raw(0), stdout: Vec::new(), stderr: Vec::new() })
        }
    }

    fn dry_run(&self, current_dir: &Path, log_file: &mut File) -> Result<(), String> {
        let command_str = format!("Remove rust-version constraints");
        writeln!(log_file, "[DRY_RUN_COMMAND] Would execute command: '{}' in directory: {:?}", command_str, current_dir)
            .map_err(|e| format!("Failed to write to log file: {}", e))?;
        println!("[DRY_RUN_COMMAND] Would execute command: '{}' in directory: {:?}", command_str, current_dir);
        writeln!(log_file, "[DRY_RUN_STATUS] Remove rust-version constraints dry run completed.")
            .map_err(|e| format!("Failed to write to log file: {}", e))?;
        Ok(())
    }
}

// Helper function to rename .cargo/config.toml
pub fn rename_cargo_config(current_dir: &Path, log_file: &mut File) -> Result<bool, String> {
    let config_path = current_dir.join(".cargo/config.toml");
    let config_bak_path = current_dir.join(".cargo/config.toml.bak");

    if config_path.exists() {
        writeln!(log_file, "Renaming {:?} to {:?}...", config_path, config_bak_path)
            .map_err(|e| format!("Failed to write to log file: {}", e))?;
        fs::rename(&config_path, &config_bak_path)
            .map_err(|e| format!("Failed to rename {:?} to {:?}: {}", config_path, config_bak_path, e))?;
        Ok(true)
    } else {
        Ok(false)
    }
}

// Helper function to restore .cargo/config.toml
pub fn restore_cargo_config(current_dir: &Path, log_file: &mut File, was_renamed: bool) -> Result<(), String> {
    if was_renamed {
        let config_path = current_dir.join(".cargo/config.toml");
        let config_bak_path = current_dir.join(".cargo/config.toml.bak");
        writeln!(log_file, "Restoring {:?} from {:?}...", config_path, config_bak_path)
            .map_err(|e| format!("Failed to write to log file: {}", e))?;
        fs::rename(&config_bak_path, &config_path)
            .map_err(|e| format!("Failed to restore {:?} from {:?}: {}", config_bak_path, config_path, e))?;
    }
    Ok(())
}