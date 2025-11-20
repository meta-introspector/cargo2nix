use anyhow::Result;
use clap::Parser;

mod cli;
mod cargo_config_generator;
use crate::cli::args::{AddSubmodulesArgs, AnalyzeArgs, Cli, CollectRepoStateArgs, Commands, GenerateNixArgs, GeneratePatchesArgs, GenerateWorkspacesArgs, ProcessTtTxtArgs, SubmoduleStatusArgs, UpdateCargoTomlArgs};
use crate::cli::commands::add_submodules::run_add_submodules_command;
use crate::cli::commands::analyze::run_analyze_command;
use crate::cli::commands::collect_repo_state::run_collect_repo_state_command;
use crate::cli::commands::generate_nix::run_generate_nix_command;
use crate::cli::commands::generate_patches::run_generate_patches_command;
use crate::cli::commands::generate_workspaces::run_generate_workspaces_command;
use crate::cli::commands::process_tt_txt::run_process_tt_txt_command;
use crate::cli::commands::submodule_status::run_submodule_status_command;
use crate::cli::commands::update_cargo_toml::run_update_cargo_toml_command;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::AddSubmodules(ref args) => run_add_submodules_command(args, &cli),
        Commands::SubmoduleStatus(ref args) => run_submodule_status_command(args, &cli),
        Commands::GenerateNix(ref args) => run_generate_nix_command(args, &cli),
        Commands::GeneratePatches(ref args) => run_generate_patches_command(args, &cli),
        Commands::Analyze(ref args) => run_analyze_command(args, &cli),
        Commands::UpdateCargoToml(ref args) => run_update_cargo_toml_command(args, &cli),
        Commands::GenerateWorkspaces(ref args) => run_generate_workspaces_command(args, &cli),
        Commands::ProcessTtTxt(ref args) => run_process_tt_txt_command(args, &cli),
        Commands::CollectRepoState(ref args) => run_collect_repo_state_command(args, &cli),
    }
}