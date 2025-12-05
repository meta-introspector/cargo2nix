use cargo2nix::{CompilerAnalyzer, MiniZincSolver};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Trait Extraction and Monster Group Verification");

    let mut analyzer = CompilerAnalyzer::new();
    analyzer.analyze_rust_compiler_blocks();

    println!("\n=== Extracted Trait Types ===");
    for trait_type in analyzer.enumerate_trait_types() {
        println!("- {}", trait_type);
    }

    println!("\n=== Generated Dummy Externals ===");
    for dummy in analyzer.generate_dummy_externals() {
        println!("{}\n", dummy);
    }

    let minizinc_input = analyzer.verify_with_minizinc();
    println!("=== MiniZinc Constraint Input ===");
    println!("{}", minizinc_input);

    let solver = MiniZincSolver::new();
    match solver.solve_with_data("./models/trait_verification.mzn", &minizinc_input) {
        Ok(solution) => {
            println!("\n✓ Trait verification completed!");
            println!("Trait placement: ({}, {})", solution.x, solution.y);
            println!("Satisfiable: {}", solution.objective > 0.0);
        }
        Err(e) => {
            eprintln!("✗ Trait verification failed: {}", e);
        }
    }

    Ok(())
}
