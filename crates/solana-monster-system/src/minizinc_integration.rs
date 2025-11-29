use std::process::Command;
use std::path::Path;
use std::fs;
use crate::minizinc_data::{MinizincInput, OptimalSolution};

pub fn execute_minizinc_with_data<P: AsRef<Path>>(
    model_path: P,
    input_data: &MinizincInput,
) -> Result<OptimalSolution, Box<dyn std::error::Error>> {
    let temp_data = format!("/tmp/minizinc_data_{}.dzn", std::process::id());
    fs::write(&temp_data, input_data.to_string())?;
    
    let result = execute_minizinc(model_path, &temp_data);
    fs::remove_file(&temp_data).ok();
    result
}

pub fn execute_minizinc(
    model_path: impl AsRef<Path>,
    data_path: impl AsRef<Path>,
) -> Result<OptimalSolution, Box<dyn std::error::Error>> {
    let output = Command::new("minizinc")
        .arg("--output-mode")
        .arg("json")
        .arg(model_path.as_ref())
        .arg(data_path.as_ref())
        .output()?;

    if !output.status.success() {
        return Err(format!("MiniZinc failed: {}", String::from_utf8_lossy(&output.stderr)).into());
    }

    let stdout = String::from_utf8(output.stdout)?;
    parse_minizinc_output(&stdout)
}

fn parse_minizinc_output(output: &str) -> Result<OptimalSolution, Box<dyn std::error::Error>> {
    for line in output.lines() {
        if line.starts_with('{') {
            return OptimalSolution::from_json(line).map_err(|e| e.into());
        }
    }
    Err("No valid solution found".into())
}
