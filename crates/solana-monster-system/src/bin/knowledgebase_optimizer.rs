use cargo2nix::{execute_minizinc_with_data, KnowledgebaseEntry, KnowledgebaseFormatter};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut formatter = KnowledgebaseFormatter::new();

    // Sample knowledgebase entries
    formatter.add_entry(KnowledgebaseEntry {
        id: "monster_group_theory".to_string(),
        weight: 0.95,
        dependencies: vec!["group_theory".to_string(), "modular_forms".to_string()],
    });

    formatter.add_entry(KnowledgebaseEntry {
        id: "elliptic_curves".to_string(),
        weight: 0.87,
        dependencies: vec!["algebraic_geometry".to_string()],
    });

    formatter.add_entry(KnowledgebaseEntry {
        id: "constraint_programming".to_string(),
        weight: 0.92,
        dependencies: vec!["optimization".to_string(), "logic".to_string()],
    });

    let minizinc_input = formatter.to_minizinc_input();
    println!("Generated MiniZinc input:\n{}", minizinc_input);

    let model_path = Path::new("./models/knowledgebase_optimization.mzn");
    if model_path.exists() {
        match execute_minizinc_with_data(model_path, &minizinc_input) {
            Ok(solution) => println!("Optimal knowledge placement: {:?}", solution),
            Err(e) => eprintln!("MiniZinc execution failed: {}", e),
        }
    } else {
        println!("MiniZinc model not found at {:?}", model_path);
    }

    Ok(())
}
