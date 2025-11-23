use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;
use toml;

use crate::traits::ConfigHandler;
use crate::error::AppError;
use crate::rustc_options::RustcOptions; // Import RustcOptions

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct CompilerConfig {
    #[serde(rename = "rust-src-path")]
    pub rust_src_path: Option<PathBuf>,
    #[serde(rename = "output-dir")]
    pub output_dir: Option<PathBuf>,
    #[serde(rename = "target-triple")]
    pub target_triple: Option<String>,
    #[serde(rename = "rustc-path")]
    pub rustc_path: Option<PathBuf>,
    #[serde(rename = "cargo-path")]
    pub cargo_path: Option<PathBuf>,
    #[serde(rename = "build-dir")]
    pub build_dir: Option<PathBuf>,
    pub rustc_options: RustcOptions, // Add RustcOptions field
    // Add other relevant fields from AppConfig as needed, but keep it minimal initially
}

impl CompilerConfig {
    pub fn new() -> Self {
        Default::default()
    }
}

impl ConfigHandler<CompilerConfig> for CompilerConfig {
    fn load_config(&self, path: &Path) -> Result<CompilerConfig, AppError> {
        let content = fs::read_to_string(path)?;
        let config: CompilerConfig = toml::from_str(&content)?;
        Ok(config)
    }

    fn merge_configs(&mut self, mut base: CompilerConfig, overlay: CompilerConfig) -> CompilerConfig {
        if let Some(rust_src_path) = overlay.rust_src_path {
            base.rust_src_path = Some(rust_src_path);
        }
        if let Some(output_dir) = overlay.output_dir {
            base.output_dir = Some(output_dir);
        }
        if let Some(target_triple) = overlay.target_triple {
            base.target_triple = Some(target_triple);
        }
        if let Some(rustc_path) = overlay.rustc_path {
            base.rustc_path = Some(rustc_path);
        }
        if let Some(cargo_path) = overlay.cargo_path {
            base.cargo_path = Some(cargo_path);
        }
        if let Some(build_dir) = overlay.build_dir {
            base.build_dir = Some(build_dir);
        }
        // Merge rustc_options
        base.rustc_options.unpretty_expanded = overlay.rustc_options.unpretty_expanded;
        base
    }
}
