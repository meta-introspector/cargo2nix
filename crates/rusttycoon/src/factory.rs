use anyhow::{Result, Context};
use std::path::{PathBuf, Path};
use rocksdb::{DB, Options};
use rust_mcp_server::file_ingestion::scan_and_ingest_project;
use std::process::Command; // Added
use std::fs; // Added
use chrono::Local; // Added for timestamps
use std::sync::Arc; // Add Arc for shared ownership

use crate::factory_blocks::core_infra_blocks;
use crate::factory_blocks::code_intel_blocks;
//use crate::factory_blocks::rustc_meta_blocks;
use crate::factory_blocks::solana_platform_blocks;
use crate::factory_blocks::system_trace_blocks;
use crate::factory_blocks::media_workflow_blocks;
use crate::factory_blocks::ai_llm_blocks;
use crate::factory_blocks::math_crypto_blocks;
use crate::factory_blocks::automorphic_blocks;
use crate::factory_blocks::meme_comm_blocks;
//use crate::factory_blocks::godel_golem_blocks;
//use crate::factory_blocks::dynamic_blocks;
use crate::factory_blocks::transform_blocks;
//use crate::factory_blocks::flake_importer_exporter_blocks;







pub fn get_available_tools() -> Vec<Arc<dyn FactoryBlock>> {
    vec![
        Arc::new(core_infra_blocks::ConveyerBeltBlock),
        Arc::new(core_infra_blocks::RobotArmBlock),
        Arc::new(core_infra_blocks::CodeEvaluatorBlock),
        Arc::new(code_intel_blocks::SynBlock),
        Arc::new(code_intel_blocks::UseResolverBlock),
        Arc::new(code_intel_blocks::DeclSplitterBlock),
        Arc::new(core_infra_blocks::ReadFileBlock),
        Arc::new(core_infra_blocks::RocksDBBlock),
        Arc::new(core_infra_blocks::GitBlock),
        Arc::new(code_intel_blocks::LLVMBlock),
//        Arc::new(rustc_meta_blocks::RustcBlock), // Original RustcBlock
        Arc::new(code_intel_blocks::RustcCompileBlock), // New RustcCompileBlock
//        Arc::new(rustc_meta_blocks::Rust71PartsBuilderBlock),
        Arc::new(solana_platform_blocks::SolanaRustcIngestBlock),
        Arc::new(solana_platform_blocks::SolanaRustcMonsterProveBlock),
        Arc::new(solana_platform_blocks::SolanaRustcLevel10Block),
        Arc::new(code_intel_blocks::FeatureDiagnosticBlock),
        Arc::new(core_infra_blocks::CargoBlock),
        Arc::new(core_infra_blocks::CrateScannerBlock),
        Arc::new(core_infra_blocks::RustToolchainIntegratorBlock),
        Arc::new(system_trace_blocks::TcpdumpBlock),
        Arc::new(system_trace_blocks::EbpfBlock),
        Arc::new(system_trace_blocks::StraceBlock),
        Arc::new(system_trace_blocks::PtraceBlock),
        Arc::new(media_workflow_blocks::MermaidIntegrationBlock),
        Arc::new(core_infra_blocks::HttpServerBlock),
        Arc::new(core_infra_blocks::RenderingServerBlock),
        // Arc::new(media_workflow_blocks::LaTeXProcessorBlock),
        Arc::new(ai_llm_blocks::HuggingFaceBlock),
        // Arc::new(ai_llm_blocks::DatasetBlock),

        Arc::new(ai_llm_blocks::OllamaIntegrationBlock),
        Arc::new(ai_llm_blocks::OpenRouterBlock),
        Arc::new(core_infra_blocks::KeyVaultBlock),
        Arc::new(core_infra_blocks::AWSParameterStoreBlock),
        // Arc::new(media_workflow_blocks::ImageGeneratorBlock),
        // Arc::new(media_workflow_blocks::AudioGeneratorBlock),
        // Arc::new(media_workflow_blocks::VideoGeneratorBlock),
        // Arc::new(media_workflow_blocks::InvokeAIConversionBlock),
        Arc::new(ai_llm_blocks::NotebookLMBlock),
        Arc::new(ai_llm_blocks::SoraBlock),
        Arc::new(ai_llm_blocks::GroqImagineBlock),
        // Arc::new(media_workflow_blocks::ReportGeneratorBlock),
        // Arc::new(media_workflow_blocks::AutomatedReportVideoAudioWorkflowBlock),
//        Arc::new(rustc_meta_blocks::RustcCrateBlock::new("default_rustc_crate")),
        Arc::new(automorphic_blocks::SolanaRustcTycoonFactoryBuilderBlock),
        Arc::new(ai_llm_blocks::LlmBlock),
        Arc::new(ai_llm_blocks::LLMProofReviewBlock),
        // Arc::new(Lean4Block),
        // Arc::new(MiniZincBlock),
        // Arc::new(LspBlock),
        // Arc::new(McpBlock),
        Arc::new(math_crypto_blocks::LmfdbBlock),
        Arc::new(code_intel_blocks::WikidataBlock),

        Arc::new(media_workflow_blocks::ArchiveOrgBlock),
        Arc::new(meme_comm_blocks::EmojicodeLoaderBlock),
//        Arc::new(automorphic_blocks::AutomorphicLoopBlock), // The One Ring
        Arc::new(meme_comm_blocks::MemelordBotBlock),
        Arc::new(meme_comm_blocks::ShillBotBlock),
        Arc::new(meme_comm_blocks::TwitterInputBlock),
        Arc::new(meme_comm_blocks::TelegramInputBlock),
        Arc::new(meme_comm_blocks::DiscordInputBlock),
//        Arc::new(godel_golem_blocks::GodelGolemBotBlock),
//        Arc::new(dynamic_blocks::DynamicBlock),
//        Arc::new(dynamic_blocks::FunctionalBlock),
        Arc::new(automorphic_blocks::RustCombinatorBlock),
        Arc::new(math_crypto_blocks::Lean4MathlibBlock),
        Arc::new(math_crypto_blocks::ConwayMonsterProofBlock),
        Arc::new(math_crypto_blocks::RaoulBottBlock),
        // Arc::new(dynamic_blocks::QuasifiberBlock), // Quasifiber Reducer
        // Arc::new(dynamic_blocks::ExpandToLayerBlock), // Layer Expander
        Arc::new(math_crypto_blocks::ZKPMapperBlock),
        Arc::new(math_crypto_blocks::HeckeOperatorBlock),
        Arc::new(transform_blocks::UseBlock),
        Arc::new(math_crypto_blocks::R1CSBlock),
        Arc::new(core_infra_blocks::RedstoneLayerBlock),
        Arc::new(transform_blocks::CrateDecomposerBlock),
        Arc::new(code_intel_blocks::HasherBlock),
        Arc::new(code_intel_blocks::PetgraphBlock),
        Arc::new(code_intel_blocks::GraphEigenvectorBlock),
        Arc::new(code_intel_blocks::TopologicalSortBlock),
        Arc::new(code_intel_blocks::NumericalTransformBlock),
        Arc::new(automorphic_blocks::RustDiagramFlakeV1Block),
//        Arc::new(flake_importer_exporter_blocks::FlakeLockImporterBlock),
        Arc::new(code_intel_blocks::TraitFeatureExtractorBlock),
        Arc::new(solana_platform_blocks::SolanaSealevelLayerBlock),
        Arc::new(solana_platform_blocks::SolanaValidatorTycoonBlock),
        Arc::new(solana_platform_blocks::RustcToSolanaLoaderBlock),
        Arc::new(core_infra_blocks::LibP2PBlock),
        Arc::new(core_infra_blocks::IPFSBlock),
//        Arc::new(flake_importer_exporter_blocks::CrateExporterBlock),
        Arc::new(core_infra_blocks::NixDevelopBlock),
Arc::new(core_infra_blocks::MakeTargetBlock),
        Arc::new(code_intel_blocks::RustSrcIngestBlock),
        Arc::new(core_infra_blocks::DirectoryMappingBlock),
        Arc::new(code_intel_blocks::IdeaGeneratorBlock),
        Arc::new(code_intel_blocks::BinaryCatalogBlock),
//        Arc::new(code_intel_blocks::TaskCatalogBlock),
        Arc::new(code_intel_blocks::CodeConceptMapperBlock),
        Arc::new(automorphic_blocks::AutomorphicOrbitReflectorBlock),
        Arc::new(automorphic_blocks::SelfRefactorBlock),
    ]
}



