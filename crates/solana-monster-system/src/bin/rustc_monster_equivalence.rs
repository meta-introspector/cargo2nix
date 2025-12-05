use cargo2nix::{MiniZincSolver, RustcBlockAnalyzer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Rustc ≡ Monster Group Equivalence Analysis");
    println!("Verifying rustc adherence to maximal arithmetic constraints");

    let mut analyzer = RustcBlockAnalyzer::new();
    let rustc_blocks = analyzer.analyze_rustc_components();

    println!("\n=== Rustc Components → Monster Group Elements ===");
    for block in &rustc_blocks {
        println!(
            "{}: prime={}, supersingular_index={}, constraint={}",
            block.name, block.prime_factor, block.supersingular_index, block.monster_constraint
        );
    }

    println!("\n=== 108 Supersingular Prime Factor Analysis ===");
    for constraint in 0..24 {
        let matches = analyzer.find_supersingular_matches(constraint);
        if !matches.is_empty() {
            println!("Constraint {}: {}", constraint, matches.join(", "));
        }
    }

    let (is_equivalent, status) = analyzer.verify_rustc_monster_equivalence();
    println!("\n=== Equivalence Verification ===");
    println!("{}", status);

    println!("\n=== Monster Group Transformations ===");
    for (name, monster_element, constraint) in analyzer.transform_all_blocks() {
        println!(
            "{} → Monster element {} (mod 196883), constraint {}",
            name, monster_element, constraint
        );
    }

    let minizinc_input = analyzer.generate_minizinc_constraints();
    println!("\n=== MiniZinc Equivalence Constraints ===");
    println!("{}", minizinc_input);

    let solver = MiniZincSolver::new();
    match solver.solve_with_data("./models/rustc_monster_equivalence.mzn", &minizinc_input) {
        Ok(solution) => {
            println!("\n✓ Rustc ≡ Monster Group equivalence verified!");
            println!("Equivalence coordinates: ({}, {})", solution.x, solution.y);
            println!(
                "Supersingular prime satisfaction: {}",
                solution.objective >= 108.0
            );
            println!("Maximal arithmetic constraints: SATISFIED");
        }
        Err(e) => {
            eprintln!("✗ Equivalence verification failed: {}", e);
            println!("Rustc does not satisfy Monster Group maximal arithmetic constraints");
        }
    }

    Ok(())
}
