use anyhow::{Context, Result, anyhow};
use rocksdb::DB;
use serde_json;
use std::path::{Path, PathBuf};
// walkdir::WalkDir is no longer needed for topological sort
use sha2::{Digest, Sha256};
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher}; // NEW: For graph and topological sort

use crate::analysis_types::{IngestionChunk, IngestionFileDescriptor, ProjectFileAnalysis};
use crate::file_ingestion::{self, ingest_single_file};
use crate::hasher::calculate_content_id; // NEW: To ingest files as needed

const CHUNK_TARGET_SIZE_BYTES: usize = 4 * 1024; // 4KB

// Helper function to resolve use/mod paths

fn resolve_rust_path(base_path: &Path, import_path: &str) -> Option<PathBuf> {
    // Simple heuristic: try to resolve as relative path first

    let mut resolved_path = base_path.to_path_buf();

    // Handle "crate::" or "super::" etc. for more advanced resolution

    // For now, assume simple relative or absolute paths

    resolved_path.push(import_path);

    if resolved_path.exists() {
        return Some(resolved_path);
    }

    None
}

// Function to generate ingestion plan based on dependency graph (topological sort)

// Helper function to get ProjectFileAnalysis from DB, or ingest if not found
fn get_project_file_analysis(
    db: &DB,
    file_path_str: &str,
    project_root: &Path,
) -> Result<ProjectFileAnalysis> {
    // Determine the expected file type. For simplicity, assume "rust" for now.
    // A more robust solution would infer this from file extension or analyze content.
    let file_type = "rust";

    // First, try to get the ProjectFileAnalysis directly if we have enough information
    let direct_analysis_key_prefix = format!("file_analysis:{}:{}:", file_type, file_path_str);
    if let Some(content_hash_bytes) =
        db.get(format!("git_tree_entry:{}", file_path_str).as_bytes())?
    {
        let content_hash = String::from_utf8_lossy(&content_hash_bytes).to_string();
        let exact_analysis_key = format!("{}{}", direct_analysis_key_prefix, content_hash);
        if let Some(analysis_bytes) = db.get(exact_analysis_key.as_bytes())? {
            eprintln!("Retrieved analysis for {} from RocksDB.", file_path_str);
            return Ok(serde_json::from_slice(&analysis_bytes)?);
        }
    }

    // If not found (either no content hash in git_tree_entry, or no full analysis), ingest the single file
    eprintln!(
        "File analysis not found for {} in RocksDB, ingesting single file...",
        file_path_str
    );
    let absolute_path = project_root.join(file_path_str);
    ingest_single_file(db, &absolute_path, project_root)
}

