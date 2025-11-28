use lsp_server::{Connection, Message};
use lsp_types::{
    InitializeParams, ServerCapabilities, TextDocumentSyncCapability,
    TextDocumentSyncKind,
};
use anyhow::{Context, Result, anyhow};
use std::path::{PathBuf, Path}; // Need PathBuf and Path for project_root
use std::io; // Added for stdin().read_line
use rocksdb::{
    DB, Options
};
use mcp_plugin_traits::{McpPlugin, MorphologicalIndex}; // For plugin handling
use std::process::Command; // For plugin rebuilding
use libloading::{Library, Symbol}; // For plugin loading
use clap::Parser; // NEW: For Cli::parse()

// Module imports
mod new_test_function; // Keep this as is for now
mod cli_args;
mod analysis_types;
mod hasher;
mod file_ingestion;
mod query_analysis;
mod file_retrieval;
mod bootstrapper;
mod plan_generator; // Re-added
mod lsp_handlers;

// Use statements
use cli_args::Cli;
use analysis_types::{PluginMetadata, ProjectFileAnalysis, IngestionChunk, IngestionFileDescriptor};
use hasher::calculate_content_id; // Still needed for plugin handling
use file_ingestion::{scan_and_ingest_project};
use query_analysis::query_project_analysis;
use file_retrieval::get_file_analysis;
use bootstrapper::boot_compiler;
use plan_generator::generate_ingestion_plan; // Re-added
use lsp_handlers::{ANALYZE_CODE_COMMAND, handle_request, handle_notification, analyze_code};


fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize RocksDB
    let db_path = PathBuf::from("./mcp_db");
    let mut db_options = Options::default();
    db_options.create_if_missing(true);
    let db = DB::open(&db_options, &db_path)
        .context(format!("Failed to open RocksDB at {:?}", db_path))?;
    eprintln!("RocksDB initialized at {:?}", db_path);

    if let Some(plugin_path_str) = cli.plugin_path {
        // This block needs to be moved to a plugin_manager module later.
        // For now, it remains in main.
        let plugin_path = PathBuf::from(&plugin_path_str);

        if let Some(plugin_crate_path_str) = cli.rebuild_plugin {
            eprintln!("Rebuilding plugin crate: {}", plugin_crate_path_str);
            let plugin_crate_path = PathBuf::from(&plugin_crate_path_str);
            
            // Extract crate name from path
            let plugin_crate_name = plugin_crate_path.file_name()
                .and_then(|s| s.to_str())
                .context(format!("Invalid plugin crate path: {}", plugin_crate_path_str))?;

            let build_output = Command::new("cargo")
                .arg("build")
                .arg("--release") // Build in release mode for dynamic libraries
                .arg("--lib")
                .arg(format!("--package={}", plugin_crate_name))
                .arg(format!("--manifest-path={}", plugin_crate_path.join("Cargo.toml").display()))
                .output()
                .context(format!("Failed to execute cargo build for plugin: {}", plugin_crate_name))?;

            if !build_output.status.success() {
                return Err(anyhow!(
                    "Plugin rebuild failed for '{}':\nStdout: {}\nStderr: {}",
                    plugin_crate_name,
                    String::from_utf8_lossy(&build_output.stdout),
                    String::from_utf8_lossy(&build_output.stderr)
                ));
            }
            eprintln!("Plugin '{}' rebuilt successfully.", plugin_crate_name);
        }

        eprintln!("Running in direct plugin execution mode for: {}", plugin_path_str);
        
        // --- Dynamic Plugin Loading Logic ---
        // SAFETY: Loading dynamic libraries and calling C functions is inherently unsafe.
        // We assume the plugin provides a safe interface and matches the expected function signatures.
        let lib = unsafe { Library::new(&plugin_path) } // Library::new is unsafe
            .context(format!("Failed to load dynamic library: {}", plugin_path_str))?;
        
        // Resolve the create_plugin symbol
        let create_plugin: Symbol<fn() -> Box<dyn McpPlugin>> = unsafe { lib.get(b"create_plugin") }
            .context("Failed to find 'create_plugin' symbol in plugin library")?;
        
        // Resolve the destroy_plugin symbol
        let destroy_plugin: Symbol<fn(Box<dyn McpPlugin>)> = unsafe { lib.get(b"destroy_plugin") }
            .context("Failed to find 'destroy_plugin' symbol in plugin library")?;

        // Create an instance of the plugin
        let plugin = create_plugin();

        eprintln!("Loaded plugin: {} (v{})", plugin.name(), plugin.version());
        
        let test_input = "Hello from MCP server!";
        let plugin_result = plugin.execute(test_input)
            .context(format!("Plugin '{}' execution failed", plugin.name()))?;
        
        println!("Plugin Output: {}", plugin_result);

        // Store plugin metadata in RocksDB
        let plugin_binary_data = std::fs::read(&plugin_path)
            .context(format!("Failed to read plugin binary: {}", plugin_path_str))?;
        let plugin_content_id = calculate_content_id(&plugin_binary_data);

        let metadata = PluginMetadata {
            name: plugin.name().to_string(),
            version: plugin.version().to_string(),
            content_id: plugin_content_id.clone(),
            path: plugin_path_str.clone(),
        };
        let metadata_json = serde_json::to_string(&metadata)?;
        db.put(plugin_content_id.as_bytes(), metadata_json.as_bytes())
            .context("Failed to write plugin metadata to RocksDB")?;
        eprintln!("Plugin metadata stored in RocksDB with content ID: {}", metadata.content_id);

        // Retrieve and store morphological index
        let morphological_index = plugin.morphological_index();
        let morphological_index_json = serde_json::to_string(&morphological_index)?;
        let morphological_index_key = format!("{}_morphological_index", plugin_content_id);
        db.put(morphological_index_key.as_bytes(), morphological_index_json.as_bytes())
            .context("Failed to write morphological index to RocksDB")?;
        eprintln!("Morphological index stored in RocksDB with key: {}", morphological_index_key);

        // Example of retrieving data
        if let Some(retrieved_data) = db.get(plugin_content_id.as_bytes())
            .context("Failed to retrieve plugin metadata from RocksDB")? {
            let retrieved_metadata: PluginMetadata = serde_json::from_slice(&retrieved_data)?;
            eprintln!("Retrieved metadata from RocksDB: {:?}", retrieved_metadata);
        }
        if let Some(retrieved_index_data) = db.get(morphological_index_key.as_bytes())
            .context("Failed to retrieve morphological index from RocksDB")? {
            let retrieved_morphological_index: MorphologicalIndex = serde_json::from_slice(&retrieved_index_data)?;
            eprintln!("Retrieved morphological index from RocksDB: {:?}", retrieved_morphological_index);
        }
        
        // Explicitly destroy the plugin instance to prevent memory leaks.
        destroy_plugin(plugin); // Call the safe Rust function
        
        eprintln!("Plugin execution complete.");
        Ok(())
        // --- End Dynamic Plugin Loading Logic ---

    } else if let Some(file_path) = cli.file {
        eprintln!("Running in direct file analysis mode for: {}", file_path);
        let project_root = PathBuf::from("."); // Use current directory as context
        scan_and_ingest_project(&db, &project_root)?; // Calls the moved function
        eprintln!("File analysis complete.");
        Ok(())
    } else if let Some(project_path_str) = cli.project_path {
        eprintln!("Scanning and ingesting project at: {}", project_path_str);
        let project_root = PathBuf::from(&project_path_str);
        scan_and_ingest_project(&db, &project_root)?; // Calls the moved function
        eprintln!("Project ingestion complete.");
        Ok(())
    } else if cli.query_project_analysis { // NEW BRANCH
        eprintln!("Querying project analysis from RocksDB...");
        query_analysis::query_project_analysis(&db)?; // Calls the moved function
        eprintln!("Query complete.");
        Ok(())
    } else if let Some(file_path) = cli.get_file_analysis {
        eprintln!("Retrieving file analysis for: {}", file_path);
        file_retrieval::get_file_analysis(&db, &file_path)?; // Calls the moved function
        eprintln!("File analysis retrieved.");
        Ok(())
    } else if let Some(boot_args) = cli.boot {
        let compiler_source_path = &boot_args[0];
        let target_source_path = &boot_args[1];
        eprintln!("Initiating bootstrap compilation:");
        eprintln!("  Compiler Source: {}", compiler_source_path);
        eprintln!("  Target Source: {}", target_source_path);
        bootstrapper::boot_compiler(&db, compiler_source_path, target_source_path)?; // Calls the moved function
        eprintln!("Bootstrap compilation initiated.");
        Ok(())
    } else if cli.tycoon_start_simulation { // New flag for starting tycoon
        let rustc_main_path = PathBuf::from("submodules/rust/compiler/rustc/src/main.rs");
        eprintln!("Initiating Rust Tycoon meme simulation with base: {:?}", rustc_main_path);
        let project_root = PathBuf::from("submodules/rust/compiler/rustc/"); // Project root for rustc
        scan_and_ingest_project(&db, &project_root)?;
        eprintln!("Project ingested for Tycoon simulation.");
        eprintln!("Performing initial analysis for Tycoon iteration...");
        query_analysis::query_project_analysis(&db)?; // Simulate analysis
        eprintln!("Analysis complete. Next: Apply transformation and re-ingest for next 'tycoon' generation.");
        // This marks the end of a single "tycoon" iteration.
        eprintln!("Rust Tycoon simulation initial iteration complete.");
        let rustc_main_content = std::fs::read_to_string(&rustc_main_path)
            .context(format!("Failed to read rustc main file: {:?}", rustc_main_path))?;
        eprintln!("Loaded rustc main content (first 100 chars): {}", &rustc_main_content[0..std::cmp::min(rustc_main_content.len(), 100)]);

        eprintln!("\n--- Rust Tycoon: Identifying Needs (Dependencies) ---");
        let use_statements: Vec<_> = rustc_main_content.lines()
            .filter_map(|line| {
                if line.trim().starts_with("use ") {
                    Some(line.trim().to_string())
                } else {
                    None
                }
            })
            .collect();

        if use_statements.is_empty() {
            eprintln!("No 'use' statements found as immediate needs.");
        } else {
            eprintln!("Detected the following 'use' statements (potential parts to buy):");
            for (i, statement) in use_statements.iter().enumerate() {
                eprintln!("{}. {}", i + 1, statement);
            }
        }
        eprintln!("----------------------------------------------------\n");
        eprintln!("Rust Tycoon: Press Ctrl+F to focus and interact.");

        if !use_statements.is_empty() {
            loop {
                eprintln!("\nEnter the number of the 'part' (use statement) you want to 'buy', or '0' to skip for now:");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)
                    .context("Failed to read line from stdin")?;
                let choice: usize = match input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        eprintln!("Invalid input. Please enter a number.");
                        continue;
                    }
                };

                if choice == 0 {
                    eprintln!("Skipping 'buying' parts for now.");
                    break;
                }

                if choice > 0 && choice <= use_statements.len() {
                    let bought_part = &use_statements[choice - 1];
                    eprintln!("You 'bought' the part: {}", bought_part);
                    // In a real implementation, this would trigger actions like:
                    // - Adding to Cargo.toml
                    // - Generating mock code
                    // - Performing further analysis
                    // For now, it's a simulation.
                    break; // For this minimal iteration, buy one and exit.
                } else {
                    eprintln!("Choice out of range. Please try again.");
                }
            }
        }
        // TODO: More sophisticated "Roblox-like" interaction will follow.

        Ok(())
    } else {
        eprintln!("Starting MCP server in LSP mode...");

        let (connection, io_threads) = Connection::stdio();

        let server_capabilities = serde_json::to_value(&lsp_types::ServerCapabilities {
            text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
            execute_command_provider: Some(lsp_types::ExecuteCommandOptions {
                commands: vec![lsp_handlers::ANALYZE_CODE_COMMAND.to_string()],
                work_done_progress_options: Default::default(),
            }),
            ..lsp_types::ServerCapabilities::default()
        })?;

        let initialization_params = connection.initialize(server_capabilities)?;
        let _params: lsp_types::InitializeParams = serde_json::from_value(initialization_params)?;

        eprintln!("Initialized LSP server.");

        for msg in &connection.receiver {
            eprintln!("Received message: {:?}", msg);
            match msg {
                Message::Request(req) => {
                    if connection.handle_shutdown(&req)? {
                        break;
                    }
                    lsp_handlers::handle_request(&connection, req)?;
                }
                Message::Notification(notification) => {
                    lsp_handlers::handle_notification(&connection, notification)?;
                }
                _ => {}
            }
        }

        io_threads.join()?;
        eprintln!("MCP server stopped.");
        Ok(())
    }
}