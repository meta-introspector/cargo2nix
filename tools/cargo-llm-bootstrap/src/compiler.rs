use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

use chrono::Utc;
use sha2::{Digest, Sha256};
use toml;
use crate::error::AppError;
use crate::results::CompilationResult;
use crate::traits::Compiler;
use crate::config::CompilerConfig; // Import CompilerConfig

pub struct RustcCompilerImpl;

impl Compiler for RustcCompilerImpl {
    fn compile_crate(&self, crate_root_path: &Path, config: &CompilerConfig, crate_name_to_root_map: &HashMap<String, PathBuf>, compiled_artifacts_map: &HashMap<String, PathBuf>) -> Result<CompilationResult, AppError> {
        // Calculate hash of the crate (e.g., Cargo.toml + src directory)
        // For simplicity, let's just hash the Cargo.toml for now.
        let cargo_toml_path = crate_root_path.join("Cargo.toml");
        let cargo_toml_content = std::fs::read_to_string(&cargo_toml_path).map_err(AppError::Io)?;

        // Parse Cargo.toml to get dependencies
        let parsed_cargo_toml: toml::Value = toml::from_str(&cargo_toml_content).map_err(|e| AppError::Custom(format!("Failed to parse Cargo.toml for dependency extraction at {:?}: {}", cargo_toml_path, e)))?;
        let mut dependencies_to_patch: HashMap<String, String> = HashMap::new(); // Map of dep_name -> version_req

        if let Some(dependencies) = parsed_cargo_toml.get("dependencies").and_then(|d| d.as_table()) {
            for (dep_name, dep_value) in dependencies {
                if let Some(version_req) = dep_value.as_str() {
                    dependencies_to_patch.insert(dep_name.clone(), version_req.to_string());
                } else if let Some(dep_table) = dep_value.as_table() {
                    if let Some(version_req) = dep_table.get("version").and_then(|v| v.as_str()) {
                        dependencies_to_patch.insert(dep_name.clone(), version_req.to_string());
                    }
                }
            }
        }

        let file_content = std::fs::read(&cargo_toml_path).map_err(AppError::Io)?;
        let mut hasher = Sha256::new();
        hasher.update(&file_content);
        let source_checksum = format!("{:x}", hasher.finalize());

        let start_time = Instant::now();
        let mut command = Command::new(config.rustc_path.as_ref().ok_or_else(|| AppError::Custom("rustc_path is not provided in config.".to_string()))?);

        let lib_rs_path = crate_root_path.join("src/lib.rs");
        let main_rs_path = crate_root_path.join("src/main.rs");

        let target_source_file = if lib_rs_path.exists() {
            lib_rs_path
        } else if main_rs_path.exists() {
            main_rs_path
        } else {
            return Err(AppError::Custom(format!("No src/lib.rs or src/main.rs found for crate {:?}", crate_root_path)));
        };

        command.arg(&target_source_file)
            .arg("--edition")
            .arg("2024")
            .arg("--crate-type")
            .arg("lib"); // Assuming all crates are libraries for now

        // Add target triple if specified
        if let Some(target) = &config.target_triple {
            command.arg(format!("--target={}", target));
        }

        // Add output directory if specified
        if let Some(output_dir) = &config.output_dir {
            command.arg("--out-dir").arg(output_dir);
        }

        // Add rustc options
        if config.rustc_options.unpretty_expanded {
            command.arg("-Zunpretty=expanded");
        }

        // Add --extern flags for dependencies
        for (dep_name, _version_req) in &dependencies_to_patch {
            if let Some(dep_rlib_path) = compiled_artifacts_map.get(dep_name) {
                command.arg("--extern").arg(format!("{}={}", dep_name, dep_rlib_path.display()));
            } else {
                println!("Warning: Compiled artifact for dependency {} not found in map. This might cause compilation issues.", dep_name);
            }
        }

        println!("Running rustc command: {:?}", command);

        let command_output = command.output().map_err(AppError::Io)?;
        let duration = start_time.elapsed();

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

        // Extract crate name from Cargo.toml
        let crate_name = parsed_cargo_toml.get("package")
            .and_then(|p| p.get("name"))
            .and_then(|n| n.as_str())
            .ok_or_else(|| AppError::Custom(format!("Could not find package name in Cargo.toml for {:?}", crate_root_path)))?
            .to_string();

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
}
