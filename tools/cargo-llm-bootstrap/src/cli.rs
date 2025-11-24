use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Path to a TOML configuration file
    #[arg(long, env = "CONFIG_FILE")]
    pub config_file: Option<PathBuf>,

    /// Path to the Rust source directory
    #[arg(long, env = "RUST_SRC_PATH")]
    pub rust_src_path: Option<PathBuf>,

    /// Directory for compiler output
    #[arg(long, env = "OUTPUT_DIR")]
    pub output_dir: Option<PathBuf>,

    /// Target triple for compilation (e.g., x86_64-unknown-linux-gnu)
    #[arg(long, env = "TARGET_TRIPLE")]
    pub target_triple: Option<String>,

    /// Path to the rustc executable
    #[arg(long, env = "RUSTC_PATH")]
    pub rustc_path: Option<PathBuf>,

    /// Path to the cargo executable
    #[arg(long, env = "CARGO_PATH")]
    pub cargo_path: Option<PathBuf>,

    /// Directory for build artifacts
    #[arg(long, env = "BUILD_DIR")]
    pub build_dir: Option<PathBuf>,

    /// Optional: Process only crates at this specific layer level
    #[clap(long)]
    pub level: Option<u32>,

    /// Optional: Limit the number of crates to process at the specified level
    #[clap(long)]
    pub limit: Option<u32>,

    /// Optional: Skip the graph generation steps (rust-src-scanner and graph-petal-generator)
    #[clap(long)]
    pub skip_graph_generation: bool,

    /// Optional: Perform a dry run without actual compilation
    #[clap(long)]
    pub dry_run: bool,
}

impl Args {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
