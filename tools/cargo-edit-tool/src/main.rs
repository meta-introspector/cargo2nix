extern crate anyhow;
extern crate clap;
extern crate git_wrapper_lib;
extern crate nix_generator_lib;
extern crate syn_adapter_lib;
extern crate cargo_edit_lib;
extern crate cargo_toml_editor_lib;
extern crate cargo_submodule_tool_lib;
extern crate regex;
extern crate toml_edit;
extern crate walkdir;
extern crate serde;
extern crate cargo_metadata;
extern crate git2;

extern crate lazy_static;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum, Args};
use std::path::{Path, PathBuf};

mod adapters_factory;
use adapters_factory::Mode;


mod cargo_metadata_provider;
mod cargo_toml_updater;
mod generate_workspaces;
mod update_cargo_config;
mod workspace_deps_generator;
mod cargo_edit_adapter_impl;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generates .cargo/config.toml patches for submodules
    GenerateConfig {
        /// Path to the project root (where the main Cargo.toml is)
        #[arg(long, default_value = ".")]
        project_root: PathBuf,

        /// Path to the output .cargo/config.toml
        #[arg(long, default_value = ".cargo/config.toml")]
        output_config: PathBuf,

        /// Operation mode for adapters (DryRun, Shell, Lib)
        #[arg(long, value_enum, default_value_t = AdapterMode::Lib)]
        mode: AdapterMode,
    },
    /// Generates Nix expressions for workspace members
    GenerateNix {
        /// Path to the project root (where the main Cargo.toml is)
        #[arg(long, default_value = ".")]
        project_root: PathBuf,

        /// Path to the output directory for Nix expressions
        #[arg(long, default_value = "./nix_generated")]
        output_path: PathBuf,

        /// Operation mode for adapters (DryRun, Shell, Lib)
        #[arg(long, value_enum, default_value_t = AdapterMode::Lib)]
        mode: AdapterMode,
    },
    // Add other subcommands here as needed
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum AdapterMode {
    DryRun,
    Shell,
    Lib,
}

impl From<AdapterMode> for Mode {
    fn from(adapter_mode: AdapterMode) -> Self {
        match adapter_mode {
            AdapterMode::DryRun => Mode::DryRun,
            AdapterMode::Shell => Mode::Shell,
            AdapterMode::Lib => Mode::Lib,
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::GenerateConfig { project_root, output_config, mode } => {
            println!("Running GenerateConfig command...");
            println!("Project Root: {:?}", project_root);
            println!("Output Config: {:?}", output_config);
            println!("Mode: {:?}", mode);

            let (git_adapter, cargo_metadata_provider, _nix_adapter, _syn_adapter, cargo_edit_adapter) = adapters_factory::get_adapters((*mode).into())?;

            // Ensure the .cargo directory exists
            let cargo_dir = output_config.parent().unwrap_or_else(|| Path::new("."));
            std::fs::create_dir_all(cargo_dir)
                .map_err(|e| anyhow::anyhow!("Failed to create directory {:?}: {}", cargo_dir, e))?;

            let generated_config_content = cargo_edit_adapter.generate_cargo_config(
                git_adapter.as_ref(),
                cargo_metadata_provider.as_ref(),
            )
            .context("Failed to generate cargo config using CargoEditAdapter")?;

            std::fs::write(output_config, generated_config_content)
                .context(format!("Failed to write updated config.toml: {:?}", output_config))?;

            println!("Successfully generated .cargo/config.toml.");
        },
        Commands::GenerateNix { project_root, output_path, mode } => {
            println!("Running GenerateNix command...");
            println!("Project Root: {:?}", project_root);
            println!("Output Path: {:?}", output_path);
            println!("Mode: {:?}", mode);

            let (git_adapter, cargo_metadata_provider, nix_adapter, _syn_adapter, cargo_edit_adapter) = adapters_factory::get_adapters((*mode).into())?;
            let workspace_info_provider = cargo_edit_lib::CargoConfigGeneratorImpl; // Instantiate the implementation

            nix_generator_lib::cli::commands::generate_nix::generate_nix(
                project_root,
                output_path,
                git_adapter.as_ref(),
                cargo_metadata_provider.as_ref(),
                nix_adapter.as_ref(),
                cargo_edit_adapter.as_ref(), // Pass cargo_edit_adapter
                &workspace_info_provider, // Pass the new workspace_info_provider
            )
            .context("Failed to generate Nix expressions")?;

            println!("Successfully generated Nix expressions.");
        }
    }

    Ok(())
}