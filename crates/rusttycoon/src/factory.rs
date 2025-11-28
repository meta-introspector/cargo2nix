use anyhow::{Result, Context};
use anyhow::{Result, Context}; // Added
use std::path::{PathBuf, Path};
use rocksdb::{DB, Options};
use rust_mcp_server::file_ingestion::scan_and_ingest_project;
use std::process::Command; // Added
use std::fs; // Added
use chrono::Local; // Added for timestamps


#[derive(Clone)] // Add Clone derive
pub struct ConveyerBeltBlock; // Re-inserted
impl FactoryBlock for ConveyerBeltBlock {
    fn name(&self) -> &'static str { "Conveyer Belt" }
    fn cost(&self) -> u32 { 10 }
}

#[derive(Clone)] // Add Clone derive
pub struct RobotArmBlock;
impl FactoryBlock for RobotArmBlock {
    fn name(&self) -> &'static str { "Robot Arm" }
    fn cost(&self) -> u32 { 20 }
}

#[derive(Clone)] // Add Clone derive
pub struct CodeEvaluatorBlock;
impl FactoryBlock for CodeEvaluatorBlock {
    fn name(&self) -> &'static str { "Code Evaluator" }
    fn cost(&self) -> u32 { 50 }
}

#[derive(Clone)] // Add Clone derive
pub struct SynBlock;
impl FactoryBlock for SynBlock {
    fn name(&self) -> &'static str { "Syn Parser" }
    fn cost(&self) -> u32 { 15 }
}

#[derive(Clone)] // Add Clone derive
pub struct ReadFileBlock;
impl FactoryBlock for ReadFileBlock {
    fn name(&self) -> &'static str { "File Reader" }
    fn cost(&self) -> u32 { 5 }
}

#[derive(Clone)] // Add Clone derive
pub struct RocksDBBlock;
impl FactoryBlock for RocksDBBlock {
    fn name(&self) -> &'static str { "RocksDB Integrator" }
    fn cost(&self) -> u32 { 10 }
}

#[derive(Clone)] // Add Clone derive
pub struct GitBlock;
impl FactoryBlock for GitBlock {
    fn name(&self) -> &'static str { "Git Analyzer" }
    fn cost(&self) -> u32 { 25 }
}

#[derive(Clone)] // Add Clone derive
pub struct LLVMBlock;
impl FactoryBlock for LLVMBlock {
    fn name(&self) -> &'static str { "LLVM Backend" }
    fn cost(&self) -> u32 { 75 }
}

#[derive(Clone)] // Add Clone derive
pub struct RustcBlock;
impl FactoryBlock for RustcBlock {
    fn name(&self) -> &'static str { "Rustc Compiler" }
    fn cost(&self) -> u32 { 100 }
}

#[derive(Clone)] // Add Clone derive
pub struct CargoBlock;
impl FactoryBlock for CargoBlock {
    fn name(&self) -> &'static str { "Cargo Manager" }
    fn cost(&self) -> u32 { 30 }
}

#[derive(Clone)] // Add Clone derive
pub struct CrateScannerBlock;
impl FactoryBlock for CrateScannerBlock {
    fn name(&self) -> &'static str { "Crate Scanner" }
    fn cost(&self) -> u32 { 40 }
}

#[derive(Clone)] // Add Clone derive
pub struct RustToolchainIntegratorBlock;
impl FactoryBlock for RustToolchainIntegratorBlock {
    fn name(&self) -> &'static str { "Rust Toolchain Integrator" }
    fn cost(&self) -> u32 { 60 }
}

#[derive(Clone)] // Add Clone derive
pub struct TcpdumpBlock;
impl FactoryBlock for TcpdumpBlock {
    fn name(&self) -> &'static str { "Tcpdump" }
    fn cost(&self) -> u32 { 35 }
}

#[derive(Clone)] // Add Clone derive
pub struct EbpfBlock;
impl FactoryBlock for EbpfBlock {
    fn name(&self) -> &'static str { "eBPF Tracer" }
    fn cost(&self) -> u32 { 80 }
}

