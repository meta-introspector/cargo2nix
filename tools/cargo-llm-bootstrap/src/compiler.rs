use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Instant;

use sha2::{Digest, Sha256};
use crate::error::AppError;
use crate::results::CompilationResult;
use crate::traits::Compiler;
use crate::config::CompilerConfig;

mod compiler_args;
mod compilation_output;

pub struct RustcCompilerImpl;

impl Compiler for RustcCompilerImpl {
    fn compile_crate(&self, crate_root_path: &Path, config: &CompilerConfig, crate_name_to_root_map: &HashMap<String, PathBuf>, compiled_artifacts_map: &HashMap<String, PathBuf>) -> Result<CompilationResult, AppError> {
        let lib_rs_path = crate_root_path.join("src/lib.rs");
        let main_rs_path = crate_root_path.join("src/main.rs");

        let target_source_file = if lib_rs_path.exists() {
            lib_rs_path
        } else if main_rs_path.exists() {
            main_rs_path
        } else {
            return Err(AppError::Custom(format!("No src/lib.rs or src/main.rs found for crate {:?}", crate_root_path)));
        };

        let file_content = std::fs::read(&target_source_file).map_err(AppError::Io)?;
        let mut hasher = Sha256::new();
        hasher.update(&file_content);
        let source_checksum = format!("{:x}", hasher.finalize());

        let crate_name = crate_root_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown_crate")
            .to_string();

        let start_time = Instant::now();
        let mut command = compiler_args::build_rustc_command(
            crate_root_path,
            config,
            &crate_name,
            crate_name_to_root_map,
            compiled_artifacts_map,
        )?;

        println!("Running rustc command: {:?}", command);

        let command_output = command.output().map_err(AppError::Io)?;
        let duration = start_time.elapsed();

        compilation_output::process_rustc_output(
            command_output,
            crate_root_path,
            config,
            &crate_name,
            source_checksum,
            duration,
        )
    }
}
