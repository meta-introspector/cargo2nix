use std::collections::BTreeMap;
use std::path::PathBuf;
use std::fs;

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use serde_json;
use clap::Parser;
use anyhow::Result;



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileMetadata {
    pub path: PathBuf,
    pub last_modified: DateTime<Utc>,
    pub hash: String,
    pub index: u64, // Sequential index for the file
    pub category: String, // Category of the file (e.g., "Nix", "Rust", "Cargo", "Doc")
    pub monster_godel_index: Option<u128>, // Gödel number for semantic hashing
    pub prime_exponents: BTreeMap<u32, u32>, // Exponents for each supersingular prime
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainState {
    pub rust_src_path_hash: String,
    pub index_file_paths: Vec<PathBuf>,
    pub last_saved_timestamp: DateTime<Utc>,
    pub output_dir: PathBuf,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Directory containing the chunk_*.json files and main_state.json from rust-src-scanner
    #[arg(long)]
    input_dir: PathBuf,

    /// Directory to save the new grouped chunks and updated main state file
    #[arg(long)]
    output_dir: PathBuf,

    /// Target size for the new grouped chunks in bytes (e.g., 4096 for 4KB)
    #[arg(long, default_value_t = 4096)]
    chunk_size_limit: usize,
}

fn main() -> Result<()> {
    let args = Args::parse();

    fs::create_dir_all(&args.output_dir)?;

    // Get the absolute path of the input directory
    let current_dir = std::env::current_dir()?;
    let absolute_input_dir = current_dir.join(&args.input_dir);
    println!("DEBUG: Absolute input directory: {:?}", absolute_input_dir);

    // 1. Read main_state.json to get list of chunk files
    let main_state_path = absolute_input_dir.join("main_state.json");
    println!("DEBUG: Main state path: {:?}", main_state_path);
    let main_state_content = fs::read_to_string(&main_state_path)?;
    let rust_scanner_main_state: MainState = serde_json::from_str(&main_state_content)?;

    // 2. Read all chunk_*.json files into a single Vec<FileMetadata>
    let mut all_file_metadata: Vec<FileMetadata> = Vec::new();
    for chunk_file_relative_path in rust_scanner_main_state.index_file_paths {
        // Resolve the relative chunk file path against the absolute input directory
        let chunk_file_name = chunk_file_relative_path.file_name().unwrap();
        let full_path = absolute_input_dir.join(chunk_file_name);
        println!("DEBUG: Attempting to read chunk file: {:?}", full_path);
        let chunk_content = fs::read_to_string(&full_path)?;
        let chunk_data: Vec<FileMetadata> = serde_json::from_str(&chunk_content)?;
        all_file_metadata.extend(chunk_data);
    }

    // 3. Sort FileMetadata by monster_godel_index
    all_file_metadata.sort_by_key(|fm| fm.monster_godel_index.unwrap_or(0));

    // 4. Group sorted FileMetadata into new chunks based on chunk_size_limit
    let mut new_main_state = MainState {
        rust_src_path_hash: rust_scanner_main_state.rust_src_path_hash,
        index_file_paths: Vec::new(),
        last_saved_timestamp: Utc::now(),
        output_dir: args.output_dir.clone(),
    };

    let mut current_chunk_files: Vec<FileMetadata> = Vec::new();
    let mut current_chunk_size_bytes: usize = 0;
    let mut chunk_index: usize = 0;

    for file_metadata in all_file_metadata {
        let estimated_size = serde_json::to_string_pretty(&vec![file_metadata.clone()])?.len(); // Estimate size of one item

        if current_chunk_size_bytes + estimated_size >= args.chunk_size_limit && !current_chunk_files.is_empty() {
            // Write current chunk to file
            let chunk_file_name = format!("grouped_chunk_{}.json", chunk_index);
            let chunk_file_path = args.output_dir.join(&chunk_file_name);
            fs::write(&chunk_file_path, serde_json::to_string_pretty(&current_chunk_files)?)?;
            new_main_state.index_file_paths.push(chunk_file_path.clone());
            println!("Wrote grouped chunk {} to {:?}", chunk_index, chunk_file_path.display());

            // Reset for next chunk
            current_chunk_files.clear();
            current_chunk_size_bytes = 0;
            chunk_index += 1;
        }

        current_chunk_files.push(file_metadata);
        current_chunk_size_bytes += estimated_size;
    }

    // Write any remaining files in the current chunk
    if !current_chunk_files.is_empty() {
        let chunk_file_name = format!("grouped_chunk_{}.json", chunk_index);
        let chunk_file_path = args.output_dir.join(&chunk_file_name);
        fs::write(&chunk_file_path, serde_json::to_string_pretty(&current_chunk_files)?)?;
        new_main_state.index_file_paths.push(chunk_file_path.clone());
        println!("Wrote final grouped chunk {} to {:?}", chunk_index, chunk_file_path.display());
    }

    // Save the new main state file
    let new_main_state_path = args.output_dir.join("main_state.json");
    fs::write(&new_main_state_path, serde_json::to_string_pretty(&new_main_state)?)?;
    println!("New main state saved to: {:?}", new_main_state_path.display());

    Ok(())
}