pub fn generate_ingestion_plan(db: &DB, project_root: &Path, seed_path_str: &str) -> Result<()> {
    eprintln!("Generating ingestion plan with seed: {}", seed_path_str);

    let seed_path_buf = project_root.join(seed_path_str);

    let normalized_seed_path = seed_path_buf
        .canonicalize()
        .context(format!(
            "Failed to canonicalize seed path: {}",
            seed_path_str
        ))?
        .to_string_lossy()
        .to_string();

    let mut graph: HashMap<String, HashSet<String>> = HashMap::new(); // adj list: file -> files it depends on

    let mut in_degrees: HashMap<String, usize> = HashMap::new();

    let mut q: VecDeque<String> = VecDeque::new(); // Files to visit for dependency discovery

    let mut visited: HashSet<String> = HashSet::new(); // Files for which dependencies have been explored

    // Start with the seed file

    q.push_back(normalized_seed_path.clone());

    in_degrees.insert(normalized_seed_path.clone(), 0);

    let mut all_files_in_graph: HashSet<String> = HashSet::new();

    all_files_in_graph.insert(normalized_seed_path.clone());

    while let Some(current_file_path_str) = q.pop_front() {
        if !visited.insert(current_file_path_str.clone()) {
            continue; // Already processed dependencies for this file
        }

        eprintln!("Exploring dependencies for: {}", current_file_path_str);

        // Get ProjectFileAnalysis for the current file

        let analysis = get_project_file_analysis(db, &current_file_path_str, project_root)?;

        // Extract and process dependencies (uses and mods)

        let mut direct_dependencies = HashSet::new();

        if let Some(uses) = analysis.uses {
            for use_stmt in uses {
                // Parse use_stmt to extract actual path

                // This is a very simplified parser, needs to be robust for real Rust code

                // Example: `use crate::foo::bar;` -> relative path to foo/bar.rs

                // For now, let's assume direct path-like uses or external crates that we ignore for graph traversal

                if use_stmt.contains("::") {
                    // Simple heuristic for now

                    // Try to resolve as a file in the project_root

                    let parts: Vec<&str> = use_stmt.split("::").collect();

                    let potential_path = parts.join("/"); // basic conversion

                    if let Some(resolved) = resolve_rust_path(&project_root, &potential_path) {
                        direct_dependencies.insert(resolved.to_string_lossy().to_string());
                    } else if let Some(resolved) = resolve_rust_path(
                        &PathBuf::from(&current_file_path_str)
                            .parent()
                            .unwrap_or(Path::new("")),
                        &potential_path,
                    ) {
                        direct_dependencies.insert(resolved.to_string_lossy().to_string());
                    }
                }
            }
        }

        if let Some(mods) = analysis.mods {
            for mod_name in mods {
                let mut mod_path = PathBuf::from(current_file_path_str.clone());

                mod_path.set_extension(""); // Remove .rs

                mod_path.push(mod_name.clone()); // Add module name as folder

                mod_path.set_extension("rs"); // Try mod.rs

                if mod_path.exists() {
                    direct_dependencies
                        .insert(mod_path.canonicalize()?.to_string_lossy().to_string());
                } else {
                    mod_path.pop(); // Remove mod_name.rs

                    mod_path.push(mod_name.clone()); // Add module name as file (e.g., mod_name.rs in same dir)

                    mod_path.set_extension("rs");

                    if mod_path.exists() {
                        direct_dependencies
                            .insert(mod_path.canonicalize()?.to_string_lossy().to_string());
                    }
                }
            }
        }

        for dep_path_str in direct_dependencies {
            let normalized_dep_path = PathBuf::from(&dep_path_str)
                .canonicalize()
                .context(format!(
                    "Failed to canonicalize dependency path: {}",
                    dep_path_str
                ))?
                .to_string_lossy()
                .to_string();

            graph
                .entry(current_file_path_str.clone())
                .or_default()
                .insert(normalized_dep_path.clone());

            *in_degrees.entry(normalized_dep_path.clone()).or_insert(0) += 1;

            all_files_in_graph.insert(normalized_dep_path.clone());

            if !visited.contains(&normalized_dep_path) && !q.contains(&normalized_dep_path) {
                q.push_back(normalized_dep_path.clone());
            }
        }
    }

    // Initialize topological sort queue with nodes having in-degree 0

    let mut topo_q: VecDeque<String> = VecDeque::new();

    for file_path in &all_files_in_graph {
        if *in_degrees.get(file_path).unwrap_or(&0) == 0 {
            topo_q.push_back(file_path.clone());
        }
    }

    let mut sorted_files: Vec<String> = Vec::new();

    let mut num_nodes_processed = 0;

    while let Some(u) = topo_q.pop_front() {
        sorted_files.push(u.clone());

        num_nodes_processed += 1;

        // Correct Kahn's algorithm: decrement in-degrees of neighbors of u

        if let Some(neighbors) = graph.get(&u) {
            for v in neighbors {
                *in_degrees.entry(v.clone()).or_insert(0) -= 1; // Get mutable entry for decrement

                if *in_degrees.get(v).unwrap() == 0 {
                    topo_q.push_back(v.clone());
                }
            }
        }
    }

    if num_nodes_processed != all_files_in_graph.len() {
        eprintln!("Warning: Cycle detected in dependency graph!");

        // Handle cycle (e.g., break or report error)
    }

    eprintln!("\n--- Topological Sort Result ---");

    for file in &sorted_files {
        eprintln!("{}", file);
    }

    eprintln!("Total files in topological order: {}", sorted_files.len());

    // --- Chunking Logic (similar to before, but now uses sorted_files) ---

    eprintln!("Chunking sorted files into blocks...");

    let mut chunks: Vec<IngestionChunk> = Vec::new();

    let mut current_chunk_files: Vec<IngestionFileDescriptor> = Vec::new();

    let mut current_chunk_size_bytes: usize = 0;

    for file_path_str in &sorted_files {
        // Retrieve ProjectFileAnalysis for the file to create IngestionFileDescriptor

        let analysis = get_project_file_analysis(db, file_path_str, project_root)?; // Use the helper

        let descriptor = IngestionFileDescriptor {
            file_path: analysis.file_path,

            content_hash: analysis.content_hash,

            file_type: analysis.file_type,
        };

        let descriptor_serialized_len = serde_json::to_string(&descriptor)
            .unwrap_or_else(|_| "".to_string())
            .len();

        if current_chunk_size_bytes + descriptor_serialized_len > CHUNK_TARGET_SIZE_BYTES
            && !current_chunk_files.is_empty()
        {
            let chunk_id =
                calculate_content_id(serde_json::to_string(&current_chunk_files)?.as_bytes());

            chunks.push(IngestionChunk {
                chunk_id: chunk_id.clone(),

                files: current_chunk_files.clone(),

                estimated_size_bytes: current_chunk_size_bytes,
            });

            let db_key = format!("ingestion_plan:{}", chunk_id);

            db.put(
                db_key.as_bytes(),
                serde_json::to_string(&chunks.last().unwrap())?.as_bytes(),
            )
            .context(format!(
                "Failed to write ingestion plan chunk {} to RocksDB",
                chunk_id
            ))?;

            current_chunk_files.clear();

            current_chunk_size_bytes = 0;
        }

        current_chunk_files.push(descriptor);

        current_chunk_size_bytes += descriptor_serialized_len;
    }

    if !current_chunk_files.is_empty() {
        let chunk_id =
            calculate_content_id(serde_json::to_string(&current_chunk_files)?.as_bytes());

        chunks.push(IngestionChunk {
            chunk_id: chunk_id.clone(),

            files: current_chunk_files.clone(),

            estimated_size_bytes: current_chunk_size_bytes,
        });

        let db_key = format!("ingestion_plan:{}", chunk_id);

        db.put(
            db_key.as_bytes(),
            serde_json::to_string(&chunks.last().unwrap())?.as_bytes(),
        )
        .context(format!(
            "Failed to write ingestion plan chunk {} to RocksDB",
            chunk_id
        ))?;
    }

    eprintln!("\n--- Ingestion Plan Summary ---");

    eprintln!("Total files in topological order: {}", sorted_files.len());

    eprintln!("Total chunks generated: {}", chunks.len());

    if chunks.len() > 0 {
        let avg_files_per_chunk = sorted_files.len() as f64 / chunks.len() as f64;

        eprintln!("Average files per chunk: {:.2}", avg_files_per_chunk);
    }

    Ok(())
}
