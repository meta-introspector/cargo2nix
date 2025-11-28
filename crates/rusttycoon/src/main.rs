use clap::Parser;
use std::path::PathBuf;
use anyhow::{Result, Context}; // Add anyhow for error handling
use std::io; // Added for stdin().read_line
use std::sync::Arc; // Added for Arc

mod factory; // Declare the new factory module
mod factory_blocks; // Declare the new factory_blocks module
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
    let available_tools: Vec<Arc<dyn factory::FactoryBlock>> = factory::get_available_tools();

    // Main interactive game loop
    loop {
        println!("\n--- Rust Tycoon Menu ---");
        println!("Current Points: {}", factory.points);
        println!("1. Buy a new tool");
        println!("2. Use an already bought tool");
        println!("3. View Factory Floor (Mermaid Diagram)");
        println!("4. Ask LLM for suggestions (Placeholder)");
        println!("0. Exit");

        print!("Enter your choice: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).context("Failed to read line from stdin")?;
        let choice: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => { // Buy a new tool
                println!("\n--- Available Tools ---");
                for (i, tool) in available_tools.iter().enumerate() {
                    println!("{}. {} (Cost: {} points)", i + 1, tool.name(), tool.cost());
                }
                print!("Enter the number of the tool you want to buy, or '0' to go back: ");
                let mut tool_input = String::new();
                io::stdin().read_line(&mut tool_input).context("Failed to read line from stdin")?;
                let tool_choice: usize = match tool_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a number.");
                        continue;
                    }
                };

                if tool_choice == 0 {
                    continue; // Go back to main menu
                }

                if tool_choice > 0 && tool_choice <= available_tools.len() {
                    let chosen_tool = available_tools[tool_choice - 1].clone(); // Clone the Arc
                    if factory.points >= chosen_tool.cost() {
                        factory.points -= chosen_tool.cost();
                        factory.bought_tools.push(chosen_tool.clone());
                        println!("You bought a {}! Remaining points: {}", chosen_tool.name(), factory.points);
                    } else {
                        println!("Not enough points to buy {}. You need {} points.", chosen_tool.name(), chosen_tool.cost());
                    }
                } else {
                    println!("Tool choice out of range. Please try again.");
                }
            },
            2 => { // Use an already bought tool
                if factory.bought_tools.is_empty() {
                    println!("You haven't bought any tools yet.");
                    continue;
                }
                println!("\n--- Your Bought Tools ---");
                for (i, tool) in factory.bought_tools.iter().enumerate() {
                    println!("{}. {}", i + 1, tool.name());
                }
                print!("Enter the number of the tool you want to use, or '0' to go back: ");
                let mut tool_input = String::new();
                io::stdin().read_line(&mut tool_input).context("Failed to read line from stdin")?;
                let tool_choice: usize = match tool_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a number.");
                        continue;
                    }
                };

                if tool_choice == 0 {
                    continue; // Go back to main menu
                }

                if tool_choice > 0 && tool_choice <= factory.bought_tools.len() {
                    let chosen_tool = factory.bought_tools[tool_choice - 1].clone(); // Clone the Arc
                    println!("Using {}...", chosen_tool.name());
                    
                    let target_crate_path = if chosen_tool.name() == "Rust Diagram Flake V1" || chosen_tool.name() == "Self-Refactor (Factory V2 Quine)" {
                        PathBuf::from("crates/rusttycoon/src/factory.rs")
                    } else {
                        cli.main_program_path.clone()
                    };

                    chosen_tool.execute(&mut factory, &target_crate_path)?;
                } else {
                    println!("Tool choice out of range. Please try again.");
                }
            },
            3 => { // View Factory Floor
                let mermaid_diagram = factory.render_factory_floor();
                println!("\n--- Factory Floor (Mermaid Diagram) ---");
                println!("{}", mermaid_diagram);
                println!("---------------------------------------\n");
            },
            4 => { // Ask LLM for suggestions
                println!("LLM is thinking... (Not yet implemented)");
                // Future: Implement LLM interaction logic here
            },
            0 => { // Exit
                println!("Exiting Rust Tycoon. Goodbye!");
                break;
            },
            _ => {
                println!("Invalid choice. Please enter a valid option.");
            }
        }
    }

    Ok(())
}
