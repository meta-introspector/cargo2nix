use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use crate::config::CompilerConfig;
use crate::error::AppError;

pub fn build_rustc_command(
    crate_root_path: &Path,
    config: &CompilerConfig,
    crate_name: &str,
    crate_name_to_root_map: &HashMap<String, PathBuf>,
    compiled_artifacts_map: &HashMap<String, PathBuf>,
) -> Result<Command, AppError> {
    let lib_rs_path = crate_root_path.join("src/lib.rs");
    let main_rs_path = crate_root_path.join("src/main.rs");

    let target_source_file = if lib_rs_path.exists() {
        lib_rs_path
    } else if main_rs_path.exists() {
        main_rs_path
    } else {
        return Err(AppError::Custom(format!("No src/lib.rs or src/main.rs found for crate {:?}", crate_root_path)));
    };

    let mut command = Command::new(config.rustc_path.as_ref().ok_or_else(|| AppError::Custom("rustc_path is not provided in config.".to_string()))?);

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

    // Add --extern flags for dependencies (simplified, assuming artifacts are in compiled_artifacts_map)
    for (dep_name, dep_rlib_path) in compiled_artifacts_map {
        // Only add --extern if the dependency is a direct dependency (or needed for linking)
        // This logic might need further refinement based on the exact dependency analysis of the LLM bootstrap
        command.arg("--extern").arg(format!("{}={}", dep_name, dep_rlib_path.display()));
    }

    Ok(command)
}
