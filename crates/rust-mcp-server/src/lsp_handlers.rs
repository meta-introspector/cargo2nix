use anyhow::{Context, Result};
use lsp_server::{Connection, Message, Notification, Request, RequestId, Response};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use syn::{visit::Visit, ItemFn};

// Custom command to analyze code
pub const ANALYZE_CODE_COMMAND: &str = "mcp/analyzeCode";

use crate::file_ingestion::RustItemCollector; // Changed from FunctionNameCollector

pub fn handle_request(connection: &Connection, req: Request) -> Result<()> {
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

pub fn handle_notification(connection: &Connection, notification: Notification) -> Result<()> {
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
pub struct AnalyzeCodeParams {
    pub text: String,
    // Add other parameters like file_path, custom analysis options etc.
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalyzeCodeResult {
    pub function_names: Vec<String>,
    // Add other analysis results
}

pub fn analyze_code(params: Value) -> Result<AnalyzeCodeResult> {
    let analyze_params: AnalyzeCodeParams = serde_json::from_value(params)?;
    let syntax_tree = syn::parse_file(&analyze_params.text)
        .context("Failed to parse Rust code")?;

    let mut collector = RustItemCollector::new(); // Changed to RustItemCollector
    collector.visit_file(&syntax_tree);

    Ok(AnalyzeCodeResult {
        function_names: collector.functions,
    })
}