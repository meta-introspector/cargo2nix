use axum::{
    routing::{get, post},
    http::StatusCode,
    response::Json,
    Router,
    extract::{State, Path as AxumPath, Query},
};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tokio;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

use introspector_core::continuation::{CapturedState, Resolution};

// Define a shared state for the application
type SharedState = Arc<Mutex<AppState>>;

#[derive(Default, Debug)]
struct AppState {
    states: HashMap<String, CapturedState>,
}

const STATE_DIR: &str = "/tmp/rustc_state";
const RESOLUTION_DIR: &str = "/tmp/rustc_resolution";

#[tokio::main]
async fn main() {
    let state = SharedState::new(Mutex::new(AppState::default()));

    // Spawn the file watcher in a background thread
    let watcher_state = state.clone();
    tokio::spawn(async move {
        watch_state_directory(watcher_state, Path::new(STATE_DIR)).await;
    });

    // Spawn the Axum server in a background thread
    let server_state = state.clone();
    tokio::spawn(async move {
        run_server(server_state).await;
    });

    // Run the REPL in the main thread
    run_repl(state).await;
}

// --- File System Watcher ---

async fn watch_state_directory(state: SharedState, path: &Path) {
    println!("Watching for state files in: {:?}", path);
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = RecommendedWatcher::new(tx, notify::Config::default()).unwrap();
    watcher.watch(path, RecursiveMode::NonRecursive).unwrap();

    for res in rx {
        match res {
            Ok(event) => {
                if event.kind.is_create() {
                    for path in event.paths {
                        println!("New state file detected: {:?}", path);
                        if let Ok(file_content) = fs::read_to_string(&path) {
                            if let Ok(captured_state) = serde_json::from_str::<CapturedState>(&file_content) {
                                println!("Successfully deserialized state for session: {}", captured_state.session_id);
                                let mut app_state = state.lock().unwrap();
                                app_state.states.insert(captured_state.session_id.clone(), captured_state);
                            }
                        }
                    }
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}


// --- Axum Web Server ---

async fn run_server(state: SharedState) {
    let app = Router::new()
        .route("/states", get(get_all_states))
        .route("/state/:id", get(get_state_by_id))
        .route("/resolve", post(post_resolution))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Axum server listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_all_states(State(state): State<SharedState>) -> (StatusCode, Json<Vec<String>>) {
    let app_state = state.lock().unwrap();
    let keys = app_state.states.keys().cloned().collect();
    (StatusCode::OK, Json(keys))
}

async fn get_state_by_id(State(state): State<SharedState>, AxumPath(id): AxumPath<String>) -> Result<Json<CapturedState>, StatusCode> {
    let app_state = state.lock().unwrap();
    if let Some(captured_state) = app_state.states.get(&id) {
        Ok(Json(captured_state.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[derive(Deserialize)]
struct ResolutionRequest {
    resolution: Resolution,
    session_id: String, // ID to match the state file
}

async fn post_resolution(
    State(_state): State<SharedState>,
    Json(payload): Json<ResolutionRequest>,
) -> StatusCode {
    println!("Received resolution via API: {:?}", payload.resolution);
    let resolution_file = PathBuf::from(RESOLUTION_DIR).join(format!("resolution_{}.json", payload.session_id));
    if let Ok(json) = serde_json::to_string_pretty(&payload.resolution) {
        if fs::write(&resolution_file, json).is_ok() {
            println!("Successfully wrote resolution to {:?}", resolution_file);
            return StatusCode::OK;
        }
    }
    StatusCode::INTERNAL_SERVER_ERROR
}


// --- Rustyline REPL ---

async fn run_repl(state: SharedState) {
    let mut rl = DefaultEditor::new().unwrap();
    println!("REPL started. Type 'help' for a list of commands.");

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).unwrap();
                let parts: Vec<&str> = line.trim().split_whitespace().collect();
                if let Some(command) = parts.get(0) {
                    match *command {
                        "help" => {
                            println!("Commands:");
                            println!("  states                                   - List all captured session IDs.");
                            println!("  state <session_id>                     - Show the captured state for a session.");
                            println!("  resolve <session_id> continue            - Send a 'Continue' resolution.");
                            println!("  resolve <session_id> modify <file> <l> <c> <code> - Send a 'ModifyCode' resolution.");
                            println!("  exit                                     - Exit the REPL.");
                        }
                        "states" => {
                             let app_state = state.lock().unwrap();
                             let keys: Vec<String> = app_state.states.keys().cloned().collect();
                             println!("Captured sessions: {:?}", keys);
                        }
                        "state" => {
                            if let Some(session_id) = parts.get(1) {
                                let app_state = state.lock().unwrap();
                                if let Some(captured) = app_state.states.get(*session_id) {
                                     println!("{}", serde_json::to_string_pretty(captured).unwrap());
                                } else {
                                    println!("No state found for session ID: {}", session_id);
                                }
                            } else {
                                println!("Usage: state <session_id>");
                            }
                        }
                        "resolve" => {
                            if parts.len() < 3 {
                                println!("Usage: resolve <session_id> <continue|modify ...>");
                                continue;
                            }
                            let session_id = parts[1];
                            let resolution_type = parts[2];
                            let resolution_file = PathBuf::from(RESOLUTION_DIR).join(format!("resolution_{}.json", session_id));

                            let resolution = match resolution_type {
                                "continue" => Some(Resolution::Continue),
                                "modify" => {
                                    if parts.len() >= 7 {
                                        let file = parts[3].to_string();
                                        let line: u32 = parts[4].parse().unwrap_or(0);
                                        let column: u32 = parts[5].parse().unwrap_or(0);
                                        let new_code = parts[6..].join(" ");
                                        Some(Resolution::ModifyCode { file, line, column, new_code })
                                    } else {
                                        println!("Usage: resolve <session_id> modify <file> <line> <col> <code>");
                                        None
                                    }
                                }
                                _ => {
                                    println!("Unknown resolve command. Try 'continue' or 'modify'.");
                                    None
                                }
                            };

                            if let Some(res) = resolution {
                                 if let Ok(json) = serde_json::to_string_pretty(&res) {
                                    if fs::write(&resolution_file, json).is_ok() {
                                        println!("Successfully wrote resolution to {:?}", resolution_file);
                                    } else {
                                        println!("Error writing resolution file.");
                                    }
                                }
                            }
                        }
                        "exit" => {
                            println!("Exiting REPL.");
                            break;
                        }
                        _ => {
                            println!("Unknown command: {}", line);
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}