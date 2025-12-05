use anyhow::Result;
use clap::Parser;

mod cli;

use crate::cli::commands::add_submodules::run_add_submodules_command;
use crate::cli::commands::analyze::run_analyze_command;
use crate::cli::commands::collect_repo_state::run_collect_repo_state_command;
use cargo_submodule_tool_lib::cli::args::add_submodules::AddSubmodulesArgs;
use cargo_submodule_tool_lib::cli::args::analyze::AnalyzeArgs;
use cargo_submodule_tool_lib::cli::args::collect_repo_state::CollectRepoStateArgs;
use cargo_submodule_tool_lib::cli::args::generate_patches::GeneratePatchesArgs;
use cargo_submodule_tool_lib::cli::args::process_tt_txt::ProcessTtTxtArgs;
use cargo_submodule_tool_lib::cli::args::submodule_status::SubmoduleStatusArgs;
use cargo_submodule_tool_lib::cli::args::Cli;
use cargo_submodule_tool_lib::cli::args::Commands; // Re-added
                                                   //use crate::cli::commands::generate_nix::run_generate_nix_command;
use crate::cli::commands::generate_patches::run_generate_patches_command;
//use crate::cli::commands::generate_workspaces::run_generate_workspaces_command;
use crate::cli::commands::process_tt_txt::run_process_tt_txt_command;
use crate::cli::commands::submodule_status::run_submodule_status_command;
//use crate::cli::commands::update_cargo_toml::run_update_cargo_toml_command;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::AddSubmodules(ref args)) => run_add_submodules_command(args, &cli),
        Some(Commands::SubmoduleStatus(ref args)) => run_submodule_status_command(args),
        //Some(Commands::GenerateNix(ref args)) => run_generate_nix_command(args, &cli),
        Some(Commands::GeneratePatches(ref args)) => run_generate_patches_command(args, &cli),
        Some(Commands::Analyze(ref args)) => run_analyze_command(args, &cli),
        //Some(Commands::UpdateCargoToml(ref args)) => run_update_cargo_toml_command(args, &cli),
        //Some(Commands::GenerateWorkspaces(ref args)) => run_generate_workspaces_command(args, &cli),
        Some(Commands::ProcessTtTxt(ref args)) => run_process_tt_txt_command(args, &cli),
        Some(Commands::CollectRepoState(ref args)) => {
            run_collect_repo_state_command(args.project_root.clone())
        }
        None => {
            // Handle the case where no subcommand is provided.
            // This usually means printing help or a default action.
            println!("No command provided. Use --help for more information.");
            Ok(())
        }
    }
}
