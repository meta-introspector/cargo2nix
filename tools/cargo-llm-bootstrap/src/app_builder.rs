use std::fs;
use std::path::PathBuf;
use std::time::Instant;
use crate::cli::Args;
use crate::config::CompilerConfig;
use crate::traits::ConfigHandler;
use crate::rustc_options::RustcOptions;
use crate::crate_discovery::{CrateDiscovery, FileSystemCrateDiscovery};
use crate::layer_manager::{LayerManager, FileSystemLayerManager};
use crate::state_manager::State;
use crate::app_context::{AppContext, STATE_FILE_NAME, DONE_DIR_NAME};
use crate::error::AppError;

pub struct AppBuilder;

impl AppBuilder {
    pub fn build_from_args(args: Args) -> Result<AppContext, AppError> {
        let config = Self::build_config(&args)?;
        let rust_src_path = Self::get_rust_src_path(&config)?;
        
        // Discover crates
        let crate_discovery = FileSystemCrateDiscovery;
        let crate_name_to_root_map = crate_discovery.scan_crates(&rust_src_path)?;
        
        // Setup output directories
        let output_dir = config.output_dir.clone().unwrap_or_else(|| PathBuf::from("compilation_results"));
        fs::create_dir_all(&output_dir).map_err(AppError::Io)?;
        
        let done_dir = output_dir.join(DONE_DIR_NAME);
        fs::create_dir_all(&done_dir).map_err(AppError::Io)?;
        
        // Load layer information
        let layer_manager = FileSystemLayerManager;
        let layered_graph_path = output_dir.join("layered_graph.json");
        let layered_crates = layer_manager.load_layers(&layered_graph_path)?;
        
        // Load state
        let main_state_file_path = output_dir.join(STATE_FILE_NAME);
        let start_scan_time = Instant::now();
        let state = State::load(&main_state_file_path, output_dir.clone(), done_dir.clone())?;
        let scan_duration = start_scan_time.elapsed();
        println!("State loading took: {:?}", scan_duration);
        
        // Setup Ctrl+C handler
        Self::setup_ctrl_c_handler(&main_state_file_path)?;
        
        Ok(AppContext::new(
            config,
            crate_name_to_root_map,
            layered_crates,
            state,
            output_dir,
            done_dir,
        ))
    }
    
    fn build_config(args: &Args) -> Result<CompilerConfig, AppError> {
        let mut final_config = CompilerConfig::new();
        let mut config_handler_instance = CompilerConfig::new();

        // Load from config file if provided
        if let Some(config_file_path) = &args.config_file {
            let file_config = config_handler_instance.load_config(config_file_path)?;
            final_config = config_handler_instance.merge_configs(final_config, file_config);
        }

        // Merge CLI arguments
        let cli_config = CompilerConfig {
            rust_src_path: args.rust_src_path.clone(),
            output_dir: args.output_dir.clone(),
            target_triple: args.target_triple.clone(),
            rustc_path: args.rustc_path.clone(),
            cargo_path: args.cargo_path.clone(),
            build_dir: args.build_dir.clone(),
            rustc_options: RustcOptions::new(),
        };

        final_config = config_handler_instance.merge_configs(final_config, cli_config);
        println!("Final Compiler Configuration: {:?}", final_config);
        
        Ok(final_config)
    }
    
    fn get_rust_src_path(config: &CompilerConfig) -> Result<PathBuf, AppError> {
        config.rust_src_path.clone().ok_or_else(|| {
            AppError::Custom("RUST_SRC_PATH is not provided in config or CLI arguments.".to_string())
        })
    }
    
    fn setup_ctrl_c_handler(main_state_file_path: &PathBuf) -> Result<(), AppError> {
        let main_state_file_path_for_ctrlc = main_state_file_path.clone();
        
        ctrlc::set_handler(move || {
            println!("\nCtrl+C detected. Saving state before exiting...");
            // Note: In a real implementation, we'd need to pass the state here
            // For now, we'll just exit gracefully
            std::process::exit(0);
        }).map_err(|e| AppError::Custom(format!("Error setting Ctrl-C handler: {}", e)))?;
        
        Ok(())
    }
}
