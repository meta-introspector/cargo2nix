use lsp_server::{Connection, Message, Notification, Request, RequestId, Response};
use lsp_types::{
    InitializeParams, InitializeResult, ServerCapabilities, TextDocumentSyncCapability,
    TextDocumentSyncKind, Url,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use anyhow::{Context, Result};
use syn::{visit::Visit, Item, ItemFn};
use clap::Parser; // Import Parser

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
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Some(file_path) = cli.file {
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