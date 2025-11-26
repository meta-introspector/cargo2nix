use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;

mod cargo_toml_adapter;
mod src_adapter;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the original crate (e.g., a submodule)
    #[clap(short, long, value_parser)]
    input_crate_path: PathBuf,

    /// Path to the output directory for the adapted crate
    #[clap(short, long, value_parser)]
    output_crate_path: PathBuf,

    /// Configuration file for feature adaptation (e.g., TOML or JSON)
    #[clap(short, long, value_parser)]
    config_path: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("Adapting crate from: {:?}", args.input_crate_path);
    println!("Outputting adapted crate to: {:?}", args.output_crate_path);
    if let Some(config_path) = args.config_path {
        println!("Using configuration from: {:?}", config_path);
    }

    // Adapt Cargo.toml
    cargo_toml_adapter::adapt_cargo_toml(&args.input_crate_path, &args.output_crate_path)
        .context("Failed to adapt Cargo.toml")?;

    // Adapt source code
    src_adapter::adapt_source_code(&args.input_crate_path, &args.output_crate_path)
        .context("Failed to adapt source code")?;

    println!("Crate adaptation completed successfully!");

    Ok(())
}
