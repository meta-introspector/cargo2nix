use cargo2nix::{InteractiveConstraintMatcher, MiniZincSolver};
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Interactive Rust Compiler Constraint Matcher");

    let mut matcher = InteractiveConstraintMatcher::new();
    let solver = MiniZincSolver::new();

    loop {
        println!("\n=== Current Partial Matches ===");
        for (name, match_data) in &matcher.rustc_components {
            println!(
                "{}: score={:.2}, constraints={:?}",
                name, match_data.match_score, match_data.constraints
            );
        }

        println!("\n=== Review Suggestions ===");
        for suggestion in matcher.interactive_review() {
            println!("{}", suggestion);
        }

        println!("\n=== Current MiniZinc Constraints ===");
        let constraints = matcher.get_current_minizinc_input();
        println!("{}", constraints);

        // Test current constraints
        match solver.solve_with_data("./models/vernacular_monster_path.mzn", &constraints) {
            Ok(solution) => {
                println!("✓ Constraints satisfiable: cost={}", solution.objective);
            }
            Err(_) => {
                println!("✗ Constraints unsatisfiable - need tweaking");
            }
        }

        print!("\nTweak component (rustc_parse/rustc_hir/rustc_middle) or 'quit': ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input == "quit" {
            break;
        }

        if matcher.tweak_constraint(input, 3) {
            println!("✓ Tweaked {}", input);
        } else {
            println!("✗ Unknown component: {}", input);
        }
    }

    println!("Final constraints optimized for Rust compiler partial match!");
    Ok(())
}
