use anyhow::{Result, Context};
use std::path::{PathBuf, Path};
use rocksdb::{DB, Options};
use rust_mcp_server::file_ingestion::scan_and_ingest_project;
use std::process::Command; // Added
use std::fs; // Added
use chrono::Local; // Added for timestamps
use std::sync::Arc; // Add Arc for shared ownership
use serde::{Deserialize, Serialize}; // Import Serialize and Deserialize
use std::collections::HashMap; // Add this import


pub mod factory_blocks;

pub fn build_available_tools_map() -> HashMap<String, Arc<dyn FactoryBlock + Send + Sync>> {
    let mut map = HashMap::new();
    map.insert("Conveyer Belt".to_string(), Arc::new(factory_blocks::core_infra_blocks::ConveyerBeltBlock));
    map.insert("Robot Arm".to_string(), Arc::new(factory_blocks::core_infra_blocks::RobotArmBlock));
    map.insert("Code Evaluator".to_string(), Arc::new(factory_blocks::core_infra_blocks::CodeEvaluatorBlock));
    map.insert("Syn Parser".to_string(), Arc::new(factory_blocks::code_intel_blocks::SynBlock));
    map.insert("Use Resolver".to_string(), Arc::new(factory_blocks::code_intel_blocks::UseResolverBlock));
    map.insert("Declaration Splitter".to_string(), Arc::new(factory_blocks::code_intel_blocks::DeclSplitterBlock));
    map.insert("File Reader".to_string(), Arc::new(factory_blocks::core_infra_blocks::ReadFileBlock));
    map.insert("RocksDB Integrator".to_string(), Arc::new(factory_blocks::core_infra_blocks::RocksDBBlock));
    map.insert("Git Analyzer".to_string(), Arc::new(factory_blocks::core_infra_blocks::GitBlock));
    map.insert("LLVM Backend".to_string(), Arc::new(factory_blocks::code_intel_blocks::LLVMBlock));
    map.insert("Rustc Compile Task".to_string(), Arc::new(factory_blocks::code_intel_blocks::RustcCompileBlock));
    map.insert("Solana Rustc Ingester".to_string(), Arc::new(factory_blocks::solana_platform_blocks::SolanaRustcIngestBlock));
    map.insert("Solana Rustc Monster Prover".to_string(), Arc::new(factory_blocks::solana_platform_blocks::SolanaRustcMonsterProveBlock));
    map.insert("Solana Rustc Lvl 10".to_string(), Arc::new(factory_blocks::solana_platform_blocks::SolanaRustcLevel10Block));
    map.insert("Feature Diagnostic".to_string(), Arc::new(factory_blocks::code_intel_blocks::FeatureDiagnosticBlock));
    map.insert("Cargo Manager".to_string(), Arc::new(factory_blocks::core_infra_blocks::CargoBlock));
    map.insert("Crate Scanner".to_string(), Arc::new(factory_blocks::core_infra_blocks::CrateScannerBlock));
    map.insert("Rust Toolchain Integrator".to_string(), Arc::new(factory_blocks::core_infra_blocks::RustToolchainIntegratorBlock));
    map.insert("Tcpdump".to_string(), Arc::new(factory_blocks::system_trace_blocks::TcpdumpBlock));
    map.insert("eBPF Tracer".to_string(), Arc::new(factory_blocks::system_trace_blocks::EbpfBlock));
    map.insert("Strace".to_string(), Arc::new(factory_blocks::system_trace_blocks::StraceBlock));
    map.insert("Ptrace".to_string(), Arc::new(factory_blocks::system_trace_blocks::PtraceBlock));
    map.insert("Mermaid Integration".to_string(), Arc::new(factory_blocks::media_workflow_blocks::MermaidIntegrationBlock));
    map.insert("HTTP Server".to_string(), Arc::new(factory_blocks::core_infra_blocks::HttpServerBlock));
    map.insert("Rendering Server".to_string(), Arc::new(factory_blocks::core_infra_blocks::RenderingServerBlock));
    map.insert("Hugging Face Integrator".to_string(), Arc::new(factory_blocks::ai_llm_blocks::HuggingFaceBlock));
    map.insert("Ollama LLM Integration".to_string(), Arc::new(factory_blocks::ai_llm_blocks::OllamaIntegrationBlock));
    map.insert("OpenRouter LLM Access".to_string(), Arc::new(factory_blocks::ai_llm_blocks::OpenRouterBlock));
    map.insert("Generic Key Vault".to_string(), Arc::new(factory_blocks::core_infra_blocks::KeyVaultBlock));
    map.insert("AWS Parameter Store".to_string(), Arc::new(factory_blocks::core_infra_blocks::AWSParameterStoreBlock));
    map.insert("NotebookLM Integration".to_string(), Arc::new(factory_blocks::ai_llm_blocks::NotebookLMBlock));
    map.insert("Sora Text-to-Video".to_string(), Arc::new(factory_blocks::ai_llm_blocks::SoraBlock));
    map.insert("GroqImage Generator".to_string(), Arc::new(factory_blocks::ai_llm_blocks::GroqImagineBlock));
    map.insert("Solana Rustc Tycoon Factory Builder".to_string(), Arc::new(factory_blocks::automorphic_blocks::SolanaRustcTycoonFactoryBuilderBlock));
    map.insert("LLM".to_string(), Arc::new(factory_blocks::ai_llm_blocks::LlmBlock));
    map.insert("LLM Proof Reviewer".to_string(), Arc::new(factory_blocks::ai_llm_blocks::LLMProofReviewBlock));
    map.insert("LMFDB Integrator".to_string(), Arc::new(factory_blocks::math_crypto_blocks::LmfdbBlock));
    map.insert("Wikidata Explorer".to_string(), Arc::new(factory_blocks::code_intel_blocks::WikidataBlock));
    map.insert("Archive.org Downloader".to_string(), Arc::new(factory_blocks::media_workflow_blocks::ArchiveOrgBlock));
    map.insert("Emojicode Loader".to_string(), Arc::new(factory_blocks::meme_comm_blocks::EmojicodeLoaderBlock));
    map.insert("Meme Lord Bot".to_string(), Arc::new(factory_blocks::meme_comm_blocks::MemelordBotBlock));
    map.insert("Shill Bot".to_string(), Arc::new(factory_blocks::meme_comm_blocks::ShillBotBlock));
    map.insert("Twitter Feed Integrator".to_string(), Arc::new(factory_blocks::meme_comm_blocks::TwitterInputBlock));
    map.insert("Telegram Feed Integrator".to_string(), Arc::new(factory_blocks::meme_comm_blocks::TelegramInputBlock));
    map.insert("Discord Feed Integrator".to_string(), Arc::new(factory_blocks::meme_comm_blocks::DiscordInputBlock));
    map.insert("Rust Combinator (Self-Apply)".to_string(), Arc::new(factory_blocks::automorphic_blocks::RustCombinatorBlock));
    map.insert("Lean 4 Mathlib".to_string(), Arc::new(factory_blocks::math_crypto_blocks::Lean4MathlibBlock));
    map.insert("Conway Monster Proof".to_string(), Arc::new(factory_blocks::math_crypto_blocks::ConwayMonsterProofBlock));
    map.insert("Raoul Bott (8-fold Periodicity)".to_string(), Arc::new(factory_blocks::math_crypto_blocks::RaoulBottBlock));
    map.insert("ZKP Graph Mapper".to_string(), Arc::new(factory_blocks::math_crypto_blocks::ZKPMapperBlock));
    map.insert("Hecke Operator (8-fold)".to_string(), Arc::new(factory_blocks::math_crypto_blocks::HeckeOperatorBlock));
    map.insert("Use Statement Analyzer".to_string(), Arc::new(factory_blocks::transform_blocks::UseBlock));
    map.insert("R1CS (Rank-1 Constraint System)".to_string(), Arc::new(factory_blocks::math_crypto_blocks::R1CSBlock));
    map.insert("Redstone Layer".to_string(), Arc::new(factory_blocks::core_infra_blocks::RedstoneLayerBlock));
    map.insert("Crate Decomposer (Redstone/Scratch)".to_string(), Arc::new(factory_blocks::transform_blocks::CrateDecomposerBlock));
    map.insert("File Hasher".to_string(), Arc::new(factory_blocks::code_intel_blocks::HasherBlock));
    map.insert("Petgraph Analyzer".to_string(), Arc::new(factory_blocks::code_intel_blocks::PetgraphBlock));
    map.insert("Graph Eigenvector Calculator".to_string(), Arc::new(factory_blocks::code_intel_blocks::GraphEigenvectorBlock));
    map.insert("Topological Sorter".to_string(), Arc::new(factory_blocks::code_intel_blocks::TopologicalSortBlock));
    map.insert("Numerical Transformer".to_string(), Arc::new(factory_blocks::code_intel_blocks::NumericalTransformBlock));
    map.insert("Rust Diagram Flake V1".to_string(), Arc::new(factory_blocks::automorphic_blocks::RustDiagramFlakeV1Block));
    map.insert("Trait & Feature Extractor".to_string(), Arc::new(factory_blocks::code_intel_blocks::TraitFeatureExtractorBlock));
    map.insert("Solana Sealevel Layer".to_string(), Arc::new(factory_blocks::solana_platform_blocks::SolanaSealevelLayerBlock));
    map.insert("Solana Validator Tycoon".to_string(), Arc::new(factory_blocks::solana_platform_blocks::SolanaValidatorTycoonBlock));
    map.insert("Rustc to Solana Loader".to_string(), Arc::new(factory_blocks::solana_platform_blocks::RustcToSolanaLoaderBlock));
    map.insert("LibP2P Network".to_string(), Arc::new(factory_blocks::core_infra_blocks::LibP2PBlock));
    map.insert("IPFS Storage".to_string(), Arc::new(factory_blocks::core_infra_blocks::IPFSBlock));
    map.insert("Nix Develop Environment".to_string(), Arc::new(factory_blocks::core_infra_blocks::NixDevelopBlock));
    map.insert("Make Target Executor".to_string(), Arc::new(factory_blocks::core_infra_blocks::MakeTargetBlock));
    map.insert("Rust Source Ingester".to_string(), Arc::new(factory_blocks::code_intel_blocks::RustSrcIngestBlock));
    map.insert("Directory Monster Mapper".to_string(), Arc::new(factory_blocks::core_infra_blocks::DirectoryMappingBlock));
    map.insert("Idea Generator (Code Discovery)".to_string(), Arc::new(factory_blocks::code_intel_blocks::IdeaGeneratorBlock));
    map.insert("Binary Catalog (Documentation)".to_string(), Arc::new(factory_blocks::code_intel_blocks::BinaryCatalogBlock));
    map.insert("Code Concept Mapper".to_string(), Arc::new(factory_blocks::code_intel_blocks::CodeConceptMapperBlock));
    map.insert("Automorphic Orbit Reflector (Level 3)".to_string(), Arc::new(factory_blocks::automorphic_blocks::AutomorphicOrbitReflectorBlock));
    map.insert("Self-Refactor (Factory V2 Quine)".to_string(), Arc::new(factory_blocks::automorphic_blocks::SelfRefactorBlock));
    map.insert("Factory Blueprint Exporter".to_string(), Arc::new(factory_blocks::automorphic_blocks::FactoryBlueprintExporterBlock));
    map
}



