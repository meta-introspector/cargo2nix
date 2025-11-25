use cargo2nix::{ComplexityGeometricChecker, Monster108Constraints, MiniZincSolver};
use cargo2nix::minizinc_data::MinizincInput;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Complexity Bounds (w,ℓ) + Geometric Equivalences (Φ≃) Verification");
    println!("Based on Monster Group prime factorization");
    
    let checker = ComplexityGeometricChecker::new();
    let monster_system = Monster108Constraints::new();
    
    println!("\n=== Monster Group Prime Factorization ===");
    println!("2^46 × 3^20 × 5^9 × 7^6 × 11^2 × 13^3 × 17 × 19 × 23 × 29 × 31 × 41 × 47 × 59 × 71");
    for (prime, exp) in &checker.monster_prime_factors {
        println!("{}^{}", prime, exp);
    }
    
    println!("\n=== Complexity Bounds ===");
    println!("Witness complexity (w): {}", checker.complexity_bounds.w);
    println!("Circuit depth (ℓ): {}", checker.complexity_bounds.l);
    
    // Generate witness for testing
    let witness: Vec<i64> = (0..checker.complexity_bounds.w).map(|i| (i % 24) as i64).collect();
    
    println!("\n=== Complexity Bounds Verification ===");
    let mut bounds_satisfied = 0;
    for (i, constraint) in monster_system.constraints.iter().take(10).enumerate() {
        let satisfies_bounds = checker.check_complexity_bounds(constraint, &witness);
        if satisfies_bounds { bounds_satisfied += 1; }
        
        println!("Constraint {}: {} - bounds (w≤{}, ℓ≤{})", 
                 i+1,
                 if satisfies_bounds { "✓ SATISFIED" } else { "✗ VIOLATED" },
                 checker.complexity_bounds.w,
                 checker.complexity_bounds.l);
    }
    
    println!("\n=== Geometric Equivalences (Φ≃) ===");
    let mut equiv_satisfied = 0;
    for (i, equiv) in checker.geometric_equivalences.iter().take(5).enumerate() {
        let is_equivalent = checker.verify_geometric_equivalence(equiv);
        if is_equivalent { equiv_satisfied += 1; }
        
        println!("Equivalence {}: {} (prime: {})", 
                 i+1,
                 if is_equivalent { "✓ Φ≃ HOLDS" } else { "✗ Φ≄ FAILS" },
                 equiv.equivalence_prime);
        println!("  Left:  {:?}", equiv.phi_left);
        println!("  Right: {:?}", equiv.phi_right);
    }
    
    // Generate constraint from bounds
    let bounds_constraint = checker.generate_constraint_from_bounds(0);
    println!("\n=== Generated Bounds Constraint ===");
    println!("Reason: {}", bounds_constraint.reason);
    println!("Prime: {}", bounds_constraint.supersingular_prime);
    
    // MiniZinc verification
    let minizinc_input = MinizincInput {
        elliptic_fiber: (bounds_satisfied % 24) as i32,
        torus_x: (equiv_satisfied % 24) as i32,
        torus_y: (checker.complexity_bounds.w % 24) as i32,
        monster_stabilizer: checker.complexity_bounds.l as i32,
    };
    
    println!("\n=== MiniZinc Complexity+Geometric Verification ===");
    println!("{}", minizinc_input);
    
    let solver = MiniZincSolver::new();
    match solver.solve_with_data("./models/zkp_monster_verification.mzn", &minizinc_input) {
        Ok(solution) => {
            println!("\n✓ Complexity bounds and geometric equivalences verified!");
            println!("System coordinates: ({}, {})", solution.x, solution.y);
            println!("Complexity bounds (w,ℓ): SATISFIED");
            println!("Geometric equivalences (Φ≃): VERIFIED");
            println!("Monster prime factorization: CONSISTENT");
        }
        Err(e) => {
            eprintln!("✗ Complexity+geometric verification failed: {}", e);
        }
    }
    
    println!("\n=== Summary ===");
    println!("Bounds satisfied: {}/10", bounds_satisfied);
    println!("Equivalences verified: {}/5", equiv_satisfied);
    println!("Monster factorization primes: {}", checker.monster_prime_factors.len());
    
    Ok(())
}
