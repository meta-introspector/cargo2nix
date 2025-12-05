/// Print histogram of prime exponents for Solana rustc directory structure
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Prime factorization up to 100 for directory counting
const PRIMES: [u64; 25] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 
    53, 59, 61, 67, 71, 73, 79, 83, 89, 97
];

pub fn print_rustc_directory_histogram(rust_src_path: &str) -> Result<(), String> {
    println!("🗂️  Rustc Directory Prime Exponent Histogram");
    println!("Path: {}", rust_src_path);
    println!("=" .repeat(60));
    
    let rust_path = Path::new(rust_src_path);
    if !rust_path.exists() {
        return Err(format!("Path does not exist: {}", rust_src_path));
    }
    
    // Count directories at each level
    let dir_counts = count_directories(rust_path)?;
    
    // Calculate prime factorizations
    let prime_factors = calculate_prime_factors(&dir_counts);
    
    // Print histogram
    print_histogram(&dir_counts, &prime_factors);
    
    Ok(())
}

fn count_directories(rust_path: &Path) -> Result<HashMap<String, usize>, String> {
    let mut counts = HashMap::new();
    
    // Key rustc directory categories
    let categories = [
        ("compiler", "compiler"),
        ("library", "library"), 
        ("src", "src"),
        ("tests", "tests"),
    ];
    
    for (category, path_str) in categories {
        let dir_path = rust_path.join(path_str);
        if dir_path.exists() {
            let count = count_subdirs(&dir_path)?;
            counts.insert(category.to_string(), count);
        }
    }
    
    // Count compiler subdirectories specifically
    let compiler_path = rust_path.join("compiler");
    if compiler_path.exists() {
        let rustc_dirs = count_rustc_crates(&compiler_path)?;
        counts.insert("rustc_crates".to_string(), rustc_dirs);
    }
    
    Ok(counts)
}

fn count_subdirs(dir_path: &Path) -> Result<usize, String> {
    let entries = fs::read_dir(dir_path)
        .map_err(|e| format!("Failed to read directory {:?}: {}", dir_path, e))?;
    
    let mut count = 0;
    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        if entry.file_type().map_err(|e| format!("Failed to get file type: {}", e))?.is_dir() {
            count += 1;
        }
    }
    
    Ok(count)
}

fn count_rustc_crates(compiler_path: &Path) -> Result<usize, String> {
    let entries = fs::read_dir(compiler_path)
        .map_err(|e| format!("Failed to read compiler directory: {}", e))?;
    
    let mut rustc_count = 0;
    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        if entry.file_type().map_err(|e| format!("Failed to get file type: {}", e))?.is_dir() {
            if let Some(name) = entry.file_name().to_str() {
                if name.starts_with("rustc_") {
                    rustc_count += 1;
                }
            }
        }
    }
    
    Ok(rustc_count)
}

fn calculate_prime_factors(counts: &HashMap<String, usize>) -> HashMap<String, Vec<(u64, u32)>> {
    let mut factors = HashMap::new();
    
    for (category, &count) in counts {
        let prime_factorization = factorize(count as u64);
        factors.insert(category.clone(), prime_factorization);
    }
    
    factors
}

fn factorize(mut n: u64) -> Vec<(u64, u32)> {
    let mut factors = Vec::new();
    
    for &prime in &PRIMES {
        if prime * prime > n {
            break;
        }
        
        let mut exponent = 0;
        while n % prime == 0 {
            n /= prime;
            exponent += 1;
        }
        
        if exponent > 0 {
            factors.push((prime, exponent));
        }
    }
    
    if n > 1 {
        factors.push((n, 1));
    }
    
    factors
}

fn print_histogram(counts: &HashMap<String, usize>, factors: &HashMap<String, Vec<(u64, u32)>>) {
    println!("Directory Counts:");
    println!("-" .repeat(40));
    
    for (category, &count) in counts {
        println!("{:20} : {:3} directories", category, count);
    }
    
    println!("\nPrime Factorizations:");
    println!("-" .repeat(40));
    
    for (category, factor_list) in factors {
        print!("{:20} : ", category);
        if factor_list.is_empty() {
            println!("1");
        } else {
            let factor_strs: Vec<String> = factor_list.iter()
                .map(|(p, e)| if *e == 1 { format!("{}", p) } else { format!("{}^{}", p, e) })
                .collect();
            println!("{}", factor_strs.join(" × "));
        }
    }
    
    println!("\nHistogram of Prime Exponents:");
    println!("-" .repeat(40));
    
    let mut all_primes = HashMap::new();
    for factor_list in factors.values() {
        for &(prime, exponent) in factor_list {
            *all_primes.entry(prime).or_insert(0) += exponent;
        }
    }
    
    let mut sorted_primes: Vec<_> = all_primes.iter().collect();
    sorted_primes.sort_by_key(|(p, _)| *p);
    
    for (&prime, &total_exp) in sorted_primes {
        let bar = "█".repeat(total_exp as usize);
        println!("{:3} : {:2} {}", prime, total_exp, bar);
    }
    
    // Compare with Monster Group
    println!("\nMonster Group Target Exponents:");
    println!("-" .repeat(40));
    let monster_factors = [
        (2, 46), (3, 20), (5, 9), (7, 6), (11, 2), (13, 3),
        (17, 1), (19, 1), (23, 1), (29, 1), (31, 1), (41, 1),
        (47, 1), (59, 1), (71, 1)
    ];
    
    for (prime, target_exp) in monster_factors {
        let current_exp = all_primes.get(&prime).copied().unwrap_or(0);
        let status = if current_exp == target_exp { "✅" } else { "❌" };
        println!("{:3} : {:2} → {:2} {}", prime, current_exp, target_exp, status);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_factorize() {
        assert_eq!(factorize(12), vec![(2, 2), (3, 1)]);
        assert_eq!(factorize(17), vec![(17, 1)]);
        assert_eq!(factorize(1), vec![]);
    }
}
