use std::path::Path;
use std::collections::HashMap;
use std::path::PathBuf;

mod error;
use crate::error::AppError;

/// Trait for compiling Rust files.
pub trait Compiler {
    fn compile_crate(&self, crate_root_path: &Path, config: &crate::config::CompilerConfig, crate_name_to_root_map: &HashMap<String, PathBuf>, compiled_artifacts_map: &HashMap<String, PathBuf>) -> Result<crate::results::CompilationResult, AppError>;
}

/// Trait for saving compilation results.
pub trait ResultStore {
    fn save_result(&self, result: &crate::results::CompilationResult, output_dir: &Path) -> Result<(), AppError>;
}



/// Trait for handling configuration loading and merging.
pub trait ConfigHandler<C>
where
    C: for<'de> serde::Deserialize<'de> + serde::Serialize + Default,
{
    fn load_config(&self, path: &Path) -> Result<C, AppError>;
    fn merge_configs(&mut self, base: C, overlay: C) -> C;
}


