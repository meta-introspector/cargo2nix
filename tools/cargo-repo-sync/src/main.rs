use anyhow::Result;
use clap::Parser;

mod cli;
mod cargo_config_generator;
use crate::cli::args::{Cli, Commands};
use crate::cli::commands::{
    run_add_submodules_command,
    run_submodule_status_command,
    run_generate_nix_command,
    run_generate_patches_command,
    run_analyze_command,
    run_update_cargo_toml_command,
    run_process_tt_txt_command, // Add the new command
    run_collect_repo_state_command, // Add the new command
};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::AddSubmodules(ref args) => run_add_submodules_command(args, &cli),
        Commands::SubmoduleStatus(ref args) => run_submodule_status_command(args, &cli),
        Commands::GenerateNix(ref args) => run_generate_nix_command(args, &cli),
        Commands::GeneratePatches(ref args) => run_generate_patches_command(args, &cli),
        Commands::Analyze(ref args) => run_analyze_command(args, &cli),
        Commands::UpdateCargoToml(ref args) => run_update_cargo_toml_command(args, &cli),
        Commands::ProcessTtTxt(ref args) => run_process_tt_txt_command(args, &cli), // Add the new command
        Commands::CollectRepoState(ref args) => run_collect_repo_state_command(args, &cli), // Add the new command
    }
}