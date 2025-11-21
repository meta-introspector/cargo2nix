use clap::Parser;
use std::path::PathBuf;

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
