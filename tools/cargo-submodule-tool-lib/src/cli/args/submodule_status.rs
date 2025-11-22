#[cfg(feature = "clap_enabled")]
use clap::Parser;
use std::path::PathBuf;

#[cfg(feature = "clap_enabled")]
#[derive(Parser, Debug)]
pub struct SubmoduleStatusArgs {
    /// The root directory of the project.
    #[arg(long, default_value = ".")]
    pub root_dir: PathBuf,
}

#[cfg(not(feature = "clap_enabled"))]
#[derive(Debug)]
pub struct SubmoduleStatusArgs {
    pub root_dir: PathBuf,
}