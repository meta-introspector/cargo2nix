use crate::knowledgebase_formatter::KnowledgebaseFormatter;
use crate::minizinc_data::{MinizincInput, OptimalSolution};
use std::fs;
use std::path::Path;
use std::process::Command;

pub struct MiniZincSolver {
    solver_path: String,
}

impl MiniZincSolver {
    pub fn new() -> Self {
        Self {
            solver_path: "minizinc".to_string(),
        }
    }

    pub fn execute_knowledgebase_optimization(
        &self,
        formatter: &KnowledgebaseFormatter,
    ) -> Result<OptimalSolution, Box<dyn std::error::Error>> {
        let input_data = formatter.to_minizinc_input();
        let model_path = "./models/knowledgebase_optimization.mzn";

        self.solve_with_data(model_path, &input_data)
    }

    pub fn solve_with_data<P: AsRef<Path>>(
        &self,
        model_path: P,
        input_data: &MinizincInput,
    ) -> Result<OptimalSolution, Box<dyn std::error::Error>> {
        let temp_file = format!("/tmp/minizinc_input_{}.dzn", std::process::id());
        fs::write(&temp_file, input_data.to_string())?;

        let output = Command::new(&self.solver_path)
            .arg("--output-mode")
            .arg("json")
            .arg("--solver")
            .arg("gecode")
            .arg(model_path.as_ref())
            .arg(&temp_file)
            .output()?;

        fs::remove_file(&temp_file).ok();

        if !output.status.success() {
            return Err(format!(
                "MiniZinc solver failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )
            .into());
        }

        let stdout = String::from_utf8(output.stdout)?;
        self.parse_solution(&stdout)
    }

    fn parse_solution(&self, output: &str) -> Result<OptimalSolution, Box<dyn std::error::Error>> {
        for line in output.lines() {
            if line.trim().starts_with('{') {
                return OptimalSolution::from_json(line.trim()).map_err(|e| e.into());
            }
        }
        Err("No valid solution found in MiniZinc output".into())
    }
}
