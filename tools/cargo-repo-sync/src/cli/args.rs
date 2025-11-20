use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Perform a dry run without making actual changes.
    #[arg(long, global = true)]
    pub dry_run: bool,

    /// Capture all executed commands and their output to a JSON file.
    #[arg(long, global = true)]
    pub json_log_file: Option<PathBuf>,

    /// Report detailed information about executed commands to stdout.
    #[arg(long, global = true)]
    pub report: bool,

    /// Use the pure Rust Git implementation instead of shelling out to the system 'git' command.
    #[arg(long, global = true)]
    pub pure_rust_git: bool,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Adds submodules based on discovered or provided plan
    AddSubmodules(AddSubmodulesArgs),
    /// Reports status of submodules
    SubmoduleStatus(SubmoduleStatusArgs),
    /// Generates Cargo.nix files for all discovered Cargo.toml/Cargo.lock pairs
    GenerateNix(GenerateNixArgs),
    /// Generates .cargo/config.toml patch entries for workspace submodules
    GeneratePatches(GeneratePatchesArgs),
    /// Analyzes dependency graph, non-vendored modules, and generates config patches
    Analyze(AnalyzeArgs),
    /// Updates the [workspace.dependencies] section of a Cargo.toml file
    UpdateCargoToml(UpdateCargoTomlArgs),
    /// Processes the tt.txt file to generate workspace dependencies
    ProcessTtTxt(ProcessTtTxtArgs),
    /// Collects and displays the current state of the repository (Git, Cargo, Nix)
    CollectRepoState(CollectRepoStateArgs),
    /// Generates a lattice of workspaces based on dependency analysis
    GenerateWorkspaces(GenerateWorkspacesArgs),
}

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
}

#[derive(Parser, Debug)]
pub struct GenerateNixArgs {
    /// The root directory to start scanning for Cargo.toml files.
    #[arg(long, default_value = ".")]
    pub root_dir: PathBuf,
}

#[derive(Parser, Debug)]
pub struct GeneratePatchesArgs {
    /// The root directory to start scanning for Cargo.toml files.
    #[arg(long, default_value = ".")]
    pub root_dir: PathBuf,
}

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

#[derive(Parser, Debug)]
pub struct UpdateCargoTomlArgs {
    /// Path to the Cargo.toml file to be updated.
    #[arg(long, default_value = "Cargo.toml")]
    pub cargo_toml_path: PathBuf,

    /// The root directory of the project.
    #[arg(long, default_value = ".")]
    pub project_root: PathBuf,
}

#[derive(Parser, Debug)]
pub struct ProcessTtTxtArgs {
    /// Path to the tt.txt file to be processed.
    #[arg(long, default_value = "tt.txt")]
    pub tt_txt_path: PathBuf,

    /// Path to the submodules directory.
    #[arg(long, default_value = "submodules")]
    pub submodules_dir: PathBuf,
}

#[derive(Parser, Debug)]
pub struct CollectRepoStateArgs {
    /// The root directory of the project.
    #[arg(long, default_value = ".")]
    pub project_root: PathBuf,
}

#[derive(Parser, Debug)]
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

#[derive(Parser, Debug)]
pub struct SubmoduleStatusArgs {
    /// The root directory of the project.
    #[arg(long, default_value = ".")]
    pub root_dir: PathBuf,
}