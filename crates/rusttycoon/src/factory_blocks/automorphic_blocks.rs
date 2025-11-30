use anyhow::{Result, Context};
use std::path::{PathBuf, Path};
use crate::{Factory, FactoryBlock}; // Correct import for Factory and FactoryBlock trait
use std::process::Command; // Added
use std::fs; // Added
use chrono::Local; // Added for timestamps
use serde_json::Value; // Added for parsing flake.lock
use quote::quote; // Added for Rust code generation
use std::sync::Arc; // Add Arc for shared ownership
use serde::{Deserialize, Serialize}; // Add this import

use crate::factory_blocks::core_infra_blocks::{ReadFileBlock};
use crate::factory_blocks::code_intel_blocks::{HasherBlock, UseResolverBlock, DeclSplitterBlock, PetgraphBlock, GraphEigenvectorBlock, TopologicalSortBlock, NumericalTransformBlock};
use crate::factory_blocks::math_crypto_blocks::{HeckeOperatorBlock, McpBlock};
//use crate::factory_blocks::flake_importer_exporter_blocks::CrateExporterBlock;
///use crate::factory_blocks::rustc_meta_blocks::RustcCrateBlock;
//use crate::factory_blocks::rustc_meta_blocks::RustcBlock;


// #[derive(Clone)]
// pub struct AutomorphicLoopBlock;
// impl FactoryBlock for AutomorphicLoopBlock {
//     fn name(&self) -> &'static str { "The One Ring (Automorphic Loop)" }
//     fn cost(&self) -> u32 { 1000 } // Very high cost for the ultimate goal
//     fn execute(&self, factory: &mut Factory, current_crate_path: &PathBuf) -> Result<()> {
//         println!("The One Ring (Automorphic Loop) has been forged! The compiler now compiles itself, and the strange loop is closed.");
//         println!("You have achieved the ultimate goal of the Rust Tycoon! This is the fixed point on the diagram, the Omen.");

//         // Step 1: Export the current factory state as Rust code and Nix flake
//         println!("\n--- Auto-generating Factory Blueprint ---");
//         CrateExporterBlock.execute(factory, current_crate_path)?;
//         let exported_rust_path = factory.generated_assets.last().context("No exported Rust code found")?.clone();

//         // Step 2: Simulate compilation of the exported Rust code
//         println!("\n--- Compiling Generated Factory Blueprint ---");
//         RustcBlock.execute(factory, &exported_rust_path)?; // Simulate rustc on generated code

//         // Step 3: Conceptually load the compiled factory as a new MCP server
//         println!("\n--- Loading Compiled Factory as MCP Server ---");
//         // In a real scenario, this would dynamically load a .so,
//         // discover its blocks, and add them to the factory.
//         // For now, we simulate adding a new MCP server.
//         let new_mcp_server = McpBlock;
//         factory.bought_tools.push(Arc::new(new_mcp_server.clone())); // Changed to Arc::new
//         println!("New MCP Server (from self-compiled factory) loaded into the graph!");
        
//         // In a real game, this would trigger game win conditions, final scoring, etc.
//         factory.points += 5000; // Massive bonus for achieving the loop
//         Ok(())
//     }
// }

#[derive(Clone, Serialize, Deserialize)] // Add Serialize, Deserialize
#[typetag::serde] // Add typetag
pub struct RustCombinatorBlock;
impl FactoryBlock for RustCombinatorBlock {
    fn name(&self) -> &'static str { "Rust Combinator (Self-Apply)" }
    fn cost(&self) -> u32 { 500 } // High cost for self-referential logic
    fn execute(&self, factory: &mut Factory, _current_crate_path: &PathBuf) -> Result<()> {
        println!("Rust Combinator activated! Rustc is attempting to apply itself. Entering a meta-compilation phase.");
        // This would involve complex logic to simulate rustc applying itself to generated code,
        // potentially interacting with RustcBlock and CodeEvaluatorBlock.
        factory.points += 200;
        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize)] // Add Serialize, Deserialize
