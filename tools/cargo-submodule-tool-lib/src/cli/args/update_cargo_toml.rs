use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct UpdateCargoTomlArgs {
    /// Path to the Cargo.toml file to be updated.
    #[arg(long, default_value = "Cargo.toml")]
    pub cargo_toml_path: PathBuf,

    /// The root directory of the project.
    #[arg(long, default_value = ".")]
    pub project_root: PathBuf,
}
