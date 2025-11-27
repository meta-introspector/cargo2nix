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
use std::path::PathBuf;
use rocksdb::{DB, Options};
use sha2::{Sha256, Digest};

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
}

#[derive(Debug, Serialize, Deserialize)]
struct PluginMetadata {
    name: String,
    version: String,
    content_id: String,
    path: String,
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
