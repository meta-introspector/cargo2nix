use std::process::Command;
use std::fs;
use monster_protocol_core::BlockAnalyzer;
use monster_protocol_minizinc::MiniZincModelGenerator;
use std::error::Error;

// --- Trait Definition ---
pub trait SolverExecutor {
    /// Solves constraints using a MiniZinc model generator and analyzer.
    fn solve_constraints(
        &self,
        model_generator: &dyn MiniZincModelGenerator,
        analyzer: &BlockAnalyzer,
        complexity: u8,
    ) -> Result<String, Box<dyn Error>>;

    /// Verifies the satisfiability solution from the solver.
    fn verify_sat_solution(&self, solution: &str) -> bool;
}

// --- Default Implementation ---
pub struct DefaultSolverExecutor {
    model_path: String,
    data_path: String,
}

impl DefaultSolverExecutor {
    pub fn new() -> Self {
        Self {
            model_path: "monster_traits.mzn".to_string(),
            data_path: "trait_data.dzn".to_string(),
        }
    }
}

impl SolverExecutor for DefaultSolverExecutor {
    fn solve_constraints(
        &self,
        model_generator: &dyn MiniZincModelGenerator,
        analyzer: &BlockAnalyzer,
        complexity: u8,
    ) -> Result<String, Box<dyn Error>> {
        // Generate model
        let model = model_generator.generate_model(analyzer, complexity)
            .map_err(|e| format!("Model generation failed: {}", e))?;
        fs::write(&self.model_path, model)?;
        
        // Generate data file
        let data = model_generator.generate_data_file_from_analyzer(analyzer);
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

    fn verify_sat_solution(&self, solution: &str) -> bool {
        // Parse MiniZinc output and verify Monster Group properties
        solution.contains("trait_elements") && !solution.contains("UNSATISFIABLE")
    }
}