#[derive(Debug)]
pub enum ProcessingLevel {
    SourceCode,
    SystemCalls,
    NetworkTraffic,
    // Add more levels as needed (e.g., AST, LLVM IR, etc.)
}

pub trait FactoryBlock { // Removed Clone bound
    fn name(&self) -> &'static str;
    fn cost(&self) -> u32;
    // New: Execute method for the factory block
    fn execute(&self, factory: &mut Factory, current_crate_path: &PathBuf) -> Result<()> {
        println!("{} block executed. (No specific action implemented yet)", self.name());
        Ok(())
    }
}

pub struct Factory {
    db: DB,
    pub points: u32, // Points to "pay" for tools
    pub bought_tools: Vec<Arc<dyn FactoryBlock>>, // Now stores Arc<dyn FactoryBlock>
    pub discovered_crates: Vec<PathBuf>, // Tracks crates discovered by the scanner
    pub processing_crates: Vec<(PathBuf, ProcessingLevel)>, // Crates currently being processed at a specific level
    pub generated_assets: Vec<PathBuf>, // Stores paths to generated image assets (SVG, PNG, etc.)
    // Other factory components will go here
}

    impl Factory {
    pub fn new(db_path: &Path) -> Result<Self> {
        let mut db_options = Options::default();
        db_options.create_if_missing(true);
        let db = DB::open(&db_options, db_path)
            .context(format!("Failed to open RocksDB at {:?}", db_path))?;
        Ok(Self { db, points: 0, bought_tools: Vec::new(), discovered_crates: Vec::new(), processing_crates: Vec::new(), generated_assets: Vec::new() })
    }
    pub fn ingest_project(&self, project_root: &Path) -> Result<()> {
        eprintln!("Factory: Ingesting project from {:?}", project_root);
        scan_and_ingest_project(&self.db, project_root)?;
        eprintln!("Factory: Project ingestion complete.");
        Ok(())
    }

    pub fn evaluate_code(&mut self, code_description: &str) -> Result<()> {
        eprintln!("Factory: Evaluating code related to: {}", code_description);
        let earned_points = 50; // Arbitrary points for evaluation
        self.points += earned_points;
        eprintln!("Factory: Earned {} points for evaluation! Total points: {}", earned_points, self.points);
        Ok(())
    }

    pub fn render_factory_floor(&self) -> String {
        let mut mermaid_string = String::new();
        mermaid_string.push_str("graph TD\n"); // Top-Down graph

        // Start node: The initial ingested crate
        mermaid_string.push_str(&format!("  A[Main Program: {:?}]\n", "rustc main.rs".to_string())); // Placeholder for actual main_program_path

        // Group tools into subgraphs (floors)
        let mut math_floor_tools = Vec::new();
        let mut meme_floor_tools = Vec::new();
        let mut redstone_floor_tools = Vec::new(); // New Redstone floor
        let mut schiztomath_floor_tools = Vec::new(); // New Schiztomath floor
        let mut core_floor_tools = Vec::new(); // Renamed from main_floor_tools

        for tool in self.bought_tools.iter() {
            match tool.name() {
                // Math Floor Tools
                "Lean 4 Mathlib" | "MiniZinc Solver" | "Raoul Bott (8-fold Periodicity)" | "LMFDB Integrator" | "Lean 4 Theorem Prover" => math_floor_tools.push(tool),
                // Meme Floor Tools
                "Meme Lord Bot" | "Emojicode Loader" => meme_floor_tools.push(tool),
                // Redstone Floor Tools
                "Redstone Layer" | "Crate Decomposer (Redstone/Scratch)" => redstone_floor_tools.push(tool),
                // Schiztomath Floor Tools
                "Gödel Golem Bot" => schiztomath_floor_tools.push(tool),
                // Core/Infrastructure Floor Tools
                _ => core_floor_tools.push(tool),
            }
        }

        // Render Core Floor
        mermaid_string.push_str("  subgraph Core Floor\n");
        for (i, tool) in core_floor_tools.iter().enumerate() {
            mermaid_string.push_str(&format!("    CF{}[Tool: {} - Cost: {}]\n", i, tool.name(), tool.cost()));
            mermaid_string.push_str(&format!("    A --> CF{}\n", i)); // Link to Main Program
        }
        mermaid_string.push_str("  end\n");

        // Render Math Floor
        if !math_floor_tools.is_empty() {
            mermaid_string.push_str("  subgraph Math Floor\n");
            if math_floor_tools.iter().any(|tool| tool.name() == "Raoul Bott (8-fold Periodicity)") {
                mermaid_string.push_str("    subgraph 8th Level (Bott Periodicity)\n");
                mermaid_string.push_str("      RB((Raoul Bott)) -- 8-fold Periodic -- P8[[Structure]]\n"); // Circular node for Bott, special shape for Structure
                mermaid_string.push_str("      style RB fill:#afa,stroke:#333,stroke-width:2px,stroke-dasharray: 5 5\n"); // Styled as a dot/curve hint
                mermaid_string.push_str("    end\n");
            }
            for (i, tool) in math_floor_tools.iter().enumerate() {
                mermaid_string.push_str(&format!("    MATH_T{}[Tool: {} - Cost: {}]\n", i, tool.name(), tool.cost()));
                // Link from a Core floor tool to Math Floor
                if !core_floor_tools.is_empty() {
                    mermaid_string.push_str(&format!("    CF0 --> MATH_T{}\n", i)); 
                }
            }
            mermaid_string.push_str("  end\n");
        }
        
        // Render Meme Floor
        if !meme_floor_tools.is_empty() {
            mermaid_string.push_str("  subgraph Meme Floor\n");
            for (i, tool) in meme_floor_tools.iter().enumerate() {
                mermaid_string.push_str(&format!("    MEME_T{}[Tool: {} - Cost: {}]\n", i, tool.name(), tool.cost()));
                // Link from a Core floor tool to Meme Floor
                if !core_floor_tools.is_empty() {
                    mermaid_string.push_str(&format!("    CF1 --> MEME_T{}\n", i)); 
                }
            }
            mermaid_string.push_str("  end\n");
        }

        // Render Redstone Floor
        if !redstone_floor_tools.is_empty() {
            mermaid_string.push_str("  subgraph Redstone Floor\n");
            for (i, tool) in redstone_floor_tools.iter().enumerate() {
                mermaid_string.push_str(&format!("    RED_T{}[Tool: {} - Cost: {}]\n", i, tool.name(), tool.cost()));
                // Link from a Core floor tool to Redstone Floor
                if !core_floor_tools.is_empty() {
                    mermaid_string.push_str(&format!("    CF2 --> RED_T{}\n", i)); 
                }
            }
            mermaid_string.push_str("  end\n");
        }

        // Render Schiztomath Floor (Bridging Layer)
        if !schiztomath_floor_tools.is_empty() {
            mermaid_string.push_str("  subgraph Schiztomath Floor (Imaginary Layer)\n");
            for (i, tool) in schiztomath_floor_tools.iter().enumerate() {
                mermaid_string.push_str(&format!("    GG{}[Tool: {} - Cost: {}]\n", i, tool.name(), tool.cost()));
                // Link from Core Floor to Schiztomath
                if !core_floor_tools.is_empty() {
                    mermaid_string.push_str(&format!("    CF3 --> GG{}\n", i)); 
                }
                // Bridge connections to Math and Meme Floors
                if !math_floor_tools.is_empty() {
                    mermaid_string.push_str(&format!("    GG{} -- Bridges --> MATH_T0\n", i)); // Link to first Math tool
                }
                if !meme_floor_tools.is_empty() {
                    mermaid_string.push_str(&format!("    GG{} -- Connects --> MEME_T0\n", i)); // Link to first Meme tool
                }
            }
            mermaid_string.push_str("  end\n");
        }


        // Display processing crates at different levels
        for (i, (crate_path, level)) in self.processing_crates.iter().enumerate() {
            mermaid_string.push_str(&format!("  P{}[Crate: {:?} (Level: {:?})]\n", i, crate_path, level));
            // Link processing crates, e.g., from tools or to future stages
            mermaid_string.push_str(&format!("  A --> P{}\n", i)); // Simple linking from main program
        }

        mermaid_string.push_str(&format!("  F{{Factory Points: {}}}\n", self.points));
        if self.bought_tools.iter().any(|tool| tool.name() == "The One Ring (Automorphic Loop)") {
            mermaid_string.push_str("  Goal((The One Ring - Automorphic Loop Achieved!))\n");
            mermaid_string.push_str("  F -- Win! --> Goal\n"); // Link points to goal
            mermaid_string.push_str("  style Goal fill:#ff0,stroke:#f00,stroke-width:4px\n"); // Gold/Red style for the Ring
        }

        mermaid_string.push_str("  style A fill:#f9f,stroke:#333,stroke-width:2px\n"); // Style for start node
        mermaid_string.push_str("  style F fill:#9f9,stroke:#333,stroke-width:2px\n"); // Style for points node

        mermaid_string
    }

    // Other factory blocks (analysis, transformation, etc.) will be added here
}
