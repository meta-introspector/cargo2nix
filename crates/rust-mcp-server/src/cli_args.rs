use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Optional: Path to a Rust file to analyze directly via CLI, bypassing LSP.
    #[clap(long)]
    pub file: Option<String>,
    /// Optional: Path to a dynamic library (plugin) to load and execute directly.
    #[clap(long)]
    pub plugin_path: Option<String>,
    /// Optional: Path to the plugin crate to rebuild before loading (e.g., crates/mcp-plugin-example).
    #[clap(long)]
    pub rebuild_plugin: Option<String>,
    /// Optional: Path to a project directory to scan, analyze Rust files, and ingest into RocksDB.
    #[clap(long)]
    pub project_path: Option<String>,
    /// Optional: Query RocksDB for project analysis data, counting unique and duplicate content hashes.
    #[clap(long)]
    pub query_project_analysis: bool,
    /// Optional: Retrieve the ProjectFileAnalysis for a specific file path from RocksDB.
    #[clap(long)]
    pub get_file_analysis: Option<String>,
    /// Optional: Initiate a bootstrap compilation process. Takes a compiler source path and a target source path.
    /// Example: --boot submodules/rust/compiler/rustc_driver_impl/src/lib.rs submodules/rust/compiler/rustc_driver_impl/src/lib.rs
    #[clap(long, num_args = 2, value_names = ["COMPILER_SOURCE_PATH", "TARGET_SOURCE_PATH"])]
    pub boot: Option<Vec<String>>,
    /// Optional: Generate an ingestion plan, chunking files into 4KB blocks of metadata for parallel processing.
    #[clap(long)]
    pub generate_ingestion_plan: bool,
}
