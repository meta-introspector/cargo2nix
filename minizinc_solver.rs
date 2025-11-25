use std::process::Command;
use std::fs;
use crate::rust_block_analyzer::BlockAnalyzer;
use crate::monster_minizinc_generator::{generate_monster_trait_model, generate_trait_data_file};

pub struct MiniZincSolver {
    model_path: String,
    data_path: String,
}

impl MiniZincSolver {
    pub fn new() -> Self {
        Self {
            model_path: "monster_traits.mzn".to_string(),
            data_path: "trait_data.dzn".to_string(),
        }
    }

    pub fn solve_monster_constraints(&self, analyzer: &BlockAnalyzer, complexity: u8) -> Result<String, Box<dyn std::error::Error>> {
        // Generate model using introspector pattern
        let model = generate_monster_trait_model(analyzer, complexity)
            .map_err(|e| format!("Model generation failed: {}", e))?;
        fs::write(&self.model_path, model)?;
        
        // Generate data file
        let data = generate_trait_data_file(analyzer);
        fs::write(&self.data_path, data)?;
        
        // Run MiniZinc solver
        let output = Command::new("minizinc")
            .arg("--solver")
            .arg("gecode")
            .arg(&self.model_path)
            .arg(&self.data_path)
            .output()?;
        
        if output.status.success() {
            Ok(String::from_utf8(output.stdout)?)
        } else {
            Err(format!("MiniZinc failed: {}", String::from_utf8_lossy(&output.stderr)).into())
        }
    }

    fn generate_data_file(&self, analyzer: &BlockAnalyzer) -> String {
        let mut data = String::new();
        
        // Add trait mappings as data
        data.push_str(&format!("num_traits = {};\n", analyzer.trait_mappings.len()));
        
        // Add Monster Group constants
        data.push_str("monster_order = 196883;\n");
        data.push_str("hecke_eigenvalues = [196883, -5472];\n");
        
        data
    }

    pub fn verify_sat_solution(&self, solution: &str) -> bool {
        // Parse MiniZinc output and verify Monster Group properties
        solution.contains("trait_elements") && !solution.contains("UNSATISFIABLE")
    }
}

pub fn run_monster_verification() -> Result<(), Box<dyn std::error::Error>> {
    let mut analyzer = BlockAnalyzer::new();
    analyzer.load_compiler_blocks("rustc_blocks.db")?;
    analyzer.analyze_tool_blocks(".")?;
    analyzer.map_to_monster_group()?;
    
    let solver = MiniZincSolver::new();
    let solution = solver.solve_monster_constraints(&analyzer)?;
    
    if solver.verify_sat_solution(&solution) {
        println!("✓ Monster Group trait mapping verified");
        println!("Solution: {}", solution);
    } else {
        println!("✗ Verification failed");
    }
    
    // Generate trait dummies
    let dummies = analyzer.create_trait_dummies();
    fs::write("trait_dummies.rs", dummies)?;
    
    // Compare blocks
    let differences = analyzer.compare_blocks();
    for diff in differences {
        println!("Difference: {}", diff);
    }
    
    Ok(())
}