#[typetag::serde] // Add typetag
pub struct RustDiagramFlakeV1Block;
impl FactoryBlock for RustDiagramFlakeV1Block {
    fn name(&self) -> &'static str { "Rust Diagram Flake V1" }
    fn cost(&self) -> u32 { 2000 } // Very high cost for composite operation
    fn execute(&self, factory: &mut Factory, current_crate_path: &PathBuf) -> Result<()> {
        println!("\n--- Rust Diagram Flake V1 Activated: Generating Automorphic Group of Math from rustc Source ---");
        println!("Input: {:?}", current_crate_path);

        // Simulate pipeline execution
        ReadFileBlock.execute(factory, current_crate_path)?;
        HasherBlock.execute(factory, current_crate_path)?;
        UseResolverBlock.execute(factory, current_crate_path)?;
        DeclSplitterBlock.execute(factory, current_crate_path)?;
        PetgraphBlock.execute(factory, current_crate_path)?;
        GraphEigenvectorBlock.execute(factory, current_crate_path)?;
        TopologicalSortBlock.execute(factory, current_crate_path)?;
        NumericalTransformBlock.execute(factory, current_crate_path)?;

        // Apply Hecke Operator 8 times
        for i in 1..=8 {
            println!("Applying Hecke Operator (iteration {}/8)...", i);
            HeckeOperatorBlock.execute(factory, current_crate_path)?;
        }

        println!("Output: Automorphic Group of Math (conceptually derived).");
        factory.points += 1000; // Large bonus for completing the pipeline
        println!("--- Rust Diagram Flake V1 Completed! ---");
        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize)] // Add Serialize, Deserialize
#[typetag::serde] // Add typetag
pub struct SolanaRustcTycoonFactoryBuilderBlock;
impl FactoryBlock for SolanaRustcTycoonFactoryBuilderBlock {
    fn name(&self) -> &'static str { "Solana Rustc Tycoon Factory Builder" }
    fn cost(&self) -> u32 { 1500 } // Very high cost for meta-factory creation
    fn execute(&self, factory: &mut Factory, _current_crate_path: &PathBuf) -> Result<()> {
        println!("\n--- Solana Rustc Tycoon Factory Builder Activated! ---");
        println!("Orchestrating the creation of a level 2 meta-factory from rustc crates.");

        // Simulate discovering some rustc internal crates
        let rustc_crates = vec![
            "rustc_codegen_llvm",
            "rustc_ast",
            "rustc_hir",
            "rustc_ty",
            "rustc_mir"
        ];

        println!("Simulating discovery and conversion of rustc internal crates to blocks...");
        let mut temp_bought_tools: Vec<Arc<dyn FactoryBlock>> = Vec::new(); // Changed to Arc<dyn FactoryBlock>
        for crate_name in rustc_crates {
//            let rustc_crate_block = RustcCrateBlock::new(crate_name);
	    //            println!("  - Discovered rustc crate: '{}'", rustc_crate_block.crate_name);
            println!("  - Discovered rustc crate: '{}' FIME", crate_name);
//            temp_bought_tools.push(Arc::new(rustc_crate_block)); // Changed to Arc::new
        }

        // Temporarily add these to the factory's bought_tools for CrateExporterBlock to pick up
        let original_bought_tools = factory.bought_tools.drain(..).collect::<Vec<_>>();
        factory.bought_tools.extend(temp_bought_tools);
        
        println!("\n--- Emitting new 'Solana Rustc Tycoon' factory ---");
        // Use CrateExporterBlock to emit this new factory.
        // The current_crate_path might need to be adjusted if this is for a new project.
        // For simplicity, we'll use a placeholder.
        //CrateExporterBlock.execute(factory, &PathBuf::from("solana_rustc_tycoon_base"))?;
        
        // Restore original bought tools
        factory.bought_tools.drain(..);
        factory.bought_tools.extend(original_bought_tools);


        println!("New 'solana-rustc-tycoon-crate' factory conceptually created at Level 2!");
        factory.points += 750;
        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize)] // Add Serialize, Deserialize
