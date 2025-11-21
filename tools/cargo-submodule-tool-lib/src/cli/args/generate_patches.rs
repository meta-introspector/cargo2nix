use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct GeneratePatchesArgs {
    /// The root directory to start scanning for Cargo.toml files.
    #[arg(long, default_value = ".")]
    pub root_dir: PathBuf,
}
