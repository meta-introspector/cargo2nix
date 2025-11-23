use std::path::Path;

mod error;
use crate::error::AppError;

/// Trait for compiling Rust files.
pub trait Compiler {
    fn compile_file(&self, file_path: &Path, rustc_path: &Path, config: &crate::config::CompilerConfig) -> Result<crate::results::CompilationResult, AppError>;
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


