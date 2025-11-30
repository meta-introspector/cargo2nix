use anyhow::{Result, Context};
use clap::Parser;
use std::path::PathBuf;
use std::sync::Arc;
// Removed: use std::io; // No longer needed for interactive input

// Corrected imports:
use rusttycoon::Factory;
use rusttycoon::FactoryBlock;
use rusttycoon::get_available_tools;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to the main Rust program to analyze and build the tycoon factory around.
    /// Used for initial project ingestion if a state is not loaded.
    #[arg(long)]
    pub main_program_path: Option<PathBuf>, // Made optional

    /// The command to execute (e.g., "buy-tool", "use-tool", "view-blueprint", "exit").
    #[arg(long)]
    pub cmd: String,

    /// The name of the tool to buy or use. Required for "buy-tool" and "use-tool" commands.
    #[arg(long)]
    pub tool_name: Option<String>,

    /// The serialized JSON string of the Factory state for the current turn.
    /// If not provided, a new Factory state is initialized.
    #[arg(long)]
    pub turn_state_key: Option<String>,

    /// If provided, the updated Factory state will be serialized to JSON and emitted to stdout
    /// with this key.
    #[arg(long)]
    pub emit_turn_state_key: Option<String>,

    /// If true, regenerate the factory state from scratch, ignoring any provided turn_state_key.
    #[arg(long)]
    pub regenerate: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // 1. Load Factory state
    let db_path = PathBuf::from("./mcp_db_tycoon"); // Dedicated DB for tycoon
    let mut factory = if cli.regenerate || cli.turn_state_key.is_none() {
        if cli.regenerate {
            println!("Regenerating factory state from scratch due to --regenerate flag.");
        } else {
            println!("Initializing new factory state (no --turn-state-key provided).");
        }
        
        if let Some(main_path) = &cli.main_program_path {
            println!("Rust Tycoon starting with main program: {:?}", main_path);
        } else {
            println!("Rust Tycoon starting without a main program path (new factory).");
        }
        let mut new_factory = Factory::new(&db_path)?; // Initialize new factory
        new_factory.points = 100; // Give initial points

        if let Some(main_path) = cli.main_program_path {
            let project_root = main_path.parent()
                .context("main_program_path must have a parent directory if provided")?;
            new_factory.ingest_project(project_root)?;
        }
        new_factory
    } else { // cli.turn_state_key is Some and !cli.regenerate
        let state_json = cli.turn_state_key.unwrap();
        // Deserialize the factory state
        // Placeholder for actual deserialization (will be implemented in Phase 2)
        println!("DEBUG: Attempting to load state (placeholder): {}", state_json);
        Factory::new(&db_path)? // Initialize new for now, actual deserialization will replace this
    };
    
    // 2. Execute command
    let available_tools: Vec<Arc<dyn FactoryBlock>> = get_available_tools();
    // Determine the current_crate_path for tool execution
    let current_crate_path = cli.main_program_path.clone().unwrap_or_else(|| PathBuf::from(".")); 


    match cli.cmd.as_str() {
        "buy-tool" => {
            let tool_name = cli.tool_name.context("Tool name is required for 'buy-tool' command.")?;
            if let Some(chosen_tool) = available_tools.iter().find(|tool| tool.name() == tool_name) {
                if factory.points >= chosen_tool.cost() {
                    factory.points -= chosen_tool.cost();
                    factory.bought_tools.push(chosen_tool.clone());
                    println!("Bought a {}! Remaining points: {}", chosen_tool.name(), factory.points);
                } else {
                    println!("Not enough points to buy {}. Need {} points.", chosen_tool.name(), chosen_tool.cost());
                }
            } else {
                println!("Tool '{}' not found in available tools.", tool_name);
            }
        },
        "use-tool" => {
            let tool_name = cli.tool_name.context("Tool name is required for 'use-tool' command.")?;
            if let Some(chosen_tool) = factory.bought_tools.iter().find(|tool| tool.name() == tool_name) {
                println!("Using {}...", chosen_tool.name());
                
                // Special handling for some tools that modify 'current_crate_path'
                // This logic might need further refinement or abstraction.
                let target_crate_path = if chosen_tool.name() == "Rust Diagram Flake V1" || chosen_tool.name() == "Self-Refactor (Factory V2 Quine)" {
                    PathBuf::from("crates/rusttycoon/src/factory.rs")
                } else {
                    current_crate_path.clone()
                };

                chosen_tool.execute(&mut factory, &target_crate_path)?;
            } else {
                println!("Tool '{}' not found among bought tools.", tool_name);
            }
        },
        "view-blueprint" => {
            let mermaid_diagram = factory.render_factory_floor();
            println!("\n--- Factory Floor (Mermaid Diagram) ---");
            println!("{}", mermaid_diagram);
            println!("---------------------------------------\n");
        },
        "exit" => {
            println!("Exiting Rust Tycoon.");
        },
        _ => {
            println!("Unknown command: {}", cli.cmd);
        }
    }

    // 3. Emit updated Factory state
    if let Some(emit_key) = cli.emit_turn_state_key {
        // Serialize the factory state
        // Placeholder for actual serialization (will be implemented in Phase 2)
        println!("{}: {}", emit_key, serde_json::to_string(&factory).unwrap_or_else(|_| "{}".to_string())); // Temporarily use an empty JSON
    }

    Ok(())
}