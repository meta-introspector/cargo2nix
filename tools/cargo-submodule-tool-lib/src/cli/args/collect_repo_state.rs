#[cfg(feature = "clap_enabled")]
use clap::Parser;
use std::path::PathBuf;

#[cfg(feature = "clap_enabled")]
#[derive(Parser, Debug)]
pub struct CollectRepoStateArgs {
    /// The root directory of the project.
    #[arg(long, default_value = ".")]
    pub project_root: PathBuf,
}

#[cfg(not(feature = "clap_enabled"))]
#[derive(Debug)]
pub struct CollectRepoStateArgs {
    pub project_root: PathBuf,
}
