use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use crate::error::AppError;
use crate::results::CompilationResult;
use crate::traits::Compiler;
use crate::config::CompilerConfig;
use crate::state_manager::State;
use crate::hasher::calculate_file_hash;

pub trait CompilationOrchestrator {
    fn compile_crate_with_cache(
        &self,
        crate_root_path: &Path,
        compiler: &dyn Compiler,
        config: &CompilerConfig,
        crate_name_to_root_map: &HashMap<String, PathBuf>,
        compiled_artifacts_map: &mut HashMap<String, PathBuf>,
        state_arc: &Arc<Mutex<State>>,
        dry_run: bool,
    ) -> Result<CompilationResult, AppError>;
}

pub struct DefaultCompilationOrchestrator;

impl CompilationOrchestrator for DefaultCompilationOrchestrator {
    fn compile_crate_with_cache(
        &self,
        crate_root_path: &Path,
        compiler: &dyn Compiler,
        config: &CompilerConfig,
        crate_name_to_root_map: &HashMap<String, PathBuf>,
        compiled_artifacts_map: &mut HashMap<String, PathBuf>,
        state_arc: &Arc<Mutex<State>>,
        dry_run: bool,
    ) -> Result<CompilationResult, AppError> {
        let file_hash = calculate_file_hash(&crate_root_path.join("Cargo.toml"))?;

        // Check cache
        {
            let current_state = state_arc.lock().unwrap();
            if let Some(cached_result) = current_state.cache.get(crate_root_path) {
                if cached_result.source_checksum == file_hash {
                    println!("Cache hit for {:?}. Skipping compilation.", crate_root_path.display());
                    
                    // Add cached artifact to compiled_artifacts_map
                    if let Some(ref rlib_path_str) = cached_result.compiled_checksum {
                        let crate_name = crate_root_path.file_name().unwrap().to_string_lossy().to_string();
                        compiled_artifacts_map.insert(crate_name, PathBuf::from(rlib_path_str));
                    }
                    
                    return Ok(cached_result.clone());
                }
            }
        }

        if dry_run {
            println!("Dry run: Skipping compilation for {:?}", crate_root_path.display());
            return Ok(create_dry_run_result(crate_root_path));
        }

        let compilation_result = compiler.compile_crate(
            crate_root_path, 
            config, 
            crate_name_to_root_map, 
            compiled_artifacts_map
        )?;

        // Update compiled_artifacts_map on successful compilation
        if compilation_result.exit_code == 0 {
            if let Some(ref rlib_path_str) = compilation_result.compiled_checksum {
                let crate_name = crate_root_path.file_name().unwrap().to_string_lossy().to_string();
                compiled_artifacts_map.insert(crate_name, PathBuf::from(rlib_path_str));
            } else {
                println!("Warning: No compiled .rlib path found in compilation result for {:?}", 
                        crate_root_path.display());
            }
        }

        // Update cache
        {
            let mut current_state = state_arc.lock().unwrap();
            current_state.cache.insert(crate_root_path.to_path_buf(), compilation_result.clone());
        }

        Ok(compilation_result)
    }
}

fn create_dry_run_result(crate_root_path: &Path) -> CompilationResult {
    CompilationResult {
        success: true,
        output: "Dry run - no actual compilation".to_string(),
        error: None,
        file_path: crate_root_path.to_path_buf(),
        rustc_version: "dry-run".to_string(),
        rustc_flags: vec![],
        stdout: "Dry run - no output".to_string(),
        stderr: String::new(),
        exit_code: 0,
        duration_ms: 0,
        source_checksum: String::new(),
        compiled_checksum: None,
        timestamp: chrono::Utc::now(),
    }
}

pub fn handle_compilation_result(
    compilation_result: &CompilationResult,
    crate_root_path: &Path,
) -> Result<(), AppError> {
    let exit_code = compilation_result.exit_code;
    
    if exit_code != 0 {
        println!("Compilation failed for {:?}. (State update skipped)", crate_root_path.display());
        println!("--- STDOUT ---");
        println!("{}", compilation_result.stdout);
        println!("--- STDERR ---");
        println!("{}", compilation_result.stderr);
        return Err(AppError::Custom(format!("Compilation failed for {:?}", crate_root_path.display())));
    } else {
        println!("Compilation successful for {:?}. (State update skipped)", crate_root_path.display());
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_dry_run_result() {
        let path = PathBuf::from("/test/path");
        let result = create_dry_run_result(&path);
        
        assert!(result.success);
        assert_eq!(result.exit_code, 0);
        assert_eq!(result.file_path, path);
        assert_eq!(result.rustc_version, "dry-run");
    }
}
