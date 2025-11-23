use crate::config::CompilerConfig;
use crate::traits::{CodeGenerator, RustParser, ConfigHandler};
use crate::syn_quote_impl::SynQuoteProcessor;
use clap::Parser;
use std::fs;
use std::path::PathBuf;

mod config;
mod traits;
mod syn_quote_impl;
mod error;
use error::AppError;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to a TOML configuration file
    #[arg(long, env = "CONFIG_FILE")]
    config_file: Option<PathBuf>,

    /// Path to the Rust source directory
    #[arg(long, env = "RUST_SRC_PATH")]
    rust_src_path: Option<PathBuf>,

    /// Directory for compiler output
    #[arg(long, env = "OUTPUT_DIR")]
    output_dir: Option<PathBuf>,

    /// Target triple for compilation (e.g., x86_64-unknown-linux-gnu)
    #[arg(long, env = "TARGET_TRIPLE")]
    target_triple: Option<String>,

    /// Path to the rustc executable
    #[arg(long, env = "RUSTC_PATH")]
    rustc_path: Option<PathBuf>,

    /// Path to the cargo executable
    #[arg(long, env = "CARGO_PATH")]
    cargo_path: Option<PathBuf>,

    /// Directory for build artifacts
    #[arg(long, env = "BUILD_DIR")]
    build_dir: Option<PathBuf>,

    /// Optional Rust file to process
    file: Option<PathBuf>,
}

fn main() -> Result<(), AppError> {
    let args = Args::parse();

    let mut final_config = CompilerConfig::new();
    let mut config_handler_instance = CompilerConfig::new(); // Create an instance to call merge_configs on

    if let Some(config_file_path) = &args.config_file {
        let file_config = config_handler_instance.load_config(config_file_path)?;
        final_config = config_handler_instance.merge_configs(final_config, file_config);
    }

    let cli_config = CompilerConfig {
        rust_src_path: args.rust_src_path,
        output_dir: args.output_dir,
        target_triple: args.target_triple,
        rustc_path: args.rustc_path,
        cargo_path: args.cargo_path,
        build_dir: args.build_dir,
    };

    final_config = config_handler_instance.merge_configs(final_config, cli_config);

    println!("Final Compiler Configuration: {:?}", final_config);

    let rust_code = if let Some(file_path) = args.file {
        fs::read_to_string(file_path).map_err(AppError::Io)?
    } else {
        // If no file is provided, use a dummy string
        r#"
            fn main() {
                println!("Hello, LLM Bootstrap!");
            }
        "#.to_string()
    };

    let processor = SynQuoteProcessor;

    let ast = processor.parse_str(&rust_code)?;

    // For now, just print the AST back as code.
    // Later, we'll introduce transformations here.
    let quoted_code = processor.generate_code(&ast)?;

    println!("{}", quoted_code);

    Ok(())
}