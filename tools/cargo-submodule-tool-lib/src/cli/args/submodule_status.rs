use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct SubmoduleStatusArgs {
    /// The root directory of the project.
    #[arg(long, default_value = ".")]
    pub root_dir: PathBuf,
}
