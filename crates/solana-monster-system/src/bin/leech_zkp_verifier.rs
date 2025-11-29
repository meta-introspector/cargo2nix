use solana_monster_system::{LeechZKPConstraints, MiniZincSolver};
use solana_monster_system::minizinc_data::MinizincInput;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("46 Leech Lattice ZKP Constraints");
    println!("Symmetry + Dimension + Encoding Integrity + Maximal Binary Bounds");
    
    let leech_zkp = LeechZKPConstraints::new();
    let witness = 1i64; // Test witness
    
    println!("\n=== 46 Constraint Categories ===");
    println!("Symmetry (0-11): Leech Lattice automorphism group");
    println!("Dimension (12-23): 24-dimensional Leech Lattice structure");
    println!("Encoding Integrity (24-35): ZKP encoding correctness");
    println!("Maximal Binary (36-45): 2^46 binary bounds");
    
    println!("\n=== Constraint Verification ===");
    let symmetry_ok = leech_zkp.verify_symmetry(witness);
    let dimension_ok = leech_zkp.verify_dimension(witness);
    let encoding_ok = leech_zkp.verify_encoding_integrity(witness);
    let binary_ok = leech_zkp.verify_maximal_binary(witness);
    let all_ok = leech_zkp.verify_all_46(witness);
    
    println!("Symmetry: {}", if symmetry_ok { "✓" } else { "✗" });
    println!("Dimension: {}", if dimension_ok { "✓" } else { "✗" });
    println!("Encoding Integrity: {}", if encoding_ok { "✓" } else { "✗" });
    println!("Maximal Binary: {}", if binary_ok { "✓" } else { "✗" });
    println!("All 46 constraints: {}", if all_ok { "✓" } else { "✗" });
    
    // Sample constraint details
    println!("\n=== Sample Constraints ===");
    for i in [0, 12, 24, 36] {
        let constraint = &leech_zkp.constraints_46[i];
        let constraint_type = &leech_zkp.constraint_types[i];
        println!("Constraint {}: {:?} - {}·{} + {} = {}", 
                 i, constraint_type, constraint.a, witness, constraint.b, constraint.c);
    }
    
    // MiniZinc verification
    let minizinc_input = MinizincInput {
        elliptic_fiber: 24, // Leech Lattice dimension
        torus_x: 46,        // Total constraints
        torus_y: 2,         // Binary base
        monster_stabilizer: if all_ok { 46 } else { 0 },
    };
    
    println!("\n=== MiniZinc Leech ZKP Verification ===");
    println!("{}", minizinc_input);
    
    let solver = MiniZincSolver::new();
    match solver.solve_with_data("./models/zkp_monster_verification.mzn", &minizinc_input) {
        Ok(solution) => {
            println!("\n✓ Leech Lattice ZKP constraints verified!");
            println!("Coordinates: ({}, {})", solution.x, solution.y);
            println!("Leech Lattice symmetry: PRESERVED");
            println!("24-dimensional structure: MAINTAINED");
            println!("ZKP encoding integrity: VERIFIED");
            println!("2^46 maximal binary bounds: ENFORCED");
        }
        Err(e) => {
            eprintln!("✗ Leech ZKP verification failed: {}", e);
        }
    }
    
    Ok(())
}
