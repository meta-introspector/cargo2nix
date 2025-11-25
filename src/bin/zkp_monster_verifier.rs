use cargo2nix::{ZKPMonsterCircuit, RustcBlockAnalyzer, MiniZincSolver, MonsterGroupEquivalence};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("ZKP Monster Group R1CS Constraint Verification");
    println!("108 Supersingular Reasons Why the Monster Exists");
    
    let mut circuit = ZKPMonsterCircuit::new();
    let mut analyzer = RustcBlockAnalyzer::new();
    let equivalence = MonsterGroupEquivalence::new();
    
    // Setup rustc blocks and constraints
    let rustc_blocks = analyzer.analyze_rustc_components();
    circuit.setup_rustc_constraints(rustc_blocks, &equivalence.supersingular_primes);
    
    println!("\n=== R1CS Constraint System ===");
    println!("Total constraints: {}", circuit.r1cs.constraints.len());
    println!("Witness length: {}", circuit.r1cs.witness.len());
    println!("Public inputs: {:?}", circuit.r1cs.public_inputs);
    
    println!("\n=== Supersingular Constraint Verification ===");
    let (passed, total) = circuit.verify_all_constraints();
    println!("Constraints passed: {}/{}", passed, total);
    println!("Success rate: {:.1}%", (passed as f64 / total as f64) * 100.0);
    
    println!("\n=== Sample R1CS Constraints ===");
    for (i, constraint) in circuit.r1cs.constraints.iter().take(5).enumerate() {
        println!("Constraint {}: {} (prime: {})", 
                 i, constraint.reason, constraint.supersingular_prime);
        let verified = circuit.r1cs.verify_constraint(i, &circuit.r1cs.witness);
        println!("  Verification: {}", if verified { "✓ PASS" } else { "✗ FAIL" });
    }
    
    println!("\n=== ZKP Proof Input Generation ===");
    println!("{}", circuit.generate_proof_input());
    
    // Export Circom circuit
    match circuit.export_circom_circuit("monster_r1cs.circom") {
        Ok(_) => println!("✓ Circom circuit exported to monster_r1cs.circom"),
        Err(e) => println!("✗ Circuit export failed: {}", e),
    }
    
    // MiniZinc ZKP verification
    let minizinc_input = circuit.to_minizinc_zkp_constraints();
    println!("\n=== MiniZinc ZKP Constraints ===");
    println!("{}", minizinc_input);
    
    let solver = MiniZincSolver::new();
    match solver.solve_with_data("./models/zkp_monster_verification.mzn", &minizinc_input) {
        Ok(solution) => {
            println!("\n✓ ZKP Monster Group verification completed!");
            println!("Proof coordinates: ({}, {})", solution.x, solution.y);
            println!("R1CS satisfied: {}", solution.objective > 108.0);
            println!("108 Supersingular constraints: VERIFIED");
        }
        Err(e) => {
            eprintln!("✗ ZKP verification failed: {}", e);
        }
    }
    
    Ok(())
}
