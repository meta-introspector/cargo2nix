use clap::Parser;
use std::path::PathBuf;
use anyhow::{Result, Context}; // Add anyhow for error handling
use std::io; // Added for stdin().read_line
mod factory; // Declare the new factory module
use factory::ProcessingLevel; // Import ProcessingLevel

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to the main Rust program to analyze and build the tycoon factory around.
    #[clap(long)]
    pub main_program_path: PathBuf,
}

fn main() -> Result<()> { // Change main to return Result
    let cli = Cli::parse();
    println!("Rust Tycoon starting with main program: {:?}", cli.main_program_path);

    let db_path = PathBuf::from("./mcp_db_tycoon"); // Dedicated DB for tycoon
    let mut factory = factory::Factory::new(&db_path)?; // Initialize factory (make mutable)
    factory.points = 100; // Give initial points
    
    // First factory block: Ingest the project
    let project_root = cli.main_program_path.parent()
        .context("main_program_path must have a parent directory")?;
    factory.ingest_project(project_root)?;

    println!("\n--- Rust Tycoon: Factory Floor ---");
    println!("A crate (representing your main program) has landed on the factory floor.");
    println!("You need to buy tools to move it or process it.");
    println!("Current Points: {}", factory.points);
    // Get all available tools
    let available_tools = factory::get_available_tools();

    // Simple interactive loop for buying tools
    loop {
        println!("\n--- Available Tools ---");
        for (i, tool) in available_tools.iter().enumerate() {
            println!("{}. {} (Cost: {} points)", i + 1, tool.name(), tool.cost());
        }
        println!("\nEnter the number of the tool you want to buy, or '0' to exit:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)
            .context("Failed to read line from stdin")?;
        let choice: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        if choice == 0 {
            println!("Exiting factory operations for now.");
            break;
        }

        if choice > 0 && choice <= available_tools.len() {
            let chosen_tool = &available_tools[choice - 1];
            if factory.points >= chosen_tool.cost() {
                factory.points -= chosen_tool.cost();
                // Push a clone of the chosen_tool (as a Box<dyn FactoryBlock>)
                factory.bought_tools.push(chosen_tool.clone()); 
                println!("You bought a {}! Remaining points: {}", chosen_tool.name(), factory.points);
                // TODO: Implement the specific action for each tool.
                
                // For now, let's just break after buying
                break; 
            } else {
                println!("Not enough points to buy {}. You need {} points.", chosen_tool.name(), chosen_tool.cost());
            }
        } else {
            println!("Choice out of range. Please try again.");
        }
    }

        if factory.points >= cost {
            factory.points -= cost;
            factory.bought_tools.push(tool_type);
            println!("You bought a {:?}! Remaining points: {}", tool_type, factory.points);
            if let factory::Tool::CodeEvaluator = tool_type {
                factory.evaluate_code(&cli.main_program_path.to_string_lossy())?;
            } else if let factory::Tool::CrateScanner = tool_type {
                println!("Scanning for crates in the workspace...");
                // Simulate crate scanning (e.g., by listing workspace members from Cargo.toml)
                let workspace_root = PathBuf::from("./"); // Assuming current dir is workspace root
                // For now, let's just add the rusttycoon crate itself as a discovered crate
                factory.discovered_crates.push(PathBuf::from("crates/rusttycoon")); 
                println!("Discovered crates: {:?}", factory.discovered_crates);
            } else if let factory::Tool::RustToolchainIntegrator = tool_type {
                println!("Integrating Rust toolchain...");
                // Simulate integration (e.g., by indicating readiness for rustc/cargo operations)
                println!("Rust toolchain (rustc, cargo) is now integrated and available.");
            } else if let factory::Tool::Tcpdump = tool_type {
                println!("Tcpdump activated: Network packets related to compilation will now be analyzed.");
                factory.processing_crates.push((PathBuf::from("tcpdump_output.pcap"), ProcessingLevel::NetworkTraffic));
                println!("New crate 'tcpdump_output.pcap' added for processing at NetworkTraffic level.");
            } else if let factory::Tool::Ebpf = tool_type {
                println!("eBPF Tracer activated: Deep kernel-level insights are now being collected.");
                factory.processing_crates.push((PathBuf::from("ebpf_trace.log"), ProcessingLevel::SystemCalls)); // Using SystemCalls for now, could be a new level
                println!("New crate 'ebpf_trace.log' added for processing at SystemCalls level.");
            } else if let factory::Tool::Strace = tool_type {
                println!("Strace activated: System calls of the compiler process are now being traced.");
                factory.processing_crates.push((PathBuf::from("strace_output.log"), ProcessingLevel::SystemCalls));
                println!("New crate 'strace_output.log' added for processing at SystemCalls level.");
            } else if let factory::Tool::Ptrace = tool_type {
                println!("Ptrace activated: Compiler process can now be traced and manipulated for debugging.");
                factory.processing_crates.push((PathBuf::from("ptrace_debug_info.log"), ProcessingLevel::SystemCalls)); // Using SystemCalls for now
                println!("New crate 'ptrace_debug_info.log' added for processing at SystemCalls level.");
            }
            break; // For this minimal iteration, buy one and exit.
        } else {
            println!("Not enough points to buy {:?}. You need {} points.", tool_type, cost);
        }
    }

    // TODO: More sophisticated factory operations and game logic will follow.

    Ok(())
}
