use anyhow::{anyhow, Result};

#[cfg(feature = "clap_enabled")]
use clap::error::{Error, ErrorKind};
#[cfg(feature = "clap_enabled")]
use clap::{Args as ClapArgs, Command, FromArgMatches, Parser, Subcommand};

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
}

#[cfg(feature = "clap_enabled")]
#[derive(Subcommand)]
pub enum Commands {
    AddSubmodules(crate::cli::args::add_submodules::AddSubmodulesArgs),
    SubmoduleStatus(crate::cli::args::submodule_status::SubmoduleStatusArgs),
    GenerateNix(crate::cli::args::generate_nix::GenerateNixArgs),
    GeneratePatches(crate::cli::args::generate_patches::GeneratePatchesArgs),
    Analyze(crate::cli::args::analyze::AnalyzeArgs),
    UpdateCargoToml(crate::cli::args::update_cargo_toml::UpdateCargoTomlArgs),
    ProcessTtTxt(crate::cli::args::process_tt_txt::ProcessTtTxtArgs),
    CollectRepoState(crate::cli::args::collect_repo_state::CollectRepoStateArgs),
    GenerateWorkspaces(crate::cli::args::generate_workspaces::GenerateWorkspacesArgs),
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