#[derive(Clone)] // Add Clone derive
pub struct StraceBlock;
impl FactoryBlock for StraceBlock {
    fn name(&self) -> &'static str { "Strace" }
    fn cost(&self) -> u32 { 20 }
}

#[derive(Clone)] // Add Clone derive
pub struct PtraceBlock;
impl FactoryBlock for PtraceBlock {
    fn name(&self) -> &'static str { "Ptrace" }
    fn cost(&self) -> u32 { 90 }
}

#[derive(Clone)]
pub struct MermaidBlock;
impl FactoryBlock for MermaidBlock {
    fn name(&self) -> &'static str { "Mermaid" }
    fn cost(&self) -> u32 { 10 }
}

#[derive(Clone)]
pub struct HttpServerBlock;
impl FactoryBlock for HttpServerBlock {
    fn name(&self) -> &'static str { "HTTP Server" }
    fn cost(&self) -> u32 { 50 }
}

#[derive(Clone)]
pub struct RenderingServerBlock;
impl FactoryBlock for RenderingServerBlock {
    fn name(&self) -> &'static str { "Rendering Server" }
    fn cost(&self) -> u32 { 70 }
}

#[derive(Clone)]
pub struct LlmBlock;
impl FactoryBlock for LlmBlock {
    fn name(&self) -> &'static str { "LLM" }
    fn cost(&self) -> u32 { 150 }
}

#[derive(Clone)]
pub struct Lean4Block;
impl FactoryBlock for Lean4Block {
    fn name(&self) -> &'static str { "Lean 4 Theorem Prover" }
    fn cost(&self) -> u32 { 200 }
}

#[derive(Clone)]
pub struct MiniZincBlock;
impl FactoryBlock for MiniZincBlock {
    fn name(&self) -> &'static str { "MiniZinc Solver" }
    fn cost(&self) -> u32 { 80 }
}

#[derive(Clone)]
pub struct LspBlock;
impl FactoryBlock for LspBlock {
    fn name(&self) -> &'static str { "LSP Server" }
    fn cost(&self) -> u32 { 60 }
}

#[derive(Clone)]
pub struct McpBlock;
impl FactoryBlock for McpBlock {
    fn name(&self) -> &'static str { "MCP Server" }
    fn cost(&self) -> u32 { 90 }
}

#[derive(Clone)]
pub struct MermaidIntegrationBlock;
impl FactoryBlock for MermaidIntegrationBlock {
    fn name(&self) -> &'static str { "Mermaid Integration" }
    fn cost(&self) -> u32 { 10 }
    fn execute(&self, factory: &mut Factory, _current_crate_path: &PathBuf) -> Result<()> {
        println!("Mermaid Integration block executed. Generating Mermaid diagram and images.");
        let mermaid_diagram = factory.render_factory_floor();
        let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
        let output_dir = PathBuf::from("./generated_assets");
        fs::create_dir_all(&output_dir)?;

        let mmd_file_path = output_dir.join(format!("factory_floor_{}.mmd", timestamp));
        fs::write(&mmd_file_path, &mermaid_diagram)?;
        println!("Mermaid diagram saved to {:?}", mmd_file_path);

        // Generate SVG
        let svg_file_path = output_dir.join(format!("factory_floor_{}.svg", timestamp));
        let svg_output = Command::new("mmdc")
            .arg("-i").arg(&mmd_file_path)
            .arg("-o").arg(&svg_file_path)
            .output()
            .context("Failed to execute mmdc command for SVG. Is mmdc installed and in PATH?")?;
        
        if svg_output.status.success() {
            println!("Generated SVG: {:?}", svg_file_path);
            factory.generated_assets.push(svg_file_path);
        } else {
            eprintln!("Failed to generate SVG: {}", String::from_utf8_lossy(&svg_output.stderr));
        }

        // Generate PNG
        let png_file_path = output_dir.join(format!("factory_floor_{}.png", timestamp));
        let png_output = Command::new("mmdc")
            .arg("-i").arg(&mmd_file_path)
            .arg("-o").arg(&png_file_path)
            .output()
            .context("Failed to execute mmdc command for PNG. Is mmdc installed and in PATH?")?;

        if png_output.status.success() {
            println!("Generated PNG: {:?}", png_file_path);
            factory.generated_assets.push(png_file_path);
        } else {
            eprintln!("Failed to generate PNG: {}", String::from_utf8_lossy(&png_output.stderr));
        }

        // Clean up temporary .mmd file
        fs::remove_file(&mmd_file_path)?;

        Ok(())
    }
}


