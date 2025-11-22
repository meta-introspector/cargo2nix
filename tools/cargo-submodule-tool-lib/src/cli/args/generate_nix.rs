#[cfg(feature = "clap_enabled")]
use clap::Parser;
use std::path::PathBuf;

#[cfg(feature = "clap_enabled")]
#[derive(Parser, Debug)]
pub struct GenerateNixArgs {
    /// The root directory to start scanning for Cargo.toml files.
    #[arg(long, default_value = ".")]
    pub root_dir: PathBuf,
}

#[cfg(not(feature = "clap_enabled"))]
#[derive(Debug)]
pub struct GenerateNixArgs {
    pub root_dir: PathBuf,
}