use anyhow::Result;
use clap::Parser;

mod cli;
use crate::cli::args::{Cli, Commands};
use crate::cli::run_commands::{run_add_submodules_command, run_submodule_status_command, run_generate_nix_command, run_generate_patches_command, run_analyze_command};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::AddSubmodules(ref args) => run_add_submodules_command(args, &cli),
        Commands::SubmoduleStatus(ref args) => run_submodule_status_command(args, &cli),
        Commands::GenerateNix(ref args) => run_generate_nix_command(args, &cli),
        Commands::GeneratePatches(ref args) => run_generate_patches_command(args, &cli),
        Commands::Analyze(ref args) => run_analyze_command(args, &cli),
    }
}