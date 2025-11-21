use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct CollectRepoStateArgs {
    /// The root directory of the project.
    #[arg(long, default_value = ".")]
    pub project_root: PathBuf,
}
