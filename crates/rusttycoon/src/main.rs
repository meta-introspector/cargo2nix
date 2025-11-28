use clap::Parser;
use std::path::PathBuf;
use anyhow::{Result, Context}; // Add anyhow for error handling
mod factory; // Declare the new factory module

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to the main Rust program to analyze and build the tycoon factory around.
    #[clap(long)]
    pub main_program_path: PathBuf,
}

fn main() -> Result<()> { // Change main to return Result
    let cli = Cli::parse();
    println!("Rust Tycoon starting with main program: {:?}", cli.main_program_path);

    let db_path = PathBuf::from("./mcp_db_tycoon"); // Dedicated DB for tycoon
    let factory = factory::Factory::new(&db_path)?; // Initialize factory
    
    // First factory block: Ingest the project
    let project_root = cli.main_program_path.parent()
        .context("main_program_path must have a parent directory")?;
    factory.ingest_project(project_root)?;

    // TODO: Implement the lazy loading and payment mechanism here.

    Ok(())
}
