#![forbid(unsafe_code)]

use std::{
    path::PathBuf,
};

use clap::{CommandFactory, Parser, ValueHint};
use clap_complete::{Shell};

use cargo2nix::{generate_cargo_nix, print_completions, write_to_file, write_to_stdout};

#[derive(Parser, Debug, PartialEq)]
#[command(about, author, long_about = None, version = env!("CARGO_PKG_VERSION"))]
/// Granular caching, development shell, Nix & Rust integration
struct Opt {
    /// Optional workspace directory (default value: ./)
    #[arg(value_name = "WORKSPACE_DIRECTORY", value_hint = ValueHint::DirPath)]
    workspace_directory: Option<PathBuf>,
    /// Generate a SHELL completion script
    #[arg(long = "completions", value_enum)]
    generator: Option<Shell>,
    /// Output to filepath (default value: ./Cargo.nix)
    #[arg(conflicts_with = "stdout", long, short, value_name = "FILEPATH", value_hint = ValueHint::FilePath)]
    file: Option<PathBuf>,
    /// Overwrite existing output filepath without prompting
    #[arg(action, long, short)]
    overwrite: bool,
    /// Don't attempt to update the lockfile
    #[arg(action, long, short, value_name = "LOCKED")]
    locked: bool,
    /// Output to stdout
    #[arg(conflicts_with = "file", action, long, short)]
    stdout: bool,
}

fn main() -> std::io::Result<()> {
    let opt = Opt::parse();

    if let Some(generator) = opt.generator {
        let mut cmd = Opt::command();
        eprintln!("Generating completion file for {:?}...", generator);
        print_completions(generator, &mut cmd);
        std::process::exit(0);
    }

    let workspace_directory = opt
        .workspace_directory
        .unwrap_or(std::env::current_dir()?)
        .canonicalize()?;
    let file = opt.file.unwrap_or(PathBuf::from("./Cargo.nix"));

    let rendered = generate_cargo_nix(&workspace_directory, opt.locked)
        .expect("Error generating nix expressions");

    if opt.stdout {
        write_to_stdout(&rendered).expect("Error writing to stdout");
    } else {
        write_to_file(&file, &rendered, &opt.overwrite).expect("Error writing to file");
    }

    Ok(())
}
