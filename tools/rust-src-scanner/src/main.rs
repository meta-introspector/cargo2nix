use std::collections::{HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};
use std::fs;

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use serde_json;
use walkdir::WalkDir;
use clap::Parser;
use sha2::{Digest, Sha256};
use petgraph::algo::toposort;

mod cargo_parser;
mod lmfdb_semantic_index; // New module for LMFDB and semantic indexing
mod error; // New module for error handling
mod declaration_parser; // New module for parsing Rust declarations
mod semantic_id; // New module for composite semantic IDs
use crate::error::AppError;

const CHUNK_SIZE_BYTES: usize = 4096; // Target chunk size

// FileStatus is no longer relevant for Cargo.toml scanning
// #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
// pub enum FileStatus {
//     Pending,
//     Completed, // Successfully compiled, result in output_dir
//     Failed,
//     Done,      // Successfully compiled, result moved to done_dir
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CargoEntry { // Renamed from FileEntry
    pub semantic_id: semantic_id::SemanticId, // Composite semantic ID for this Cargo.toml entry
    pub path: PathBuf,
    pub package_name: String,
    pub package_version: String,
    pub dependencies: Vec<String>, // List of direct dependencies
    // Add other Cargo.toml metadata as needed
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DirectoryEntry {
    pub semantic_id: semantic_id::SemanticId,
    pub path: PathBuf,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DependencyGraphData {
    pub nodes: Vec<CargoEntry>,
    pub edges: Vec<(String, String)>, // (source_package_name, target_package_name)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileIndex {
    pub files: HashMap<PathBuf, CargoEntry>, // Changed to CargoEntry
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainState {
    pub rust_src_path_hash: String,
    pub index_file_paths: Vec<PathBuf>,
    pub last_saved_timestamp: DateTime<Utc>,
    pub output_dir: PathBuf,
    pub done_dir: PathBuf,
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

    /// Limit the number of index chunks generated
    #[arg(long)]
    limit: Option<u32>,

    /// Optional path to save the dependency graph (in DOT format)
    #[arg(long)]
    graph_output_path: Option<PathBuf>,

    /// Optional: Name of the root crate to start dependency scanning from (e.g., "rustc")
    #[arg(long)]
    root_crate: Option<String>,

    /// Optional: Directory to store the Cargo.toml parsing cache
    #[arg(long)]
    cache_dir: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let rust_src_path = &args.rust_src_path;
    let output_dir = &args.output_dir;

    fs::create_dir_all(output_dir)?;

    // Initialize semantic index and eigenmatrix
    let mut semantic_index = lmfdb_semantic_index::SemanticIndex::new();
    #[cfg(not(feature = "disable-eigenvector"))]
    let mut eigen_matrix = lmfdb_semantic_index::EigenMatrix::new(0, 0); // Will be resized later

    // For assigning unique IDs to packages and building the dependency matrix
    let mut package_name_to_id: HashMap<String, usize> = HashMap::new();
    let mut id_counter: usize = 0;

    // Instantiate CargoParser
    let mut cargo_parser_instance = cargo_parser::CargoParser::new(args.cache_dir);

    println!("Scanning for Cargo.toml files in: {:?}", rust_src_path.display());
    let mut all_cargo_entries: Vec<CargoEntry> = Vec::new();
    let mut all_declarations: Vec<declaration_parser::Declaration> = Vec::new(); // New vector for declarations
    let mut all_directory_entries: Vec<DirectoryEntry> = Vec::new(); // New vector for directory entries
    let mut count = 0;

    let mut cargo_toml_paths_to_process: VecDeque<(PathBuf, i32)> = VecDeque::new();
    let mut visited_cargo_toml_paths: HashSet<PathBuf> = HashSet::new();
    let mut package_name_to_cargo_toml_path: HashMap<String, PathBuf> = HashMap::new();

    // First, find all Cargo.toml files and map package names to their paths
    // This initial scan is necessary to resolve dependencies by package name later
    for entry in WalkDir::new(rust_src_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() && path.file_name().map_or(false, |name| name == "Cargo.toml") {
            // Temporarily parse to get package name without adding to graph/cache yet
            let content = fs::read_to_string(path).map_err(AppError::Io)?;
            let value: toml::Value = match toml::from_str(&content) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Warning: Failed to parse Cargo.toml file {}: {}", path.display(), e);
                    continue; // Skip this file
                }
            };
            if let Some(package_name) = value.get("package").and_then(|p| p.get("name")).and_then(|n| n.as_str()) {
                package_name_to_cargo_toml_path.insert(package_name.to_string(), path.to_path_buf());
            }
        }
    }

    if let Some(root_crate_name) = &args.root_crate {
        if let Some(root_cargo_toml_path) = package_name_to_cargo_toml_path.get(root_crate_name) {
            cargo_toml_paths_to_process.push_back((root_cargo_toml_path.clone(), 0));
            visited_cargo_toml_paths.insert(root_cargo_toml_path.clone());
            println!("Starting targeted scan from root crate: {} at {:?}", root_crate_name, root_cargo_toml_path.display());
        } else {
            eprintln!("Error: Root crate '{}' not found in the source directory.", root_crate_name);
            return Err("Root crate not found".into());
        }
    } else {
        // If no root crate is specified, fall back to scanning all Cargo.toml files found
        for (package_name, path) in package_name_to_cargo_toml_path.iter() {
            if !visited_cargo_toml_paths.contains(path) {
                cargo_toml_paths_to_process.push_back((path.clone(), 0)); // Assign level 0 to all top-level crates
                visited_cargo_toml_paths.insert(path.clone());
            }
        }
        println!("No root crate specified. Scanning all Cargo.toml files found.");
    }

    while let Some((cargo_toml_path, level)) = cargo_toml_paths_to_process.pop_front() {
        println!("Parsing: {:?} (Level: {})", cargo_toml_path.display(), level);
        let parsed_toml = match cargo_parser_instance.parse_cargo_toml(&cargo_toml_path) {
            Ok(toml) => toml,
            Err(e) => {
                eprintln!("Error parsing Cargo.toml file {}: {}", cargo_toml_path.display(), e);
                continue; // Skip this file if parsing fails
            }
        };

        let package_name = parsed_toml.get("package")
            .and_then(|pkg| pkg.get("name"))
            .and_then(|name| name.as_str())
            .unwrap_or("unknown_package")
            .to_string();
        let package_version = parsed_toml.get("package")
            .and_then(|pkg| pkg.get("version"))
            .and_then(|version| version.as_str())
            .unwrap_or("0.0.0")
            .to_string();

        // Assign a unique ID to the package
        let current_id = *package_name_to_id.entry(package_name.clone()).or_insert_with(|| {
            let id = id_counter;
            id_counter += 1;
            id
        });
        let mut package_semantic_id = semantic_id::SemanticId::new(current_id);
        package_semantic_id.depth = level.abs() as usize; // Use the assigned level as depth

        let mut dependencies = Vec::new();
        if let Some(deps_table) = parsed_toml.get("dependencies").and_then(|v| v.as_table()) {
            for (dep_name, _dep_value) in deps_table {
                dependencies.push(dep_name.clone());
                if let Some(dep_cargo_toml_path) = package_name_to_cargo_toml_path.get(dep_name) {
                    if !visited_cargo_toml_paths.contains(dep_cargo_toml_path) {
                        cargo_toml_paths_to_process.push_back((dep_cargo_toml_path.clone(), level - 1));
                        visited_cargo_toml_paths.insert(dep_cargo_toml_path.clone());
                    }
                }
            }
        }

        all_cargo_entries.push(CargoEntry {
            semantic_id: package_semantic_id.clone(),
            path: cargo_toml_path.to_path_buf(),
            package_name: package_name.clone(),
            package_version,
            dependencies,
        });

        let fiber_bundle = lmfdb_semantic_index::LMFDBFiberBundle::new(&package_name)
            .with_semantic_id(package_semantic_id); // Associate SemanticId with FiberBundle
        semantic_index.add_entry(&package_name, fiber_bundle);
        count += 1;
        if count % 100 == 0 { // Print progress every 100 Cargo.toml files
            println!("  Found and parsed {} Cargo.toml files...", count);
        }
    }
    println!("Finished scanning. Found and parsed {} Cargo.toml files.", count);
    println!("Found {} Rust declarations.", all_declarations.len());
    println!("Found {} directories.", all_directory_entries.len());

    println!("\n--- Semantic IDs for Cargo Entries ---");
    for entry in &all_cargo_entries {
        println!("Package: {}, Semantic ID: {}", entry.package_name, entry.semantic_id);
    }
    println!("------------------------------------");

    // Build crate dependency graph and perform topological sort
    println!("\n--- Building Crate Dependency Graph ---");
    let crate_graph = cargo_parser_instance.get_dependency_graph();

    // Perform topological sort
    match toposort(crate_graph, None) {
        Ok(sorted_nodes) => {
            println!("\n--- Topologically Sorted Crates (Build Order) ---");
            for node_idx in sorted_nodes.iter().rev() { // Reverse to get build order (dependencies first)
                println!("{}", crate_graph[*node_idx]);
            }
            println!("-------------------------------------------------");
        }
        Err(cycle) => {
            eprintln!("\nError: Cycle detected in crate dependencies. Topological sort not possible.");
            eprintln!("Node in cycle: {}", crate_graph[cycle.node_id()]);
        }
    }

    // Collect nodes and edges for JSON serialization
    let mut graph_nodes: Vec<CargoEntry> = Vec::new();
    let mut graph_edges: Vec<(String, String)> = Vec::new();

    // Populate graph_nodes with all_cargo_entries
    for entry in all_cargo_entries.iter() {
        graph_nodes.push(entry.clone());
    }

    // Populate graph_edges based on the dependencies listed in each CargoEntry
    let package_name_to_cargo_entry: HashMap<String, &CargoEntry> = all_cargo_entries.iter()
        .map(|entry| (entry.package_name.clone(), entry))
        .collect();

    for source_entry in all_cargo_entries.iter() {
        for dep_name in &source_entry.dependencies {
            if let Some(target_entry) = package_name_to_cargo_entry.get(dep_name) {
                graph_edges.push((source_entry.package_name.clone(), target_entry.package_name.clone()));
            } else {
                // This handles external dependencies or dependencies not found in our scanned set
                // For now, we'll just add them as edges to external nodes.
                // The graph-petal-generator will need to decide how to handle these.
                graph_edges.push((source_entry.package_name.clone(), dep_name.clone()));
            }
        }
    }

    // Save the dependency graph to a file if a path is provided
    if let Some(graph_output_path) = &args.graph_output_path {
        println!("\nSaving dependency graph to: {:?}", graph_output_path.display());

        let dependency_graph_data = DependencyGraphData {
            nodes: graph_nodes,
            edges: graph_edges,
        };

        let serialized_graph = serde_json::to_string_pretty(&dependency_graph_data)?;
        fs::write(graph_output_path, serialized_graph.as_bytes())?;
        println!("Dependency graph saved successfully in JSON format.");
    }

    // #[cfg(not(feature = "disable-eigenvector"))]
    // {
    //     // Resize eigen_matrix and populate it based on dependencies
    //     let num_packages = id_counter;
    //     eigen_matrix = lmfdb_semantic_index::EigenMatrix::new(num_packages, num_packages);

    //     for cargo_entry in &all_cargo_entries {
    //         let source_id = cargo_entry.semantic_id.unique_idx;
    //         for dep_name in &cargo_entry.dependencies {
    //             if let Some(&target_id) = package_name_to_id.get(dep_name) {
    //                 println!("DEBUG: source_id = {}, target_id = {}, num_packages = {}", source_id, target_id, num_packages);
    //                 std::io::stdout().flush().unwrap(); // Flush stdout
    //                 if source_id >= num_packages || target_id >= num_packages {
    //                     println!("ERROR: Index out of bounds before get/set: source_id={}, target_id={}, num_packages={}", source_id, target_id, num_packages);
    //                     std::io::stdout().flush().unwrap(); // Flush stdout
    //                 }
    //                 // Increment the count of dependency from source_id to target_id
    //                 // This assumes a simple count. More complex weighting can be added later.
    //                 if let Ok(current_value) = eigen_matrix.get(source_id, target_id) {
    //                     let _ = eigen_matrix.set(source_id, target_id, current_value + 1.0);
    //                 }
    //             }
    //         }
    //     }

    //     // Print the eigen_matrix (for debugging/verification)
    //     println!("\nDependency EigenMatrix:\n{}", eigen_matrix);
    // }

    // Calculate hash of the source directory
    let rust_src_path_hash = hash_directory(rust_src_path)?;

    // Chunk and save Cargo entries
    let mut index_file_paths: Vec<PathBuf> = Vec::new();
    let mut current_chunk_files: HashMap<PathBuf, CargoEntry> = HashMap::new(); // Changed to CargoEntry
    let mut chunk_index = 0;

    for cargo_entry in all_cargo_entries {
        current_chunk_files.insert(cargo_entry.path.clone(), cargo_entry);

        // Check if current chunk size exceeds target or if limit is reached
        let serialized_chunk = serde_json::to_string(&FileIndex { files: current_chunk_files.clone() })?;
        if serialized_chunk.len() >= CHUNK_SIZE_BYTES {
            let chunk_file_name = format!("index_{}.json", chunk_index);
            let chunk_file_path = output_dir.join(&chunk_file_name);
            fs::write(&chunk_file_path, serialized_chunk)?;
            index_file_paths.push(chunk_file_path);

            current_chunk_files.clear();
            chunk_index += 1;

            if let Some(limit) = args.limit {
                if chunk_index >= limit {
                    println!("Chunk limit ({}) reached. Stopping chunk generation.", limit);
                    break; // Stop processing further Cargo entries
                }
            }
        }
    }

    // Save any remaining files in the last chunk
    if !current_chunk_files.is_empty() {
        let chunk_file_name = format!("index_{}.json", chunk_index);
        let chunk_file_path = output_dir.join(&chunk_file_name);
        let serialized_chunk = serde_json::to_string(&FileIndex { files: current_chunk_files.clone() })?;
        fs::write(&chunk_file_path, serialized_chunk)?;
        index_file_paths.push(chunk_file_path);
    }

    // Save main state file
    let main_state = MainState {
        rust_src_path_hash,
        index_file_paths,
        last_saved_timestamp: Utc::now(),
        output_dir: output_dir.to_path_buf(),
        done_dir: output_dir.join("done_results"),
    };
    let main_state_path = output_dir.join("main_state.json");
    let serialized_main_state = serde_json::to_string_pretty(&main_state)?;
    fs::write(&main_state_path, serialized_main_state)?;

    println!("Successfully generated state and index chunks in: {:?}", output_dir.display());

    Ok(())
}

// Dummy hash_directory function for now, will be replaced by actual implementation
fn hash_directory(path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let mut hasher = Sha256::new();
    let mut entries: Vec<PathBuf> = Vec::new();

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        entries.push(entry.path().to_path_buf());
    }

    // Sort entries to ensure consistent hash regardless of file system order
    entries.sort();

    for entry_path in entries {
        let relative_path = match entry_path.strip_prefix(path) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Error stripping prefix for {}: {}", entry_path.display(), e);
                continue; // Skip this entry if prefix cannot be stripped
            }
        };
        hasher.update(relative_path.to_string_lossy().as_bytes());

        if entry_path.is_file() {
            // Hash file size and modification time
            let metadata = fs::metadata(&entry_path)?;
            hasher.update(metadata.len().to_string().as_bytes());
            hasher.update(metadata.modified()?.duration_since(std::time::UNIX_EPOCH)?.as_secs().to_string().as_bytes());
        }
    }

    Ok(format!("{:x}", hasher.finalize()))
}
