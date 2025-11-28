use clap::Parser;
use std::path::PathBuf;
use anyhow::{Result, Context}; // Add anyhow for error handling
use std::io; // Added for stdin().read_line
mod factory; // Declare the new factory module
use factory::ProcessingLevel; // Import ProcessingLevel
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
    println!("23. LSP Server (Cost: 60 points) - Provides Language Server Protocol features for IDE integration.");
    println!("24. MCP Server (Cost: 90 points) - Integrates the Meta-Compiler Protocol server for advanced code processing.");
    println!("25. LMFDB Integrator (Cost: 180 points) - Connects to the L-functions and Modular Forms Database.");
    println!("26. Wikidata Explorer (Cost: 70 points) - Explores and leverages structured data from Wikidata.");
    println!("27. OpenStreetMap Mapper (Cost: 45 points) - Integrates geographic data from OpenStreetMap.");
    println!("28. Archive.org Downloader (Cost: 30 points) - Accesses historical data and archives from Archive.org.");
    println!("29. The One Ring (Automorphic Loop) (Cost: 1000 points) - Achieve the ultimate goal of self-compilation!");
    println!("30. Meme Lord Bot (Cost: 120 points) - Deploys an AI bot to influence the factory's narrative.");
    println!("31. Gödel Golem Bot (Cost: 250 points) - Deploys an AI bot to seek inconsistencies and guide verification.");
    println!("32. Lean 4 Mathlib (Cost: 200 points) - Import the Lean 4 formal mathematics library.");
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
                
                // Render the factory floor after buying a tool
                let mermaid_diagram = factory.render_factory_floor();
                println!("\n--- Factory Floor (Mermaid Diagram) ---");
                println!("{}", mermaid_diagram);
                println!("---------------------------------------\n");
                
                // TODO: Implement the specific action for each tool here using 'if let' or 'match'
                if chosen_tool.name() == "Code Evaluator" {
                    factory.evaluate_code(&cli.main_program_path.to_string_lossy())?;
                } else if chosen_tool.name() == "Crate Scanner" {
                    println!("Scanning for crates in the workspace...");
                    // Simulate crate scanning (e.g., by listing workspace members from Cargo.toml)
                    factory.discovered_crates.push(PathBuf::from("crates/rusttycoon"));
                    println!("Discovered crates: {:?}", factory.discovered_crates);
                } else if chosen_tool.name() == "Rust Toolchain Integrator" {
                    println!("Integrating Rust toolchain...");
                    println!("Rust toolchain (rustc, cargo) is now integrated and available.");
                } else if chosen_tool.name() == "Tcpdump" {
                    println!("Tcpdump activated: Network packets related to compilation will now be analyzed.");
                    factory.processing_crates.push((PathBuf::from("tcpdump_output.pcap"), ProcessingLevel::NetworkTraffic));
                    println!("New crate 'tcpdump_output.pcap' added for processing at NetworkTraffic level.");
                } else if chosen_tool.name() == "eBPF Tracer" {
                    println!("eBPF Tracer activated: Deep kernel-level insights are now being collected.");
                    factory.processing_crates.push((PathBuf::from("ebpf_trace.log"), ProcessingLevel::SystemCalls));
                    println!("New crate 'ebpf_trace.log' added for processing at SystemCalls level.");
                } else if chosen_tool.name() == "Strace" {
                    println!("Strace activated: System calls of the compiler process are now being traced.");
                    factory.processing_crates.push((PathBuf::from("strace_output.log"), ProcessingLevel::SystemCalls));
                    println!("New crate 'strace_output.log' added for processing at SystemCalls level.");
                } else if chosen_tool.name() == "Ptrace" {
                    println!("Ptrace activated: Compiler process can now be traced and manipulated for debugging.");
                    factory.processing_crates.push((PathBuf::from("ptrace_debug_info.log"), ProcessingLevel::SystemCalls));
                    println!("New crate 'ptrace_debug_info.log' added for processing at SystemCalls level.");
                } else if chosen_tool.name() == "Mermaid Integration" {
                    // This is handled by the generic execute method on MermaidIntegrationBlock itself.
                    // No special logic needed here.
                } else if chosen_tool.name() == "HTTP Server" {
                    println!("HTTP Server activated: A web interface is now available.");
                    // In a real game, this would start an actual server or manage its state.
                } else if chosen_tool.name() == "Rendering Server" {
                    println!("Rendering Server activated: Advanced visualizations can now be generated.");
                    // This would likely manage external rendering processes or services.
                } else if chosen_tool.name() == "LLM" {
                    println!("LLM activated: Large Language Model capabilities integrated for analysis.");
                } else if chosen_tool.name() == "Lean 4 Theorem Prover" {
                    println!("Lean 4 Theorem Prover activated: Formal verification capabilities are online.");
                } else if chosen_tool.name() == "MiniZinc Solver" {
                    println!("MiniZinc Solver activated: Constraint programming for optimization is available.");
                } else if chosen_tool.name() == "LSP Server" {
                    println!("LSP Server activated: IDE integration features are enabled.");
                } else if chosen_tool.name() == "MCP Server" {
                    println!("MCP Server activated: Meta-Compiler Protocol server is running.");
                } else if chosen_tool.name() == "LMFDB Integrator" {
                    println!("LMFDB Integrator activated: Connecting to L-functions and Modular Forms Database.");
                } else if chosen_tool.name() == "Wikidata Explorer" {
                    println!("Wikidata Explorer activated: Accessing structured knowledge from Wikidata.");
                } else if chosen_tool.name() == "OpenStreetMap Mapper" {
                    println!("OpenStreetMap Mapper activated: Geographic data integration enabled.");
                } else if chosen_tool.name() == "Archive.org Downloader" {
                    println!("Archive.org Downloader activated: Historical data and archives are accessible.");
                }
                
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
