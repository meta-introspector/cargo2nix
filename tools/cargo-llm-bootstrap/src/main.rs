use crate::config::CompilerConfig;
use crate::traits::{ConfigHandler, Compiler, ResultStore};
use clap::Parser;
use std::fs;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Instant;

mod config;
mod traits;
mod error;
mod compiler;
mod result_store;
mod results;
mod state_manager;
mod hasher;
use hasher::calculate_file_hash;
mod rustc_options;

use error::AppError;
use compiler::RustcCompilerImpl;
use result_store::JsonResultStore;
use crate::state_manager::{State, FileStatus};
use crate::rustc_options::RustcOptions;

const STATE_FILE_NAME: &str = "main_state.json";
const DONE_DIR_NAME: &str = "done_results";

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



    /// Optional: Process only crates at this specific layer level
    #[clap(long)]
    level: Option<u32>,

    /// Optional: Limit the number of crates to process at the specified level
    #[clap(long)]
    limit: Option<u32>,

    /// Optional: Skip the graph generation steps (rust-src-scanner and graph-petal-generator)
    #[clap(long)]
    skip_graph_generation: bool,

    /// Optional: Perform a dry run without actual compilation
    #[clap(long)]
    dry_run: bool,
}

fn main() -> Result<(), AppError> {
    let args = Args::parse();

    let mut final_config = CompilerConfig::new();
    let mut config_handler_instance = CompilerConfig::new();

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
        rustc_options: RustcOptions::new(),
    };

    final_config = config_handler_instance.merge_configs(final_config, cli_config);

    println!("Final Compiler Configuration: {:?}", final_config);

    let rust_src_path = final_config.rust_src_path.clone().ok_or_else(|| {
        AppError::Custom("RUST_SRC_PATH is not provided in config or CLI arguments.".to_string())
    })?;

    // Pre-scan for Cargo.toml files to build a map of crate_name -> crate_root_path
    let mut crate_name_to_root_map: HashMap<String, PathBuf> = HashMap::new();
    println!("Scanning for Cargo.toml files to build crate map...");
    for entry in walkdir::WalkDir::new(&rust_src_path) {
        let entry = entry.map_err(AppError::Walkdir)?;
        if entry.file_name() == "Cargo.toml" {
            let cargo_toml_path = entry.path();
            let content = fs::read_to_string(cargo_toml_path).map_err(AppError::Io)?;
            let cargo_toml: toml::Value = match content.parse() {
                Ok(value) => value,
                Err(e) => {
                    println!("Warning: Failed to parse Cargo.toml at {:?}: {}. Skipping this file.", cargo_toml_path, e);
                    continue; // Skip to the next file
                }
            };

            if let Some(package) = cargo_toml.get("package") {
                if let Some(name) = package.get("name").and_then(|n| n.as_str()) {
                    if let Some(parent) = cargo_toml_path.parent() {
                        crate_name_to_root_map.insert(name.to_string(), parent.to_path_buf());
                    }
                }
            }
        }
    }
    println!("Crate map built with {} entries.", crate_name_to_root_map.len());

    let output_dir = final_config.output_dir.clone().unwrap_or_else(|| PathBuf::from("compilation_results"));
    fs::create_dir_all(&output_dir).map_err(AppError::Io)?;

    let layered_graph_path = output_dir.join("layered_graph.json");
    let layered_crates: std::collections::HashMap<String, u32> = if layered_graph_path.exists() {
        let layered_graph_content = fs::read_to_string(&layered_graph_path)
            .map_err(|e| AppError::Custom(format!("Failed to read layered_graph.json: {}", e)))?;
        serde_json::from_str(&layered_graph_content)
            .map_err(|e| AppError::Custom(format!("Failed to parse layered_graph.json: {}", e)))?
    } else {
        println!("Warning: layered_graph.json not found at {:?}. Level filtering will not be applied.", layered_graph_path);
        std::collections::HashMap::new()
    };

    let done_dir = output_dir.join(DONE_DIR_NAME);
    fs::create_dir_all(&done_dir).map_err(AppError::Io)?;

    let rustc_path = final_config.rustc_path.clone().unwrap_or_else(|| PathBuf::from("rustc"));

    let main_state_file_path = output_dir.join(STATE_FILE_NAME);
    let main_state_file_path_clone = main_state_file_path.clone();
    let main_state_file_path_for_ctrlc = main_state_file_path.clone();

    // --- Initial State Loading ---
    let start_scan_time = Instant::now();
    let state = State::load(&main_state_file_path, output_dir.clone(), done_dir.clone())?;
    let scan_duration = start_scan_time.elapsed();
    println!("State loading took: {:?}", scan_duration);

    let state_arc = Arc::new(Mutex::new(state));
    let state_clone = Arc::clone(&state_arc);

    ctrlc::set_handler(move || {
        println!("\nCtrl+C detected. Saving state before exiting...");
        let state = state_clone.lock().unwrap();
        state.save(&main_state_file_path_for_ctrlc).expect("Failed to save state on exit");
        std::process::exit(0);
    }).map_err(|e| AppError::Custom(format!("Error setting Ctrl-C handler: {}", e)))?;

    let compiler = RustcCompilerImpl;
    let result_store = JsonResultStore;

    let mut compiled_artifacts_map: HashMap<String, PathBuf> = HashMap::new();

    let mut crates_to_process_initial: Vec<PathBuf> = Vec::new();
    let mut processed_crate_roots: HashSet<PathBuf> = HashSet::new();

    // Determine the maximum layer for iteration
    let max_layer = layered_crates.values().max().cloned().unwrap_or(0);
    println!("Max layer found: {}", max_layer);

    let mut total_crates_processed_overall = 0;

    for current_layer in 0..=max_layer {
        println!("\n--- Processing Layer {} ---", current_layer);

        let mut crates_in_current_layer: Vec<PathBuf> = Vec::new();
        let mut processed_crate_roots_in_layer: HashSet<PathBuf> = HashSet::new();

        for (crate_name, &layer) in &layered_crates {
            if layer == current_layer {
                if let Some(crate_root_path) = crate_name_to_root_map.get(crate_name) {
                    if processed_crate_roots_in_layer.insert(crate_root_path.clone()) {
                        crates_in_current_layer.push(crate_root_path.clone());
                    }
                } else {
                    println!("Warning: Could not find crate root path in map for crate: {}", crate_name);
                }
            }
        }

        // Sort crates within the current layer (already sorted by layer, but good for consistency)
        crates_in_current_layer.sort_by(|a_path, b_path| {
            let a_name = a_path.file_name().unwrap().to_string_lossy().to_string();
            let b_name = b_path.file_name().unwrap().to_string_lossy().to_string();

            let a_layer = layered_crates.get(&a_name).unwrap_or(&0);
            let b_layer = layered_crates.get(&b_name).unwrap_or(&0);

            a_layer.cmp(b_layer)
        });

        let total_crates_in_layer = crates_in_current_layer.len();
        if total_crates_in_layer == 0 {
            println!("No crates to process in Layer {}.", current_layer);
            continue;
        }

        // Apply limit if specified for the current layer
        let limited_crates_in_layer = if let Some(limit) = args.limit {
            crates_in_current_layer.into_iter().take(limit as usize).collect::<Vec<_>>()
        } else {
            crates_in_current_layer
        };

        let mut crates_processed_in_layer = 0;
        for crate_root_path in limited_crates_in_layer {
            crates_processed_in_layer += 1;
            total_crates_processed_overall += 1;
            let progress_percent = (crates_processed_in_layer as f64 / total_crates_in_layer as f64) * 100.0;
            println!("Compiling crate {} of {} in Layer {} ({:.2}%): {:?}", crates_processed_in_layer, total_crates_in_layer, current_layer, progress_percent, crate_root_path.display());

            let file_hash = calculate_file_hash(&crate_root_path.join("Cargo.toml"))?; // Hash Cargo.toml for crate

            // Check cache
            let mut current_state = state_arc.lock().unwrap();
            if let Some(cached_result) = current_state.cache.get(&crate_root_path) {
                if cached_result.source_checksum == file_hash {
                    println!("Cache hit for {:?}. Skipping compilation.", crate_root_path.display());
                    // Use cached result
                    // For now, we'll assume cached results are valid and add their compiled_checksum to compiled_artifacts_map
                    if let Some(ref rlib_path_str) = cached_result.compiled_checksum {
                        let crate_name = crate_root_path.file_name().unwrap().to_string_lossy().to_string();
                        compiled_artifacts_map.insert(crate_name, PathBuf::from(rlib_path_str));
                    }
                    // State update logic (re-enabled later)
                    // if should_update_state {
                    //     if cached_result.exit_code == Some(0) {
                    //         current_state.update_file_status(&crate_root_path, FileStatus::Done)?;
                    //     } else {
                    //         current_state.update_file_status(&crate_root_path, FileStatus::Failed)?;
                    //     }
                    //     current_state.save(&main_state_file_path_clone)?;
                    // }
                    drop(current_state); // Release lock before continuing
                    continue;
                }
            }
            drop(current_state); // Release lock before compilation

            if args.dry_run {
                println!("Dry run: Skipping compilation for {:?}", crate_root_path.display());
                continue;
            }

            let compilation_result = compiler.compile_crate(&crate_root_path, &final_config, &crate_name_to_root_map, &compiled_artifacts_map)?;
            let exit_code = compilation_result.exit_code;

            // After successful compilation, add the compiled artifact to the map
            if exit_code == Some(0) {
                if let Some(ref rlib_path_str) = compilation_result.compiled_checksum {
                    let crate_name = crate_root_path.file_name().unwrap().to_string_lossy().to_string();
                    compiled_artifacts_map.insert(crate_name, PathBuf::from(rlib_path_str));
                } else {
                    println!("Warning: No compiled .rlib path found in compilation result for {:?}", crate_root_path.display());
                }
            }

            // Temporarily disable state updates and caching
            // let mut current_state = state_arc.lock().unwrap();
            // result_store.save_result(&compilation_result, &current_state.output_dir)?;

            // // Update cache
            // current_state.cache.insert(crate_root_path.clone(), compilation_result.clone());

            // if should_update_state {
            //     if exit_code == Some(0) {
            //         // Move successful result to done_dir
            //         let file_name = crate_root_path.file_name().unwrap().to_string_lossy().replace(".", "_");
            //         let old_path = current_state.output_dir.join(format!("{}_result.json", file_name));
            //         let new_path = current_state.done_dir.join(format!("{}_result.json", file_name));
            //         fs::rename(&old_path, &new_path).map_err(AppError::Io)?;
            //         current_state.update_file_status(&crate_root_path, FileStatus::Done)?;
            //         println!("Moved result for {:?} to done_results.", crate_root_path.display());
            //     } else {
            //         current_state.update_file_status(&crate_root_path, FileStatus::Failed)?;
            //         println!("Compilation failed for {:?}. Stopping.", crate_root_path.display());
            //         current_state.save(&main_state_file_path_clone)?;
            //         return Err(AppError::Custom(format!("Compilation failed for {:?}", crate_root_path.display())));
            //     }
            //     current_state.save(&main_state_file_path_clone)?;
            // } else {
                if exit_code != Some(0) {
                    println!("Compilation failed for {:?}. (State update skipped)", crate_root_path.display());
                    println!("--- STDOUT ---");
                    println!("{}", compilation_result.stdout);
                    println!("--- STDERR ---");
                    println!("{}", compilation_result.stderr);
                    return Err(AppError::Custom(format!("Compilation failed for {:?}", crate_root_path.display())));
                } else {
                    println!("Compilation successful for {:?}. (State update skipped)", crate_root_path.display());
                }
            // }
        }
    }
    Ok(())
}
