use crate::config::CompilerConfig;
use crate::traits::{ConfigHandler, Compiler, ResultStore};
use clap::Parser;
use std::fs;
use std::collections::HashMap;
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
    Ok(())

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

    let mut files_to_process_initial: Vec<PathBuf> = Vec::new();

    if let Some(target_level) = args.level {
        println!("Collecting files for crates at level: {}", target_level);
        for (crate_name, &layer) in &layered_crates {
            if layer == target_level {
                if let Some(crate_root_path) = crate_name_to_root_map.get(crate_name) {
                    println!("Found crate directory for {}: {:?}", crate_name, crate_root_path);
                    for entry in walkdir::WalkDir::new(crate_root_path) {
                        let entry = entry.map_err(AppError::Walkdir)?;
                        let path = entry.path();
                        if path.extension().map_or(false, |ext| ext == "rs") &&
                           !path.components().any(|comp| comp.as_os_str() == "tests" || comp.as_os_str() == "benches") {
                            files_to_process_initial.push(path.to_path_buf());
                        }
                    }
                } else {
                    println!("Warning: Could not find crate root path in map for crate: {}", crate_name);
                }
            }
        }
    } else {
        // Fallback to original behavior if no level is specified
        files_to_process_initial = state_arc.lock().unwrap().get_pending_files()?;
    }

        let should_update_state = args.level.is_none();
    
        let total_files_to_process = files_to_process_initial.len();
        let mut files_processed_count = 0;
    
        if total_files_to_process == 0 {
            println!("No pending or failed files to process. All files are up-to-date or done.");
            return Ok(());
        }
    
                for file_path in files_to_process_initial {
                    files_processed_count += 1;
                    let progress_percent = (files_processed_count as f64 / total_files_to_process as f64) * 100.0;
                    println!("Compiling file {} of {} ({:.2}%): {:?}", files_processed_count, total_files_to_process, progress_percent, file_path.display());
        
                    let file_hash = calculate_file_hash(&file_path)?;
        
                    // Check cache
                    let mut current_state = state_arc.lock().unwrap();
                    if let Some(cached_result) = current_state.cache.get(&file_path) {
                        if cached_result.source_checksum == file_hash {
                            println!("Cache hit for {:?}. Skipping compilation.", file_path.display());
                            // Use cached result
                            if should_update_state {
                                // Even if cached, update status in state if we are tracking state
                                if cached_result.exit_code == Some(0) {
                                    current_state.update_file_status(&file_path, FileStatus::Done)?;
                                } else {
                                    current_state.update_file_status(&file_path, FileStatus::Failed)?;
                                }
                                current_state.save(&main_state_file_path_clone)?;
                            }
                            continue;
                        }
                    }
                    drop(current_state); // Release lock before compilation
        
                    if args.dry_run {
                        println!("Dry run: Skipping compilation for {:?}", file_path.display());
                        continue;
                    }
        
                    let compilation_result = compiler.compile_file(&file_path, &rustc_path, &final_config)?;
                    let exit_code = compilation_result.exit_code;
        
                    let mut current_state = state_arc.lock().unwrap();
                    result_store.save_result(&compilation_result, &current_state.output_dir)?;
        
                    // Update cache
                    current_state.cache.insert(file_path.clone(), compilation_result.clone());
        
                    if should_update_state {
                        if exit_code == Some(0) {
                            // Move successful result to done_dir
                            let file_name = file_path.file_name().unwrap().to_string_lossy().replace(".", "_");
                            let old_path = current_state.output_dir.join(format!("{}_result.json", file_name));
                            let new_path = current_state.done_dir.join(format!("{}_result.json", file_name));
                            fs::rename(&old_path, &new_path).map_err(AppError::Io)?;
                            current_state.update_file_status(&file_path, FileStatus::Done)?;
                            println!("Moved result for {:?} to done_results.", file_path.display());
                        } else {
                            current_state.update_file_status(&file_path, FileStatus::Failed)?;
                            println!("Compilation failed for {:?}. Stopping.", file_path.display());
                            current_state.save(&main_state_file_path_clone)?;
                            return Err(AppError::Custom(format!("Compilation failed for {:?}", file_path.display())));
                        }
                        current_state.save(&main_state_file_path_clone)?;
                    } else {
                        if exit_code != Some(0) {
                            println!("Compilation failed for {:?}. (State update skipped)", file_path.display());
                            return Err(AppError::Custom(format!("Compilation failed for {:?}", file_path.display())));
                        } else {
                            println!("Compilation successful for {:?}. (State update skipped)", file_path.display());
                        }
                                    }
                        }
                        Ok(())
                    }
// Placeholder function - needs proper implementation
fn get_crate_name_from_path(file_path: &PathBuf, rust_src_path: &PathBuf) -> Option<String> {
    // This is a very naive implementation. A proper implementation would
    // involve parsing Cargo.toml files to determine which crate a file belongs to.
    // For now, we'll try to extract a component from the path.
    // Example: /rust_src_path/src/lib.rs -> "src" (not ideal)
    // Example: /rust_src_path/library/std/src/lib.rs -> "std"
    // Example: /rust_src_path/library/alloc/src/lib.rs -> "alloc"

    let relative_path = file_path.strip_prefix(rust_src_path).ok()?;
    let components: Vec<&str> = relative_path.iter().filter_map(|s| s.to_str()).collect();

    // This logic needs to be improved significantly.
    // For now, let's assume the crate name is the second component after "library"
    // or the first component if "library" is not present.
    if let Some(library_idx) = components.iter().position(|&s| s == "library") {
        if let Some(crate_name) = components.get(library_idx + 1) {
            return Some(crate_name.to_string());
        }
    } else if let Some(crate_name) = components.first() {
        return Some(crate_name.to_string());
    }

    None
}
