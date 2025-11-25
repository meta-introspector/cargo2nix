use cargo2nix::{VernacularMonsterSolver, VernacularEmbedding, MonsterTarget, MiniZincSolver};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Vernacular Rust → Monster Group Path Solver");
    
    // Input: Vernacular Rust embedding
    let rust_embedding = VernacularEmbedding {
        rust_tokens: vec![
            "fn".to_string(),
            "main".to_string(), 
            "println".to_string(),
            "Result".to_string(),
        ],
        semantic_weight: 0.85,
        embedding_vector: vec![42, 17, 93, 156, 7],
    };
    
    // Output: Target Monster Group element
    let monster_target = MonsterTarget {
        element_id: 196883 - 42, // Specific Monster Group element
        stabilizer_class: 12,
        eigenvalue: 0.92,
    };
    
    let path_solver = VernacularMonsterSolver::new(rust_embedding.clone(), monster_target.clone());
    let minizinc_input = path_solver.to_minizinc_path_problem();
    
    println!("\n=== Input: Vernacular Rust Embedding ===");
    println!("Tokens: {:?}", rust_embedding.rust_tokens);
    println!("Embedding vector: {:?}", rust_embedding.embedding_vector);
    
    println!("\n=== Output: Monster Group Target ===");
    println!("Element ID: {}", monster_target.element_id);
    println!("Stabilizer class: {}", monster_target.stabilizer_class);
    
    println!("\n=== MiniZinc Path Problem ===");
    println!("{}", minizinc_input);
    
    let solver = MiniZincSolver::new();
    match solver.solve_with_data("./models/vernacular_monster_path.mzn", &minizinc_input) {
        Ok(solution) => {
            println!("\n✓ Path found from vernacular to Monster Group!");
            println!("Path: {} → {}", solution.x, solution.y);
            println!("Path cost: {}", solution.objective);
            println!("Transformation successful: {}", solution.objective < 100.0);
        }
        Err(e) => {
            eprintln!("✗ No path found: {}", e);
        }
    }
    
    Ok(())
}
