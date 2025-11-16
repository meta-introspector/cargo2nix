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
}

#[derive(Parser, Debug)]
pub struct AddSubmodulesArgs {
    /// The root directory to start scanning for Cargo.toml files.
    #[arg(long, default_value = ".")]
    pub root_dir: PathBuf,

    /// The GitHub organization to fork repositories to.
    #[arg(long, default_value = "meta-introspector")]
    pub target_org: String,

    /// The branch to checkout and use for dependencies.
    #[arg(long, default_value = "feature/CRQ-016-nixify")]
    pub target_branch: String,

    /// Optional: Write the JSON plan to this file instead of stdout.
    #[arg(long)]
    pub output_file: Option<PathBuf>,

    /// Optional: Read the actions plan from a JSON file instead of discovering repositories.
    #[arg(long)]
    pub json_input_file: Option<PathBuf>,
}

#[derive(Parser, Debug)]
pub struct SubmoduleStatusArgs {
    /// The root directory to start scanning for Cargo.toml files.
    #[arg(long, default_value = ".")]
    pub root_dir: PathBuf,

    /// Optional: Read the actions plan from a JSON file instead of discovering repositories.
    #[arg(long)]
    pub json_input_file: Option<PathBuf>,
}

#[derive(Parser, Debug)]
pub struct GenerateNixArgs {
    /// The root directory to start scanning for Cargo.toml files.
    #[arg(long, default_value = ".")]
    pub root_dir: PathBuf,
}