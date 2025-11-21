use clap::error::{Error, ErrorKind};
use clap::{Parser, FromArgMatches, Subcommand, Args as ClapArgs, Command};
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

#[derive(Debug)]
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

impl FromArgMatches for Commands {
    fn from_arg_matches(matches: &clap::ArgMatches) -> Result<Self, Error> {
        match matches.subcommand() {
            Some(("add-submodules", args)) => Ok(Self::AddSubmodules(AddSubmodulesArgs::from_arg_matches(args)?)),
            Some(("submodule-status", args)) => Ok(Self::SubmoduleStatus(SubmoduleStatusArgs::from_arg_matches(args)?)),
            Some(("generate-nix", args)) => Ok(Self::GenerateNix(GenerateNixArgs::from_arg_matches(args)?)),
            Some(("generate-patches", args)) => Ok(Self::GeneratePatches(GeneratePatchesArgs::from_arg_matches(args)?)),
            Some(("analyze", args)) => Ok(Self::Analyze(AnalyzeArgs::from_arg_matches(args)?)),
            Some(("update-cargo-toml", args)) => Ok(Self::UpdateCargoToml(UpdateCargoTomlArgs::from_arg_matches(args)?)),
            Some(("process-tt-txt", args)) => Ok(Self::ProcessTtTxt(ProcessTtTxtArgs::from_arg_matches(args)?)),
            Some(("collect-repo-state", args)) => Ok(Self::CollectRepoState(CollectRepoStateArgs::from_arg_matches(args)?)),
            Some(("generate-workspaces", args)) => Ok(Self::GenerateWorkspaces(GenerateWorkspacesArgs::from_arg_matches(args)?)),
            _ => Err(Error::raw(ErrorKind::InvalidSubcommand, "Valid subcommand is `add-submodules` or `submodule-status`")),
        }
    }

    fn update_from_arg_matches(&mut self, matches: &clap::ArgMatches) -> Result<(), Error> {
        match matches.subcommand() {
            Some(("add-submodules", args)) => *self = Self::AddSubmodules(AddSubmodulesArgs::from_arg_matches(args)?),
            Some(("submodule-status", args)) => *self = Self::SubmoduleStatus(SubmoduleStatusArgs::from_arg_matches(args)?),
            Some(("generate-nix", args)) => *self = Self::GenerateNix(GenerateNixArgs::from_arg_matches(args)?),
            Some(("generate-patches", args)) => *self = Self::GeneratePatches(GeneratePatchesArgs::from_arg_matches(args)?),
            Some(("analyze", args)) => *self = Self::Analyze(AnalyzeArgs::from_arg_matches(args)?),
            Some(("update-cargo-toml", args)) => *self = Self::UpdateCargoToml(UpdateCargoTomlArgs::from_arg_matches(args)?),
            Some(("process-tt-txt", args)) => *self = Self::ProcessTtTxt(ProcessTtTxtArgs::from_arg_matches(args)?),
            Some(("collect-repo-state", args)) => *self = Self::CollectRepoState(CollectRepoStateArgs::from_arg_matches(args)?),
            Some(("generate-workspaces", args)) => *self = Self::GenerateWorkspaces(GenerateWorkspacesArgs::from_arg_matches(args)?),
            _ => (),
        }
        Ok(())
    }
}

impl Subcommand for Commands {
    fn augment_subcommands(cmd: Command) -> Command {
        cmd.subcommand(AddSubmodulesArgs::augment_args(Command::new("add-submodules")))
            .subcommand(SubmoduleStatusArgs::augment_args(Command::new("submodule-status")))
            .subcommand(GenerateNixArgs::augment_args(Command::new("generate-nix")))
            .subcommand(GeneratePatchesArgs::augment_args(Command::new("generate-patches")))
            .subcommand(AnalyzeArgs::augment_args(Command::new("analyze")))
            .subcommand(UpdateCargoTomlArgs::augment_args(Command::new("update-cargo-toml")))
            .subcommand(ProcessTtTxtArgs::augment_args(Command::new("process-tt-txt")))
            .subcommand(CollectRepoStateArgs::augment_args(Command::new("collect-repo-state")))
            .subcommand(GenerateWorkspacesArgs::augment_args(Command::new("generate-workspaces")))
            .subcommand_required(true)
    }

    fn augment_subcommands_for_update(cmd: Command) -> Command {
        cmd.subcommand(AddSubmodulesArgs::augment_args(Command::new("add-submodules")))
            .subcommand(SubmoduleStatusArgs::augment_args(Command::new("submodule-status")))
            .subcommand(GenerateNixArgs::augment_args(Command::new("generate-nix")))
            .subcommand(GeneratePatchesArgs::augment_args(Command::new("generate-patches")))
            .subcommand(AnalyzeArgs::augment_args(Command::new("analyze")))
            .subcommand(UpdateCargoTomlArgs::augment_args(Command::new("update-cargo-toml")))
            .subcommand(ProcessTtTxtArgs::augment_args(Command::new("process-tt-txt")))
            .subcommand(CollectRepoStateArgs::augment_args(Command::new("collect-repo-state")))
            .subcommand(GenerateWorkspacesArgs::augment_args(Command::new("generate-workspaces")))
            .subcommand_required(true)
    }

    fn has_subcommand(name: &str) -> bool {
        matches!(name, "add-submodules" | "submodule-status" | "generate-nix" | "generate-patches" | "analyze" | "update-cargo-toml" | "process-tt-txt" | "collect-repo-state" | "generate-workspaces")
    }
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

    /// Optional: Generate a workspace for a specific package and its inverse dependencies.
    #[arg(long)]
    pub package_name: Option<String>,
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

#[derive(Parser, Debug)]
pub struct SubmoduleStatusArgs {
    /// The root directory of the project.
    #[arg(long, default_value = ".")]
    pub root_dir: PathBuf,
}