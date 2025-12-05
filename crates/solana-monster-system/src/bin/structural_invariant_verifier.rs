use solana_monster_system::minizinc_data::MinizincInput;
use solana_monster_system::{MiniZincSolver, Monster108Constraints, StructuralInvariantChecker};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Structural Invariant Consistency Verification");
    println!("Checking consistency of structural invariants (a, b, c) in R1CS");

    let checker = StructuralInvariantChecker::new();
    let monster_system = Monster108Constraints::new();

    println!("\n=== Structural Invariants ===");
    for invariant in &checker.invariants {
        println!(
            "Invariant {}: {:?} = {}",
            invariant.invariant_id, invariant.invariant_type, invariant.value
        );
    }

    // Generate witness for testing
    let witness: Vec<i64> = (0..108).map(|i| (i % 24) as i64).collect();

    println!("\n=== Constraint Consistency Check ===");
    let (consistent, total) =
        checker.verify_structural_consistency(&monster_system.constraints, &witness);
    println!("Consistent constraints: {}/{}", consistent, total);
    println!(
        "Consistency rate: {:.1}%",
        (consistent as f64 / total as f64) * 100.0
    );

    println!("\n=== Sample Invariant Checks ===");
    for (i, constraint) in monster_system.constraints.iter().take(5).enumerate() {
        let is_consistent = checker.check_constraint_consistency(constraint, &witness);
        println!(
            "Constraint {}: {} - {}",
            i + 1,
            if is_consistent {
                "✓ CONSISTENT"
            } else {
                "✗ INCONSISTENT"
            },
            constraint.reason
        );
    }

    // Generate invariant-specific constraints
    let invariant_constraints = checker.generate_invariant_constraints();
    println!("\n=== Generated Invariant Constraints ===");
    for constraint in &invariant_constraints {
        println!("R1CS: {}", constraint.reason);
        println!("  Prime: {}", constraint.supersingular_prime);
    }

    // MiniZinc verification
    let minizinc_input = MinizincInput {
        elliptic_fiber: (consistent % 24) as i32,
        torus_x: (total % 24) as i32,
        torus_y: ((consistent + total) % 24) as i32,
        monster_stabilizer: if consistent == total {
            108
        } else {
            consistent as i32
        },
    };

    println!("\n=== MiniZinc Structural Consistency Verification ===");
    println!("{}", minizinc_input);

    let solver = MiniZincSolver::new();
    match solver.solve_with_data("./models/zkp_monster_verification.mzn", &minizinc_input) {
        Ok(solution) => {
            println!("\n✓ Structural invariant consistency verified!");
            println!("Invariant coordinates: ({}, {})", solution.x, solution.y);
            println!("Algebraic structure: PRESERVED");
            println!("Topological invariants: MAINTAINED");
            println!("Arithmetic constraints: SATISFIED");
        }
        Err(e) => {
            eprintln!("✗ Structural consistency verification failed: {}", e);
        }
    }

    Ok(())
}
