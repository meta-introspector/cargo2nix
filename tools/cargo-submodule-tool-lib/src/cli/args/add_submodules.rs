#[cfg(feature = "clap_enabled")]
use clap::Args as ClapArgs;
use std::path::PathBuf;

#[cfg(feature = "clap_enabled")]
#[derive(ClapArgs, Debug)]
pub struct AddSubmodulesArgs {
    /// The root directory of the project.
    #[arg(long, default_value = ".")]
    pub root_dir: PathBuf,

    /// The target organization for submodules.
    #[arg(long)]
    pub target_org: String,

    /// The target branch for submodules.
    #[arg(long)]
    pub target_branch: String,

    /// The output file for the generated submodule list.
    #[arg(long, default_value = "submodules.txt")]
    pub output_file: PathBuf,

    /// Path to a JSON input file for submodules.
    #[arg(long)]
    pub json_input_file: Option<PathBuf>,
}

#[cfg(not(feature = "clap_enabled"))]
#[derive(Debug)]
pub struct AddSubmodulesArgs {
    pub root_dir: PathBuf,
    pub target_org: String,
    pub target_branch: String,
    pub output_file: PathBuf,
    pub json_input_file: Option<PathBuf>,
}
