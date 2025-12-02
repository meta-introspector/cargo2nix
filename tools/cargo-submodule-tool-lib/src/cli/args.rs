use anyhow::{anyhow, Result};
#[cfg(feature = "clap_enabled")]
use clap::error::{Error, ErrorKind};
#[cfg(feature = "clap_enabled")]
use clap::{Args as ClapArgs, Command, FromArgMatches, Parser, Subcommand};
use std::path::PathBuf;
pub mod add_submodules;
pub mod submodule_status;
//pub mod generate_nix;
pub mod analyze;
pub mod collect_repo_state;
pub mod generate_patches;
pub mod process_tt_txt;
pub mod update_cargo_toml;
//pub mod generate_workspaces;

#[cfg(feature = "clap_enabled")]
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(long, global = true)]
    pub verbose: bool,

    #[arg(long, global = true)]
    pub quiet: bool,

    #[arg(long, global = true)]
    pub color: bool,

    #[arg(long, global = true)]
    pub dry_run: bool,

    /// Path to a JSON log file for detailed output.
    #[arg(long, global = true)]
    pub json_log_file: Option<PathBuf>,

    /// Generate a detailed report.
    #[arg(long, global = true)]
    pub report: bool,

    /// Use pure Rust Git implementation.
    #[arg(long, global = true)]
    pub pure_rust_git: bool,
}

#[cfg(feature = "clap_enabled")]
#[derive(Subcommand)]
pub enum Commands {
    AddSubmodules(#[clap(flatten)] add_submodules::AddSubmodulesArgs),
    SubmoduleStatus(#[clap(flatten)] submodule_status::SubmoduleStatusArgs),
    //GenerateNix(generate_nix::GenerateNixArgs),
    GeneratePatches(#[clap(flatten)] generate_patches::GeneratePatchesArgs),
    Analyze(#[clap(flatten)] analyze::AnalyzeArgs),
    //UpdateCargoToml(update_cargo_toml::UpdateCargoTomlArgs),
    ProcessTtTxt(#[clap(flatten)] process_tt_txt::ProcessTtTxtArgs),
    CollectRepoState(#[clap(flatten)] collect_repo_state::CollectRepoStateArgs),
    //GenerateWorkspaces(generate_workspaces::GenerateWorkspacesArgs),
}

#[cfg(not(feature = "clap_enabled"))]
#[derive(Debug)]
pub struct Cli {
    pub command: Option<Commands>,
    pub verbose: bool,
    pub quiet: bool,
    pub color: bool,
    pub dry_run: bool,
}

#[cfg(not(feature = "clap_enabled"))]
impl Cli {
    pub fn parse() -> Self {
        Cli {
            command: None,
            verbose: false,
            quiet: false,
            color: false,
            dry_run: false,
        }
    }
}

#[cfg(not(feature = "clap_enabled"))]
#[derive(Debug)]
pub enum Commands {}
