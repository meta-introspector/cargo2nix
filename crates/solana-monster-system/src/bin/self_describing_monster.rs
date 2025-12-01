use cargo2nix::{MiniZincSolver, SelfDescribingMonster};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Self-Describing Monster Group Code");

    let self_embedding = SelfDescribingMonster::describe_self();
    let self_target = SelfDescribingMonster::target_monster();
    let self_path = SelfDescribingMonster::self_path();

    println!("\n=== Self-Description ===");
    println!("{}", SelfDescribingMonster::generate_self_description());

    println!("\n=== Self-Embedding ===");
    println!("Tokens: {:?}", self_embedding.rust_tokens);
    println!("Weight: {}", self_embedding.semantic_weight);
    println!("Vector: {:?}", self_embedding.embedding_vector);

    println!("\n=== Self-Target ===");
    println!("Element: {} (identity)", self_target.element_id);
    println!("Stabilizer: {}", self_target.stabilizer_class);
    println!("Eigenvalue: {}", self_target.eigenvalue);

    println!("\n=== Self-Path (Identity) ===");
    println!("{}", self_path);

    let solver = MiniZincSolver::new();
    match solver.solve_with_data("./models/vernacular_monster_path.mzn", &self_path) {
        Ok(solution) => {
            println!("\n✓ Self-description verified!");
            println!(
                "Identity path: ({}, {}) → ({}, {})",
                solution.x, solution.y, solution.x, solution.y
            );
            println!("Self-consistency: {}", solution.objective == 0.0);
        }
        Err(e) => {
            println!("Self-verification: {}", e);
        }
    }

    Ok(())
}
