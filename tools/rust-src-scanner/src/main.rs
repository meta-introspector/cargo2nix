use std::collections::HashMap;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::fs;

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use serde_json;
use walkdir::WalkDir;
use clap::Parser;
use anyhow::Result; // Import anyhow::Result

const CHUNK_SIZE_BYTES: usize = 4096; // Target chunk size

// The 15 supersingular primes for Monster Group encoding
const SUPERSINGULAR_PRIMES: [u32; 15] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71];



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
pub struct FileIndex {
    pub files: HashMap<PathBuf, FileMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainState {
    pub rust_src_path_hash: String,
    pub index_file_paths: Vec<PathBuf>,
    pub last_saved_timestamp: DateTime<Utc>,
    pub output_dir: PathBuf,
}

// Function to calculate semantic exponents based on the file's sequential index
fn calculate_semantic_exponents(file_index: u64) -> BTreeMap<u32, u32> {
    let mut exponents = BTreeMap::new();

    // Convert file_index to its binary representation
    let mut temp_index = file_index;
    for &prime in SUPERSINGULAR_PRIMES.iter() {
        if temp_index == 0 {
            // If index is 0, all remaining exponents are 0
            exponents.insert(prime, 0);
        } else {
            // Map each bit of the index to an exponent (0 or 1)
            let exponent = (temp_index % 2) as u32;
            exponents.insert(prime, exponent);
            temp_index /= 2;
        }
    }

    exponents
}

// Function to generate the Monster Gödel Index
fn generate_monster_godel_index(exponents: &BTreeMap<u32, u32>) -> u128 {
    let mut godel_index: u128 = 1;
    for (&prime, &exponent) in exponents.iter() {
        godel_index *= (prime as u128).pow(exponent as u32);
    }
    godel_index
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the Rust source directory
    #[arg(long)]
    rust_src_path: PathBuf,

    /// Directory to save the index chunks and main state file
    #[arg(long)]
    output_dir: PathBuf,

    /// Optional: Path to the cache file
    #[arg(long)]
    cache_path: Option<PathBuf>,

    /// Optional: Limit the number of files to process
    #[arg(long)]
    limit: Option<usize>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let rust_src_path = &args.rust_src_path;
    let output_dir = &args.output_dir;
    let cache_path = args.cache_path.unwrap_or_else(|| output_dir.join("file_cache.json"));

    fs::create_dir_all(output_dir)?;

    // Initialize MainState
    let mut main_state = MainState {
        rust_src_path_hash: "0".to_string(), // Placeholder, will be updated later if needed
        index_file_paths: Vec::new(),
        last_saved_timestamp: Utc::now(),
        output_dir: output_dir.clone(),
    };

    // Load existing cache if it exists
    let mut file_cache: HashMap<PathBuf, FileMetadata> = if cache_path.exists() {
        let cache_content = fs::read_to_string(&cache_path)?;
        serde_json::from_str(&cache_content).unwrap_or_default()
    } else {
        HashMap::new()
    };

    println!("Scanning for files in: {:?}", rust_src_path.display());
    let mut updated_files = 0;
    let mut new_files = 0;
    let mut processed_files_count = 0;

    let mut current_chunk_files: Vec<FileMetadata> = Vec::new();
    let mut chunk_index: usize = 0;

    for entry in WalkDir::new(rust_src_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.file_name().to_string_lossy() == "Cargo.toml") // Only Cargo.toml files
        .filter(|e| !e.path().to_string_lossy().contains("/tests/")) // Exclude test directories
    {
        if let Some(max_limit) = args.limit {
            if processed_files_count >= max_limit {
                println!("Reached the specified limit of {} files. Stopping scan.", max_limit);
                break;
            }
        }

        let path = entry.path().to_path_buf();
        let metadata = fs::metadata(&path)?;
        let last_modified: DateTime<Utc> = metadata.modified()?.into();

        // Determine file category based on extension
        let category = match path.extension().and_then(|s| s.to_str()) {
            Some("nix") => "Nix".to_string(),
            Some("rs") => "Rust".to_string(),
            Some("toml") => "Cargo".to_string(),
            Some("sh") => "Script".to_string(),
            Some("md") => "Doc".to_string(),
            _ => "Other".to_string(),
        };

        // Create file_metadata for every processed file
        let mut file_metadata = FileMetadata {
            path: path.clone(),
            last_modified,
            hash: "0".to_string(),
            index: processed_files_count as u64, // Assign sequential index
            category,
            monster_godel_index: None,
            prime_exponents: BTreeMap::new(),
        };

        // Calculate semantic exponents based on sequential index
        let mut exponents = calculate_semantic_exponents(file_metadata.index);

        // Define the specific path for the rustc compiler crate
        let rustc_compiler_crate_path = PathBuf::from("/data/data/com.termux.nix/files/home/nix/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src/compiler/rustc/Cargo.toml");

        // Semantic assignment: If this is the rustc compiler crate, ensure prime 71 has an exponent of 1
        if file_metadata.path == rustc_compiler_crate_path {
            exponents.insert(71, 1);
        }

        // Generate the Monster Gödel Index with potentially modified exponents
        let godel_index = generate_monster_godel_index(&exponents);

        file_metadata.prime_exponents = exponents;
        file_metadata.monster_godel_index = Some(godel_index);

        // Chunking logic for all processed files
        if current_chunk_files.is_empty() {
            current_chunk_files.push(file_metadata.clone());
        } else {
            let mut temp_chunk = current_chunk_files.clone();
            temp_chunk.push(file_metadata.clone());
                            let estimated_chunk_size = serde_json::to_string_pretty(&temp_chunk)?.len();
            
                            if estimated_chunk_size >= CHUNK_SIZE_BYTES {                let chunk_file_name = format!("chunk_{}.json", chunk_index);
                let chunk_file_path = output_dir.join(&chunk_file_name);
                fs::write(&chunk_file_path, serde_json::to_string_pretty(&current_chunk_files)?)?;
                main_state.index_file_paths.push(chunk_file_path.clone());
                println!("Wrote chunk {} to {:?}", chunk_index, chunk_file_path.display());

                current_chunk_files.clear();
                current_chunk_files.push(file_metadata.clone());
                chunk_index += 1;
            } else {
                current_chunk_files.push(file_metadata.clone());
            }
        }

        // Update file_cache only if the file is new or updated
        let needs_update = if let Some(cached_metadata) = file_cache.get(&path) {
            cached_metadata.last_modified != last_modified
        } else {
            true
        };

        if needs_update {
            file_cache.insert(path.clone(), file_metadata.clone()); // Use the already created file_metadata
            if let Some(cached_metadata) = file_cache.get(&path) { // Check if it was an update or new
                if cached_metadata.last_modified != last_modified {
                    updated_files += 1;
                } else {
                    new_files += 1;
                }
            }
        }
        processed_files_count += 1;
    }

    println!("Finished scanning. Processed {} files. Found {} new files and {} updated files.", processed_files_count, new_files, updated_files);

    // Write any remaining files in the current chunk
    if !current_chunk_files.is_empty() {
        let chunk_file_name = format!("chunk_{}.json", chunk_index);
        let chunk_file_path = output_dir.join(&chunk_file_name);
        fs::write(&chunk_file_path, serde_json::to_string_pretty(&current_chunk_files)?)?;
        main_state.index_file_paths.push(chunk_file_path.clone());
        println!("Wrote final chunk {} to {:?}", chunk_index, chunk_file_path.display());
    }

    // Save the updated cache
    let serialized_cache = serde_json::to_string_pretty(&file_cache)?;
    fs::write(&cache_path, serialized_cache)?;
    println!("File cache saved to: {:?}", cache_path.display());

    // Save the main state file
    let main_state_path = output_dir.join("main_state.json");
    fs::write(&main_state_path, serde_json::to_string_pretty(&main_state)?)?;
    println!("Main state saved to: {:?}", main_state_path.display());

    Ok(())
}
