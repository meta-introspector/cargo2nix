#[cfg(feature = "clap_enabled")]
use clap::Parser;
use std::path::PathBuf;

#[cfg(feature = "clap_enabled")]
#[derive(Parser, Debug)]
pub struct UpdateCargoTomlArgs {
    /// Path to the Cargo.toml file to be updated.
    #[arg(long, default_value = "Cargo.toml")]
    pub cargo_toml_path: PathBuf,

    /// The root directory of the project.
    #[arg(long, default_value = ".")]
    pub project_root: PathBuf,
}

#[cfg(not(feature = "clap_enabled"))]
#[derive(Debug)]
pub struct UpdateCargoTomlArgs {
    pub cargo_toml_path: PathBuf,
    pub project_root: PathBuf,
}