#[derive(Debug, Serialize, Deserialize)] // Add Serialize, Deserialize
pub enum ProcessingLevel {
    SourceCode,
    SystemCalls,
    NetworkTraffic,
    // Add more levels as needed (e.g., AST, LLVM IR, etc.)
}

#[typetag::serde(name = "FactoryBlock")] // Add typetag for trait objects
pub trait FactoryBlock: Send + Sync { // Added Send + Sync bounds
    fn name(&self) -> &'static str;
    fn cost(&self) -> u32;
    // New: Execute method for the factory block
    fn execute(&self, factory: &mut Factory, current_crate_path: &PathBuf) -> Result<()> {
        println!("{} block executed. (No specific action implemented yet)", self.name());
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Factory")] // This allows custom deserialization
pub struct Factory {
    #[serde(skip)] // Skip serialization of the DB connection itself
    pub db: DB, // The RocksDB connection.
    pub db_path: PathBuf, // Store the path to re-open the DB

    pub points: u32, // Points to "pay" for tools
    pub bought_tools_names: Vec<String>, // Stores names for serialization
    #[serde(skip)] // Skip serialization of the actual tool objects
    pub bought_tools: Vec<Arc<dyn FactoryBlock + Send + Sync>>, // The actual tool objects, populated after deserialization
    #[serde(skip)] // Skip serialization of the map of available tools
    pub available_tools_map: HashMap<String, Arc<dyn FactoryBlock + Send + Sync>>,

    pub discovered_crates: Vec<PathBuf>, // Tracks crates discovered by the scanner
    pub processing_crates: Vec<(PathBuf, ProcessingLevel)>, // Crates currently being processed at a specific level
    pub generated_assets: Vec<PathBuf>, // Stores paths to generated image assets (SVG, PNG, etc.)
}

    impl Factory {
    pub fn new(db_path: &Path) -> Result<Self> {
        let mut db_options = Options::default();
        db_options.create_if_missing(true);
        let db = DB::open(&db_options, db_path)
            .context(format!("Failed to open RocksDB at {:?}", db_path))?;
        
        let available_tools_map = build_available_tools_map(); // Build map once

        Ok(Self { 
            db, 
            db_path: db_path.to_path_buf(), // Store the path
            points: 0, 
            bought_tools_names: Vec::new(), 
            bought_tools: Vec::new(), // Initialize as empty
            available_tools_map, // Initialize with the built map
            discovered_crates: Vec::new(), 
            processing_crates: Vec::new(), 
            generated_assets: Vec::new() 
        })
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

// Helper struct for deserialization
#[derive(Deserialize)]
struct FactoryHelper {
    db_path: PathBuf,
    points: u32,
    bought_tools_names: Vec<String>,
    discovered_crates: Vec<PathBuf>,
    processing_crates: Vec<(PathBuf, ProcessingLevel)>,
    generated_assets: Vec<PathBuf>,
}

impl<'de> Deserialize<'de> for Factory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let helper = FactoryHelper::deserialize(deserializer)?;

        let db_options = rocksdb::Options::default(); // Re-create options
        let db = DB::open(&db_options, &helper.db_path).map_err(serde::de::Error::custom)?;
        let available_tools_map = build_available_tools_map();

        let mut bought_tools = Vec::new();
        for tool_name in &helper.bought_tools_names {
            if let Some(tool) = available_tools_map.get(tool_name) {
                bought_tools.push(tool.clone());
            } else {
                // Handle error: tool not found
                return Err(serde::de::Error::custom(format!("Tool '{}' not found during deserialization.", tool_name)));
            }
        }

        Ok(Factory {
            db,
            db_path: helper.db_path,
            points: helper.points,
            bought_tools_names: helper.bought_tools_names,
            bought_tools,
            available_tools_map,
            discovered_crates: helper.discovered_crates,
            processing_crates: helper.processing_crates,
            generated_assets: helper.generated_assets,
        })
    }
}