#[derive(Clone)]
pub struct HttpServerBlock;
impl FactoryBlock for HttpServerBlock {
    fn name(&self) -> &'static str { "HTTP Server" }
    fn cost(&self) -> u32 { 50 }
}

#[derive(Clone)]
pub struct RenderingServerBlock;
impl FactoryBlock for RenderingServerBlock {
    fn name(&self) -> &'static str { "Rendering Server" }
    fn cost(&self) -> u32 { 70 }
}

pub fn get_available_tools() -> Vec<Box<dyn FactoryBlock>> {
    vec![
        Box::new(ConveyerBeltBlock),
        Box::new(RobotArmBlock),
        Box::new(CodeEvaluatorBlock),
        Box::new(SynBlock),
        Box::new(ReadFileBlock),
        Box::new(RocksDBBlock),
        Box::new(GitBlock),
        Box::new(LLVMBlock),
        Box::new(RustcBlock),
        Box::new(CargoBlock),
        Box::new(CrateScannerBlock),
        Box::new(RustToolchainIntegratorBlock),
        Box::new(TcpdumpBlock),
        Box::new(EbpfBlock),
        Box::new(StraceBlock),
        Box::new(PtraceBlock),
        Box::new(MermaidIntegrationBlock),
        Box::new(HttpServerBlock),
        Box::new(RenderingServerBlock),
        Box::new(MermaidBlock),
        Box::new(HttpServerBlock),
        Box::new(RenderingServerBlock),
        Box::new(LlmBlock),
        Box::new(Lean4Block),
        Box::new(MiniZincBlock),
        Box::new(LspBlock),
        Box::new(McpBlock),
    ]
}



pub enum ProcessingLevel {
    SourceCode,
    SystemCalls,
    NetworkTraffic,
    // Add more levels as needed (e.g., AST, LLVM IR, etc.)
}

pub trait FactoryBlock: Clone { // Add Clone bound
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
    pub bought_tools: Vec<Box<dyn FactoryBlock>>, // Now stores trait objects
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

        // Display bought tools as nodes
        for (i, tool) in self.bought_tools.iter().enumerate() {
            mermaid_string.push_str(&format!("  T{}[Tool: {} - Cost: {}]\n", i, tool.name(), tool.cost()));
            // Link tools to something, perhaps the main program or a processing stage
            mermaid_string.push_str(&format!("  A --> T{}\n", i));
        }

        // Display processing crates at different levels
        for (i, (crate_path, level)) in self.processing_crates.iter().enumerate() {
            mermaid_string.push_str(&format!("  P{}[Crate: {:?} (Level: {:?})]\n", i, crate_path, level));
            // Link processing crates, e.g., from tools or to future stages
            mermaid_string.push_str(&format!("  T{} --> P{}\n", i % self.bought_tools.len(), i)); // Simple linking for now
        }

        mermaid_string.push_str(&format!("  F{{Factory Points: {}}}\n", self.points));
        mermaid_string.push_str("  style A fill:#f9f,stroke:#333,stroke-width:2px\n"); // Style for start node
        mermaid_string.push_str("  style F fill:#9f9,stroke:#333,stroke-width:2px\n"); // Style for points node

        mermaid_string
    }

    // Other factory blocks (analysis, transformation, etc.) will be added here
}