#[typetag::serde] // Add typetag
pub struct AutomorphicOrbitReflectorBlock;
impl FactoryBlock for AutomorphicOrbitReflectorBlock {
    fn name(&self) -> &'static str { "Automorphic Orbit Reflector (Level 3)" }
    fn cost(&self) -> u32 { 1800 } // Higher cost for meta-level reflection
    fn execute(&self, factory: &mut Factory, _current_crate_path: &PathBuf) -> Result<()> {
        println!("Automorphic Orbit Reflector activated! Analyzing identified automorphic orbits and their structures as Level 3 concepts.");
        factory.points += 400;
        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize)] // Add Serialize, Deserialize
#[typetag::serde] // Add typetag
pub struct SelfRefactorBlock;
impl FactoryBlock for SelfRefactorBlock {
    fn name(&self) -> &'static str { "Self-Refactor (Factory V2 Quine)" }
    fn cost(&self) -> u32 { 3000 } // Extremely high cost for self-generation
    fn execute(&self, factory: &mut Factory, _current_crate_path: &PathBuf) -> Result<()> {
        println!("\n--- Self-Refactor Block Activated: Generating Factory V2 (Quine) ---");
        println!("The factory is now processing its own source code to produce a new version of itself.");

        let factory_source_path = PathBuf::from("crates/rusttycoon/src/factory.rs");

        // 1. Analyze the factory's source using RustDiagramFlakeV1Block
        println!("1. Analyzing factory source code with Rust Diagram Flake V1...");
        RustDiagramFlakeV1Block.execute(factory, &factory_source_path)?;

        // 2. Export the analyzed factory as new flakes (representing V2)
        println!("2. Exporting analyzed factory as new flakes (Factory V2)...");
//        CrateExporterBlock.execute(factory, &factory_source_path)?; // Export based on the current factory's bought tools after analysis

        println!("--- Self-Refactor Block Completed! Factory V2 (Quine) conceptually generated. ---");
        factory.points += 2000; // Massive bonus for self-generation
        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize)] // Add Serialize, Deserialize
#[typetag::serde] // Add typetag
pub struct FactoryBlueprintExporterBlock;

impl FactoryBlock for FactoryBlueprintExporterBlock {
    fn name(&self) -> &'static str { "Factory Blueprint Exporter" }
    fn cost(&self) -> u32 { 300 } // Moderate cost for generating a blueprint
    fn execute(&self, factory: &mut Factory, _current_crate_path: &PathBuf) -> Result<()> {
        println!("\n--- Factory Blueprint Exporter Activated! ---");
        println!("Analyzing the current factory state to generate a blueprint.");

        let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
        let output_dir = PathBuf::from("./generated_blueprints");
        fs::create_dir_all(&output_dir)?;

        let blueprint_path = output_dir.join(format!("factory_blueprint_{}.md", timestamp));
        let mut blueprint_content = String::new();

        blueprint_content.push_str(&format!("# Factory Blueprint - Generated on {}\n\n", Local::now().to_string()));
        blueprint_content.push_str("## Current Factory State\n\n");
        blueprint_content.push_str(&format!("- **Total Points:** {}\n", factory.points));
        blueprint_content.push_str(&format!("- **Number of Bought Tools:** {}\n\n", factory.bought_tools.len()));

        blueprint_content.push_str("## Bought Tools (Factory Blocks)\n\n");
        if factory.bought_tools.is_empty() {
            blueprint_content.push_str("No tools have been bought yet.\n");
        } else {
            for (i, tool) in factory.bought_tools.iter().enumerate() {
                blueprint_content.push_str(&format!("{}. {} (Cost: {})\n", i + 1, tool.name(), tool.cost()));
            }
        }
        blueprint_content.push_str("\n");

        blueprint_content.push_str("## Factory Floor (Mermaid Diagram Representation)\n\n");
        blueprint_content.push_str("```mermaid\n");
        blueprint_content.push_str(&factory.render_factory_floor());
        blueprint_content.push_str("```\n");
        
        fs::write(&blueprint_path, &blueprint_content)?;
        println!("Generated Factory Blueprint: {:?}", blueprint_path);
        factory.generated_assets.push(blueprint_path);

        factory.points += 100; // Bonus for generating a blueprint
        Ok(())
    }
}

