use cargo2nix::{KnowledgebaseFormatter, KnowledgebaseEntry, MiniZincSolver};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Executing MiniZinc Solver with Monster Group Constraints");
    
    let mut formatter = KnowledgebaseFormatter::new();
    
    // Load sample knowledgebase
    formatter.add_entry(KnowledgebaseEntry {
        id: "monster_group_theory".to_string(),
        weight: 0.95,
        dependencies: vec!["group_theory".to_string(), "modular_forms".to_string()],
    });
    
    formatter.add_entry(KnowledgebaseEntry {
        id: "constraint_programming".to_string(),
        weight: 0.92,
        dependencies: vec!["optimization".to_string()],
    });

    let solver = MiniZincSolver::new();
    
    println!("Formatted knowledgebase input:");
    println!("{}", formatter.generate_dzn());
    
    match solver.execute_knowledgebase_optimization(&formatter) {
        Ok(solution) => {
            println!("\n✓ MiniZinc solver completed successfully!");
            println!("Optimal solution: {:?}", solution);
            println!("Placement coordinates: ({}, {})", solution.x, solution.y);
            println!("Objective value: {}", solution.objective);
        }
        Err(e) => {
            eprintln!("✗ MiniZinc solver execution failed: {}", e);
            std::process::exit(1);
        }
    }
    
    Ok(())
}
