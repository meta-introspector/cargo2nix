use crate::config::CompilerConfig;
use crate::traits::{ConfigHandler, Compiler, ResultStore};
use clap::Parser;
use std::fs;
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

    /// Optional Rust file to process
    file: Option<PathBuf>,

    /// Recompile the last N failed files (e.g., --quick-boil 1)
    #[arg(long)]
    quick_boil: Option<u32>,
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

    let output_dir = final_config.output_dir.clone().unwrap_or_else(|| PathBuf::from("compilation_results"));
    fs::create_dir_all(&output_dir).map_err(AppError::Io)?;

    let done_dir = output_dir.join(DONE_DIR_NAME);
    fs::create_dir_all(&done_dir).map_err(AppError::Io)?;

    let rustc_path = final_config.rustc_path.clone().unwrap_or_else(|| PathBuf::from("rustc"));

    let main_state_file_path = output_dir.join(STATE_FILE_NAME);
    let main_state_file_path_clone = main_state_file_path.clone();
    let main_state_file_path_for_ctrlc = main_state_file_path.clone();

    // --- Initial State Loading ---
    let start_scan_time = Instant::now();
    let state = State::load(&main_state_file_path, output_dir.clone(), done_dir.clone())
        .map_err(|e| {
            if let AppError::Custom(msg) = &e {
                if msg.starts_with("Main state file not found") {
                    return AppError::Custom(format!(
                        "{}
Please run `cargo run --package rust-src-scanner -- --rust-src-path {:?} --output-dir {:?}` first to generate the initial state.",
                        msg, rust_src_path, output_dir
                    ));
                }
            }
            e
        })?;
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

    let files_to_process_initial = if let Some(quick_boil_count) = args.quick_boil {
        if quick_boil_count == 1 {
            let current_state = state_arc.lock().unwrap();
            if let Some(last_failed_file) = current_state.get_last_failed_file()? {
                println!("Quick boil mode: Recompiling last failed file: {:?}", last_failed_file.display());
                vec![last_failed_file]
            } else {
                println!("Quick boil mode: No failed files found to recompile.");
                return Ok(());
            }
        } else {
            println!("Quick boil mode: Only --quick-boil 1 is currently supported. Processing all pending/failed files.");
            state_arc.lock().unwrap().get_pending_files()?
        }
    } else if let Some(single_file_path) = args.file {
        vec![single_file_path]
    } else {
        state_arc.lock().unwrap().get_pending_files()?
    };

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

        let compilation_result = compiler.compile_file(&file_path, &rustc_path, &final_config)?;
        let exit_code = compilation_result.exit_code;

        let mut current_state = state_arc.lock().unwrap();
        result_store.save_result(&compilation_result, &current_state.output_dir)?;

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
    }

    Ok(())
}
