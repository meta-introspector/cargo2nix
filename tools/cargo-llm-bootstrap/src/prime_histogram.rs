use std::collections::HashMap;
use crate::rustc_monster_assignment::MonsterGroupVerifier;

pub fn generate_rustc_prime_histogram(rustc_path: &str) -> Result<(), String> {
    let mut verifier = MonsterGroupVerifier::new();
    verifier.assign_rustc_crates(rustc_path)?;
    
    // Monster Group primes: 2^46 × 3^20 × 5^9 × 7^6 × 11^2 × 13^3 × 17 × 19 × 23 × 29 × 31 × 41 × 47 × 59 × 71
    let monster_primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71];
    let mut prime_counts: HashMap<u64, u32> = HashMap::new();
    
    // Count assignments per prime
    for assignment in &verifier.crate_assignments {
        *prime_counts.entry(assignment.assigned_prime).or_insert(0) += 1;
    }
    
    println!("Rustc Crate → Monster Group Prime Distribution");
    println!("============================================");
    
    for &prime in &monster_primes {
        let count = prime_counts.get(&prime).unwrap_or(&0);
        let bar = "█".repeat(*count as usize);
        println!("{:2}: {:3} crates {}", prime, count, bar);
    }
    
    let total_crates = verifier.crate_assignments.len();
    println!("\nTotal rustc crates analyzed: {}", total_crates);
    
    Ok(())
}
