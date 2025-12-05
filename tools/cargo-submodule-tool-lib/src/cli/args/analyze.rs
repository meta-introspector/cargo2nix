#[cfg(feature = "clap_enabled")]
use clap::Parser;
use std::path::PathBuf;

#[cfg(feature = "clap_enabled")]
#[derive(Parser, Debug)]
pub struct AnalyzeArgs {
    /// Path to the depgraph.dot file.
    #[arg(long, default_value = "depgraph.dot")]
    pub depgraph_dot_file: PathBuf,

    /// Path to the tree.txt file.
    #[arg(long, default_value = "tree.txt")]
    pub tree_file: PathBuf,

    /// Path to the Cargo.lock file.
    #[arg(long, default_value = "Cargo.lock")]
    pub cargo_lock_file: PathBuf,

    /// Path to the .cargo/config.toml file.
    #[arg(long, default_value = ".cargo/config.toml")]
    pub cargo_config_file: PathBuf,

    /// Path to the submodules/members.txt file.
    #[arg(long, default_value = "submodules/members.txt")]
    pub members_file: PathBuf,

    /// Path to the submodules directory.
    #[arg(long, default_value = "submodules")]
    pub submodules_dir: PathBuf,

    /// The root directory of the project.
    #[arg(long, default_value = ".")]
    pub project_root: PathBuf,
}

#[cfg(not(feature = "clap_enabled"))]
#[derive(Debug)]
pub struct AnalyzeArgs {
    pub depgraph_dot_file: PathBuf,
    pub tree_file: PathBuf,
    pub cargo_lock_file: PathBuf,
    pub cargo_config_file: PathBuf,
    pub members_file: PathBuf,
    pub submodules_dir: PathBuf,
    pub project_root: PathBuf,
}
