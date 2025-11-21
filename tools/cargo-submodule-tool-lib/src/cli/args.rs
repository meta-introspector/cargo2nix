use clap::error::{Error, ErrorKind};
use clap::{Parser, FromArgMatches, Subcommand, Args as ClapArgs, Command};
use std::path::PathBuf;

pub mod add_submodules;
pub mod submodule_status;
pub mod generate_nix;
pub mod generate_patches;
pub mod analyze;
pub mod update_cargo_toml;
pub mod process_tt_txt;
pub mod collect_repo_state;
pub mod generate_workspaces;

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
    AddSubmodules(add_submodules::AddSubmodulesArgs),
    /// Reports status of submodules
    SubmoduleStatus(submodule_status::SubmoduleStatusArgs),
    /// Generates Cargo.nix files for all discovered Cargo.toml/Cargo.lock pairs
    GenerateNix(generate_nix::GenerateNixArgs),
    /// Generates .cargo/config.toml patch entries for workspace submodules
    GeneratePatches(generate_patches::GeneratePatchesArgs),
    /// Analyzes dependency graph, non-vendored modules, and generates config patches
    Analyze(analyze::AnalyzeArgs),
    /// Updates the [workspace.dependencies] section of a Cargo.toml file
    UpdateCargoToml(update_cargo_toml::UpdateCargoTomlArgs),
    /// Processes the tt.txt file to generate workspace dependencies
    ProcessTtTxt(process_tt_txt::ProcessTtTxtArgs),
    /// Collects and displays the current state of the repository (Git, Cargo, Nix)
    CollectRepoState(collect_repo_state::CollectRepoStateArgs),
    /// Generates a lattice of workspaces based on dependency analysis
    GenerateWorkspaces(generate_workspaces::GenerateWorkspacesArgs),
}

impl FromArgMatches for Commands {
    fn from_arg_matches(matches: &clap::ArgMatches) -> Result<Self, Error> {
        match matches.subcommand() {
            Some(("add-submodules", args)) => Ok(Self::AddSubmodules(add_submodules::AddSubmodulesArgs::from_arg_matches(args)?)),
            Some(("submodule-status", args)) => Ok(Self::SubmoduleStatus(submodule_status::SubmoduleStatusArgs::from_arg_matches(args)?)),
            Some(("generate-nix", args)) => Ok(Self::GenerateNix(generate_nix::GenerateNixArgs::from_arg_matches(args)?)),
            Some(("generate-patches", args)) => Ok(Self::GeneratePatches(generate_patches::GeneratePatchesArgs::from_arg_matches(args)?)),
            Some(("analyze", args)) => Ok(Self::Analyze(analyze::AnalyzeArgs::from_arg_matches(args)?)),
            Some(("update-cargo-toml", args)) => Ok(Self::UpdateCargoToml(update_cargo_toml::UpdateCargoTomlArgs::from_arg_matches(args)?)),
            Some(("process-tt-txt", args)) => Ok(Self::ProcessTtTxt(process_tt_txt::ProcessTtTxtArgs::from_arg_matches(args)?)),
            Some(("collect-repo-state", args)) => Ok(Self::CollectRepoState(collect_repo_state::CollectRepoStateArgs::from_arg_matches(args)?)),
            Some(("generate-workspaces", args)) => Ok(Self::GenerateWorkspaces(generate_workspaces::GenerateWorkspacesArgs::from_arg_matches(args)?)),
            _ => Err(Error::raw(ErrorKind::InvalidSubcommand, "Valid subcommand is `add-submodules` or `submodule-status`")),
        }
    }

