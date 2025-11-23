#[cfg(feature = "clap_enabled")]
use clap::Parser;
use std::path::PathBuf;

#[cfg(feature = "clap_enabled")]
#[derive(Parser, Debug)]
pub struct GenerateWorkspacesArgs {
    /// The root directory of the project.
    #[arg(long, default_value = ".")]
    pub project_root: PathBuf,

    /// The output directory for the generated workspaces.
    #[arg(long, default_value = "generated_workspaces")]
    pub output_dir: PathBuf,

    /// Path to the depgraph.dot file.
    #[arg(long, default_value = "depgraph.dot")]
    pub depgraph_dot_file: PathBuf,

    /// Path to the tree.txt file.
    #[arg(long, default_value = "tree.txt")]
    pub tree_file: PathBuf,

    /// Optional: Generate a workspace for a specific package and its inverse dependencies.
    #[arg(long)]
    pub package_name: Option<String>,
}

#[cfg(not(feature = "clap_enabled"))]
#[derive(Debug)]
pub struct GenerateWorkspacesArgs {
    pub project_root: PathBuf,
    pub output_dir: PathBuf,
    pub depgraph_dot_file: PathBuf,
    pub tree_file: PathBuf,
    pub package_name: Option<String>,
}
