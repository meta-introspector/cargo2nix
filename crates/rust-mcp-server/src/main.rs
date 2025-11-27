use lsp_server::{Connection, Message, Notification, Request, RequestId, Response};
use lsp_types::{
    InitializeParams, InitializeResult, ServerCapabilities, TextDocumentSyncCapability,
    TextDocumentSyncKind, Url,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use anyhow::{Context, Result, anyhow};
use syn::{visit::Visit, Item, ItemFn};
use clap::Parser;
use libloading::{Library, Symbol};
use mcp_plugin_traits::{McpPlugin, MorphologicalIndex};
use std::process::Command;
use std::path::{PathBuf, Path};
use rocksdb::{DB, Options};
use sha2::{Sha256, Digest};
use walkdir::WalkDir; // Added for project scanning
use std::collections::HashMap; // Added for query_project_analysis

mod new_test_function;

// Custom command to analyze code
const ANALYZE_CODE_COMMAND: &str = "mcp/analyzeCode";

/// A visitor to collect function names
struct FunctionNameCollector {
    functions: Vec<String>,
}

impl FunctionNameCollector {
    fn new() -> Self {
        FunctionNameCollector { functions: Vec::new() }
    }
}

impl<'ast> Visit<'ast> for FunctionNameCollector {
    fn visit_item_fn(&mut self, i: &'ast ItemFn) {
        self.functions.push(i.sig.ident.to_string());
        syn::visit::visit_item_fn(self, i);
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Optional: Path to a Rust file to analyze directly via CLI, bypassing LSP.
    #[clap(long)]
    file: Option<String>,
    /// Optional: Path to a dynamic library (plugin) to load and execute directly.
    #[clap(long)]
    plugin_path: Option<String>,
    /// Optional: Path to the plugin crate to rebuild before loading (e.g., crates/mcp-plugin-example).
    #[clap(long)]
    rebuild_plugin: Option<String>,
    /// Optional: Path to a project directory to scan, analyze Rust files, and ingest into RocksDB.
    #[clap(long)]
    project_path: Option<String>,
    /// Optional: Query RocksDB for project analysis data, counting unique and duplicate content hashes.
    #[clap(long)]
    query_project_analysis: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct PluginMetadata {
    name: String,
    version: String,
    content_id: String,
    path: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProjectFileAnalysis {
    file_path: String,
    content_hash: String,
    file_type: String, // "rust", "markdown", "toml", "unknown"
    function_names: Option<Vec<String>>, // Only for Rust files
}

/// Helper function to calculate SHA256 hash of data
fn calculate_content_id(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

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
        let code = std::fs::read_to_string(&file_path)
            .context(format!("Failed to read file: {}", file_path))?;
        
        let syntax_tree = syn::parse_file(&code)
            .context("Failed to parse Rust code from file")?;

        let mut collector = FunctionNameCollector::new();
        collector.visit_file(&syntax_tree);

        let result = AnalyzeCodeResult {
            function_names: collector.functions,
        };
        println!("{}", serde_json::to_string_pretty(&result)?);
        Ok(())
    } else if let Some(project_path_str) = cli.project_path {
        eprintln!("Scanning and ingesting project at: {}", project_path_str);
        let project_root = PathBuf::from(&project_path_str);
        scan_and_ingest_project(&db, &project_root)?;
        eprintln!("Project ingestion complete.");
        Ok(())
    } else if cli.query_project_analysis { // NEW BRANCH
        eprintln!("Querying project analysis from RocksDB...");
        query_project_analysis(&db)?;
        eprintln!("Query complete.");
        Ok(())
    } else {
        // Note: lsp_server does not use stdio directly, it uses a pipe.
        // For this example, we assume it's running via a client that sets up stdin/stdout pipes.
        eprintln!("Starting MCP server in LSP mode...");

        // Create the LSP connection.
        let (connection, io_threads) = Connection::stdio();

        // Run the server and wait for the client to initialize.
        let server_capabilities = serde_json::to_value(&ServerCapabilities {
            text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
            // Register custom command for code analysis
            execute_command_provider: Some(lsp_types::ExecuteCommandOptions {
                commands: vec![ANALYZE_CODE_COMMAND.to_string()],
                work_done_progress_options: Default::default(),
            }),
            ..ServerCapabilities::default()
        })?;

        let initialization_params = connection.initialize(server_capabilities)?;
        let _params: InitializeParams = serde_json::from_value(initialization_params)?;

        eprintln!("Initialized LSP server.");

        // Event loop
        for msg in &connection.receiver {
            eprintln!("Received message: {:?}", msg);
            match msg {
                Message::Request(req) => {
                    if connection.handle_shutdown(&req)? {
                        break;
                    }
                    handle_request(&connection, req)?;
                }
                Message::Notification(notification) => {
                    handle_notification(&connection, notification)?;
                }
                _ => {}
            }
        }

        io_threads.join()?;
        eprintln!("MCP server stopped.");
        Ok(())
    }
}

fn handle_request(connection: &Connection, req: Request) -> Result<()> {
    let Request { id, method, params, .. } = req;
    match method.as_str() {
        ANALYZE_CODE_COMMAND => {
            // A custom command to analyze code content
            let result = analyze_code(params)?;
            let resp = Response::new_ok(id, serde_json::to_value(result)?);
            connection.sender.send(Message::Response(resp))?;
        }
        _ => {
            // Handle unknown requests or other LSP requests
            let resp = Response::new_err(
                id,
                lsp_server::ErrorCode::MethodNotFound as i32,
                format!("Unknown method: {}", method),
            );
            connection.sender.send(Message::Response(resp))?;
        }
    }
    Ok(())
}

fn handle_notification(connection: &Connection, notification: Notification) -> Result<()> {
    // Handle LSP notifications (e.g., textDocument/didOpen, textDocument/didChange)
    // For a bare minimum, we might just log them or ignore.
    match notification.method.as_str() {
        "initialized" => {
            eprintln!("Client reports initialized.");
        }
        _ => {
            eprintln!("Unhandled notification: {:?}", notification);
        }
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct AnalyzeCodeParams {
    text: String,
    // Add other parameters like file_path, custom analysis options etc.
}

#[derive(Debug, Serialize, Deserialize)]
struct AnalyzeCodeResult {
    function_names: Vec<String>,
    // Add other analysis results
}

fn analyze_code(params: Value) -> Result<AnalyzeCodeResult> {
    let analyze_params: AnalyzeCodeParams = serde_json::from_value(params)?;
    let syntax_tree = syn::parse_file(&analyze_params.text)
        .context("Failed to parse Rust code")?;

    let mut collector = FunctionNameCollector::new();
    collector.visit_file(&syntax_tree);

    Ok(AnalyzeCodeResult {
        function_names: collector.functions,
    })
}

// New function to scan and ingest project files
fn scan_and_ingest_project(db: &DB, project_root: &Path) -> Result<()> {
    let mut files_processed = 0;
    for entry in WalkDir::new(project_root)
        .into_iter()
        .filter_entry(|e| !e.path().to_string_lossy().contains("submodules/rust/tests/ui/")) // Exclude problematic test directories
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            let file_path_str = path.to_string_lossy().to_string();
            let extension = path.extension().and_then(|s| s.to_str());

            let (file_type, mut function_names) = match extension {
                Some("rs") => ("rust", Some(Vec::new())),
                Some("md") => ("markdown", None),
                Some("toml") => ("toml", None),
                _ => ("unknown", None),
            };

            // Only process known file types for ingestion
            if file_type == "unknown" {
                continue;
            }

            eprintln!("Analyzing file: {} (type: {})", file_path_str, file_type);

            let code = match std::fs::read_to_string(&path) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Warning: Failed to read file {} as UTF-8: {}", file_path_str, e);
                    continue; // Skip this file and continue to the next
                }
            };
            let file_content_hash = calculate_content_id(code.as_bytes());

            if file_type == "rust" {
                let syntax_tree = match std::panic::catch_unwind(|| syn::parse_file(&code)) {
                    Ok(Ok(tree)) => tree,
                    Ok(Err(e)) => {
                        eprintln!("Warning: Failed to parse Rust code from file {}: {}", file_path_str, e);
                        continue; // Skip this file and continue to the next
                    },
                    Err(e) => {
                        eprintln!("Warning: Panic while parsing Rust code from file {}: {:?}", file_path_str, e);
                        continue; // Skip this file and continue to the next
                    }
                };

                let mut collector = FunctionNameCollector::new();
                collector.visit_file(&syntax_tree);
                function_names = Some(collector.functions);
            }

            let analysis = ProjectFileAnalysis {
                file_path: file_path_str.clone(),
                content_hash: file_content_hash.clone(),
                file_type: file_type.to_string(),
                function_names,
            };

            let analysis_json = serde_json::to_string(&analysis)?;
            let db_key = format!("file_analysis:{}:{}:{}", file_type, file_path_str, file_content_hash);
            db.put(db_key.as_bytes(), analysis_json.as_bytes())
                .context(format!("Failed to write analysis for {} to RocksDB", file_path_str))?;
            eprintln!("Stored analysis for {} in RocksDB.", file_path_str);
            files_processed += 1;
        }
    }
    eprintln!("Processed {} files.", files_processed);
    Ok(())
}

// NEW FUNCTION: query_project_analysis
fn query_project_analysis(db: &DB) -> Result<()> {
    let mut all_analyses = Vec::new();
    let mut content_hash_counts: HashMap<String, usize> = HashMap::new();
    let mut duplicate_content_hashes: HashMap<String, Vec<String>> = HashMap::new();
    let mut file_type_counts: HashMap<String, usize> = HashMap::new();
    let mut unique_file_type_hashes: HashMap<String, HashMap<String, usize>> = HashMap::new();


    let prefix = b"file_analysis:";

    for item in db.iterator(rocksdb::IteratorMode::Start) {
        let (key_bytes, value_bytes) = item?;
        let key_str = String::from_utf8_lossy(&key_bytes);

        if key_str.starts_with("file_analysis:") {
            if let Ok(analysis) = serde_json::from_slice::<ProjectFileAnalysis>(&value_bytes) {
                *content_hash_counts.entry(analysis.content_hash.clone()).or_insert(0) += 1;
                *file_type_counts.entry(analysis.file_type.clone()).or_insert(0) += 1;
                *unique_file_type_hashes.entry(analysis.file_type.clone())
                                         .or_insert_with(HashMap::new)
                                         .entry(analysis.content_hash.clone())
                                         .or_insert(0) += 1;
                all_analyses.push(analysis);
            } else {
                eprintln!("Warning: Failed to deserialize RocksDB entry: {}", key_str);
            }
        }
    }

    // Identify duplicate content hashes
    for analysis in &all_analyses {
        if let Some(&count) = content_hash_counts.get(&analysis.content_hash) {
            if count > 1 {
                duplicate_content_hashes.entry(analysis.content_hash.clone())
                                        .or_insert_with(Vec::new)
                                        .push(analysis.file_path.clone());
            }
        }
    }

    eprintln!("\n--- Project Analysis Summary ---");
    eprintln!("Total analyzed files indexed: {}", all_analyses.len());
    eprintln!("Unique file content hashes (overall): {}", content_hash_counts.len());
    eprintln!("Files with duplicate content hashes (overall): {}", duplicate_content_hashes.len());

    eprintln!("\n--- Analysis by File Type ---");
    for (file_type, count) in file_type_counts {
        let unique_hashes = unique_file_type_hashes.get(&file_type).map_or(0, |m| m.len());
        eprintln!("  {}: Total = {}, Unique Hashes = {}", file_type, count, unique_hashes);
    }


    if !duplicate_content_hashes.is_empty() {
        eprintln!("\n--- Details of Duplicate Content Hashes (Overall) ---");
        // Sort duplicates by content hash for consistent output
        let mut sorted_duplicates: Vec<_> = duplicate_content_hashes.into_iter().collect();
        sorted_duplicates.sort_by(|a, b| a.0.cmp(&b.0));

        for (hash, paths) in sorted_duplicates {
            eprintln!("Content Hash: {}", hash);
            // Sort paths for consistent output
            let mut sorted_paths = paths;
            sorted_paths.sort();
            for path in sorted_paths {
                eprintln!("  - {}", path);
            }
        }
    } else {
        eprintln!("\nNo duplicate content hashes found.");
    }

    Ok(())
}