    fn update_from_arg_matches(&mut self, matches: &clap::ArgMatches) -> Result<(), Error> {
        match matches.subcommand() {
            Some(("add-submodules", args)) => *self = Self::AddSubmodules(add_submodules::AddSubmodulesArgs::from_arg_matches(args)?),
            Some(("submodule-status", args)) => *self = Self::SubmoduleStatus(submodule_status::SubmoduleStatusArgs::from_arg_matches(args)?),
            Some(("generate-nix", args)) => *self = Self::GenerateNix(generate_nix::GenerateNixArgs::from_arg_matches(args)?),
            Some(("generate-patches", args)) => *self = Self::GeneratePatches(generate_patches::GeneratePatchesArgs::from_arg_matches(args)?),
            Some(("analyze", args)) => *self = Self::Analyze(analyze::AnalyzeArgs::from_arg_matches(args)?),
            Some(("update-cargo-toml", args)) => *self = Self::UpdateCargoToml(update_cargo_toml::UpdateCargoTomlArgs::from_arg_matches(args)?),
            Some(("process-tt-txt", args)) => *self = Self::ProcessTtTxt(process_tt_txt::ProcessTtTxtArgs::from_arg_matches(args)?),
            Some(("collect-repo-state", args)) => *self = Self::CollectRepoState(collect_repo_state::CollectRepoStateArgs::from_arg_matches(args)?),
            Some(("generate-workspaces", args)) => *self = Self::GenerateWorkspaces(generate_workspaces::GenerateWorkspacesArgs::from_arg_matches(args)?),
            _ => (),
        }
        Ok(())
    }
}

impl Subcommand for Commands {
    fn augment_subcommands(cmd: Command) -> Command {
        cmd.subcommand(add_submodules::AddSubmodulesArgs::augment_args(Command::new("add-submodules")))
            .subcommand(submodule_status::SubmoduleStatusArgs::augment_args(Command::new("submodule-status")))
            .subcommand(generate_nix::GenerateNixArgs::augment_args(Command::new("generate-nix")))
            .subcommand(generate_patches::GeneratePatchesArgs::augment_args(Command::new("generate-patches")))
            .subcommand(analyze::AnalyzeArgs::augment_args(Command::new("analyze")))
            .subcommand(update_cargo_toml::UpdateCargoTomlArgs::augment_args(Command::new("update-cargo-toml")))
            .subcommand(process_tt_txt::ProcessTtTxtArgs::augment_args(Command::new("process-tt-txt")))
            .subcommand(collect_repo_state::CollectRepoStateArgs::augment_args(Command::new("collect-repo-state")))
            .subcommand(generate_workspaces::GenerateWorkspacesArgs::augment_args(Command::new("generate-workspaces")))
            .subcommand_required(true)
    }

    fn augment_subcommands_for_update(cmd: Command) -> Command {
        cmd.subcommand(add_submodules::AddSubmodulesArgs::augment_args(Command::new("add-submodules")))
            .subcommand(submodule_status::SubmoduleStatusArgs::augment_args(Command::new("submodule-status")))
            .subcommand(generate_nix::GenerateNixArgs::augment_args(Command::new("generate-nix")))
            .subcommand(generate_patches::GeneratePatchesArgs::augment_args(Command::new("generate-patches")))
            .subcommand(analyze::AnalyzeArgs::augment_args(Command::new("analyze")))
            .subcommand(update_cargo_toml::UpdateCargoTomlArgs::augment_args(Command::new("update-cargo-toml")))
            .subcommand(process_tt_txt::ProcessTtTxtArgs::augment_args(Command::new("process-tt-txt")))
            .subcommand(collect_repo_state::CollectRepoStateArgs::augment_args(Command::new("collect-repo-state")))
            .subcommand(generate_workspaces::GenerateWorkspacesArgs::augment_args(Command::new("generate-workspaces")))
            .subcommand_required(true)
    }

    fn has_subcommand(name: &str) -> bool {
        matches!(name, "add-submodules" | "submodule-status" | "generate-nix" | "generate-patches" | "analyze" | "update-cargo-toml" | "process-tt-txt" | "collect-repo-state" | "generate-workspaces")
    }
}
