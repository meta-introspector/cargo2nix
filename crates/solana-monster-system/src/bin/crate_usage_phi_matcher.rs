use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
struct CrateUsageProfile {
    crate_name: String,
    used_functions: Vec<String>,
    usage_phi_signature: u64,
    usage_pattern: Vec<u64>, // phi values of used functions
}

struct PhiUsageMatcher {
    crate_profiles: HashMap<String, CrateUsageProfile>,
    similarity_matrix: HashMap<(String, String), f64>,
}

impl PhiUsageMatcher {
    fn new() -> Self {
        Self {
            crate_profiles: HashMap::new(),
            similarity_matrix: HashMap::new(),
        }
    }

    fn analyze_crate_usage(
        &mut self,
        crate_name: &str,
        file_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;
        let mut used_functions = Vec::new();
        let mut usage_pattern = Vec::new();

        println!("Analyzing {} usage in {}", crate_name, file_path);

        // Extract function calls
        for line in content.lines() {
            let trimmed = line.trim();

            // Find function calls: something()
            if let Some(call_pos) = trimmed.find('(') {
                let before_paren = &trimmed[..call_pos];
                if let Some(last_space) = before_paren.rfind(' ') {
                    let call_name = &before_paren[last_space + 1..];
                    if call_name.chars().all(|c| c.is_alphanumeric() || c == '_')
                        && !call_name.is_empty()
                    {
                        let phi = calculate_phi_key(call_name);
                        used_functions.push(call_name.to_string());
                        usage_pattern.push(phi);
                        println!("  uses: {} (φ = {})", call_name, phi);
                    }
                }
            }
        }

        // Calculate usage phi signature (sum of all usage phis)
        let usage_phi_signature = usage_pattern.iter().sum();

        let profile = CrateUsageProfile {
            crate_name: crate_name.to_string(),
            used_functions,
            usage_phi_signature,
            usage_pattern,
        };

        println!("  Usage signature: φ = {}", usage_phi_signature);
        self.crate_profiles.insert(crate_name.to_string(), profile);

        Ok(())
    }

    fn calculate_usage_similarity(&mut self) {
        println!("\n=== Calculating Usage Similarity Matrix ===");

        let crate_names: Vec<String> = self.crate_profiles.keys().cloned().collect();

        for i in 0..crate_names.len() {
            for j in i + 1..crate_names.len() {
                let crate_a = &crate_names[i];
                let crate_b = &crate_names[j];

                let similarity = self.compute_phi_similarity(crate_a, crate_b);
                self.similarity_matrix
                    .insert((crate_a.clone(), crate_b.clone()), similarity);

                println!(
                    "  {} ↔ {}: {:.2}% similar",
                    crate_a,
                    crate_b,
                    similarity * 100.0
                );
            }
        }
    }

    fn compute_phi_similarity(&self, crate_a: &str, crate_b: &str) -> f64 {
        let profile_a = &self.crate_profiles[crate_a];
        let profile_b = &self.crate_profiles[crate_b];

        // Phi signature similarity
        let sig_diff = (profile_a.usage_phi_signature as i64 - profile_b.usage_phi_signature as i64)
            .abs() as f64;
        let sig_similarity = 1.0 / (1.0 + sig_diff / 10000.0);

        // Pattern overlap similarity
        let common_phis = profile_a
            .usage_pattern
            .iter()
            .filter(|phi| profile_b.usage_pattern.contains(phi))
            .count() as f64;

        let total_unique = (profile_a.usage_pattern.len() + profile_b.usage_pattern.len()) as f64;
        let pattern_similarity = if total_unique > 0.0 {
            (common_phis * 2.0) / total_unique
        } else {
            0.0
        };

        // Combined similarity
        (sig_similarity * 0.6) + (pattern_similarity * 0.4)
    }

    fn find_implementation_matches(&self) -> Vec<(String, String, f64)> {
        let mut matches = Vec::new();

        for ((crate_a, crate_b), similarity) in &self.similarity_matrix {
            if *similarity > 0.7 {
                // High similarity threshold
                matches.push((crate_a.clone(), crate_b.clone(), *similarity));
            }
        }

        matches.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
        matches
    }
}

fn calculate_phi_key(name: &str) -> u64 {
    let name_hash = name.bytes().map(|b| b as u64).sum::<u64>();
    let monster_element = (name_hash * 5 + 71) % 196883;
    euler_phi(monster_element)
}

fn euler_phi(n: u64) -> u64 {
    if n <= 1 {
        return 1;
    }
    let mut result = n;
    let mut num = n;
    let mut p = 2;

    while p * p <= num {
        if num % p == 0 {
            while num % p == 0 {
                num /= p;
            }
            result -= result / p;
        }
        p += 1;
    }
    if num > 1 {
        result -= result / num;
    }
    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Crate Usage Phi Similarity Matcher ===");

    let mut matcher = PhiUsageMatcher::new();

    // Analyze usage patterns in our actual files
    matcher.analyze_crate_usage("meme_pda", "src/bin/meme_pda_storage.rs")?;
    matcher.analyze_crate_usage("monster_solver", "src/bin/real_monster_solver.rs")?;
    matcher.analyze_crate_usage("multi_input", "src/bin/multi_input_solfunmeme.rs")?;

    // Calculate similarity matrix
    matcher.calculate_usage_similarity();

    // Find automatic implementation matches
    println!("\n=== Automatic Implementation Matches ===");
    let matches = matcher.find_implementation_matches();

    if matches.is_empty() {
        println!("No high-similarity matches found (threshold: 70%)");
    } else {
        for (crate_a, crate_b, similarity) in &matches {
            println!(
                "🎯 {} ≈ {} ({:.1}% similar)",
                crate_a,
                crate_b,
                similarity * 100.0
            );
            println!("   → Can automatically substitute implementations");
        }
    }

    // Show all similarity scores
    println!("\n=== All Similarity Scores ===");
    for ((crate_a, crate_b), similarity) in &matcher.similarity_matrix {
        println!("  {} ↔ {}: {:.1}%", crate_a, crate_b, similarity * 100.0);
    }

    println!("\n✓ Usage phi signatures calculated");
    println!("✓ Similarity matrix computed");
    println!("✓ Automatic implementation matching ready");

    Ok(())
}
