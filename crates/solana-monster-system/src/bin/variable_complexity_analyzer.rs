use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
struct Variable {
    name: String,
    var_type: String,
    usage_count: u64,
    prime_factors: Vec<u64>,
    complexity_score: u64,
}

#[derive(Debug)]
struct ModuleProfile {
    module_name: String,
    imports: Vec<String>,
    exports: Vec<String>,
    functions: Vec<String>,
    variables: Vec<Variable>,
    total_complexity: u64,
    prime_signature: Vec<u64>,
}

fn get_primes() -> Vec<u64> {
    vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97,
    ]
}

fn prime_factorize(mut n: u64, primes: &[u64]) -> Vec<u64> {
    let mut factors = Vec::new();
    for &prime in primes {
        while n % prime == 0 {
            factors.push(prime);
            n /= prime;
        }
        if n == 1 {
            break;
        }
    }
    if n > 1 {
        factors.push(n);
    }
    factors
}

fn hash_string(s: &str) -> u64 {
    let mut hash = 5381u64;
    for byte in s.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(byte as u64);
    }
    hash % 10000 + 100
}

fn extract_variables_from_line(line: &str) -> Vec<(String, String)> {
    let mut vars = Vec::new();
    let trimmed = line.trim();

    if trimmed.starts_with("let ") {
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() >= 2 {
            let var_name = parts[1].trim_end_matches(':').to_string();
            let var_type = if parts.len() >= 3 && parts[2] != "=" {
                parts[2].to_string()
            } else {
                "inferred".to_string()
            };
            vars.push((var_name, var_type));
        }
    }

    vars
}

fn scan_module(path: &Path) -> Option<ModuleProfile> {
    let content = fs::read_to_string(path).ok()?;
    let module_name = path.file_stem()?.to_str()?.to_string();
    let primes = get_primes();

    let mut imports = Vec::new();
    let mut exports = Vec::new();
    let mut functions = Vec::new();
    let mut variable_usage: HashMap<String, (String, u64)> = HashMap::new();

    // First pass: collect variables and basic info
    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("use ") {
            let import = trimmed[4..].trim_end_matches(';');
            imports.push(import.to_string());
        }

        if trimmed.starts_with("pub fn ") || trimmed.starts_with("pub struct ") {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 3 {
                exports.push(parts[2].split('(').next().unwrap_or(parts[2]).to_string());
            }
        }

        if trimmed.starts_with("fn ") {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 2 {
                functions.push(parts[1].split('(').next().unwrap_or(parts[1]).to_string());
            }
        }

        for (var_name, var_type) in extract_variables_from_line(line) {
            variable_usage.entry(var_name).or_insert((var_type, 1));
        }
    }

    // Second pass: count usage
    let var_names: Vec<String> = variable_usage.keys().cloned().collect();
    for var_name in var_names {
        let mut total_usage = 0;
        for line in content.lines() {
            total_usage += line.matches(&var_name).count() as u64;
        }
        if let Some(entry) = variable_usage.get_mut(&var_name) {
            entry.1 = total_usage.max(1);
        }
    }

    // Create variables with complexity scores
    let mut variables = Vec::new();
    for (name, (var_type, usage)) in variable_usage {
        let type_hash = hash_string(&var_type);
        let name_hash = hash_string(&name);
        let combined = type_hash + usage + name_hash;

        let prime_factors = prime_factorize(combined, &primes);
        let complexity_score = if prime_factors.is_empty() {
            1
        } else {
            prime_factors.iter().product::<u64>()
        };

        variables.push(Variable {
            name,
            var_type,
            usage_count: usage,
            prime_factors,
            complexity_score,
        });
    }

    let total_complexity = variables.iter().map(|v| v.complexity_score).sum();
    let mut all_factors = Vec::new();
    for var in &variables {
        all_factors.extend(&var.prime_factors);
    }
    all_factors.sort();
    all_factors.dedup();

    Some(ModuleProfile {
        module_name,
        imports,
        exports,
        functions,
        variables,
        total_complexity,
        prime_signature: all_factors,
    })
}

fn find_similar_modules(modules: &[ModuleProfile]) -> Vec<(String, String, f64)> {
    let mut similarities = Vec::new();

    for i in 0..modules.len() {
        for j in (i + 1)..modules.len() {
            let m1 = &modules[i];
            let m2 = &modules[j];

            // Compare prime signatures
            let common_primes = m1
                .prime_signature
                .iter()
                .filter(|p| m2.prime_signature.contains(p))
                .count();

            let total_primes = (m1.prime_signature.len() + m2.prime_signature.len()) as f64;
            let prime_similarity = if total_primes > 0.0 {
                (2.0 * common_primes as f64) / total_primes
            } else {
                0.0
            };

            // Compare variable types
            let m1_types: Vec<_> = m1.variables.iter().map(|v| &v.var_type).collect();
            let m2_types: Vec<_> = m2.variables.iter().map(|v| &v.var_type).collect();

            let common_types = m1_types.iter().filter(|t| m2_types.contains(t)).count();

            let total_types = (m1_types.len() + m2_types.len()) as f64;
            let type_similarity = if total_types > 0.0 {
                (2.0 * common_types as f64) / total_types
            } else {
                0.0
            };

            let overall_similarity = (prime_similarity * 0.6) + (type_similarity * 0.4);

            if overall_similarity > 0.3 {
                similarities.push((
                    m1.module_name.clone(),
                    m2.module_name.clone(),
                    overall_similarity,
                ));
            }
        }
    }

    similarities.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
    similarities
}

fn find_duplicate_variables(modules: &[ModuleProfile]) {
    println!("=== Duplicate Variables Analysis ===");

    let mut var_signatures: HashMap<Vec<u64>, Vec<(String, String)>> = HashMap::new();

    for module in modules {
        for var in &module.variables {
            var_signatures
                .entry(var.prime_factors.clone())
                .or_default()
                .push((module.module_name.clone(), var.name.clone()));
        }
    }

    for (factors, occurrences) in var_signatures {
        if occurrences.len() > 1 {
            println!("🔄 Identical variables (factors: {:?}):", factors);
            for (module, var_name) in occurrences {
                println!("   {}::{}", module, var_name);
            }
        }
    }
}

fn main() {
    println!("=== Variable Complexity Analyzer ===");

    let mut modules = Vec::new();

    // Scan source files
    if let Ok(entries) = fs::read_dir("./src") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "rs") {
                if let Some(module) = scan_module(&path) {
                    modules.push(module);
                }
            }
        }
    }

    println!("📦 Analyzed {} modules", modules.len());

    // Show top complex modules
    modules.sort_by(|a, b| b.total_complexity.cmp(&a.total_complexity));

    println!("\n=== Top 10 Most Complex Modules ===");
    for (i, module) in modules.iter().take(10).enumerate() {
        println!(
            "{}. {} (complexity: {}, vars: {}, primes: {:?})",
            i + 1,
            module.module_name,
            module.total_complexity,
            module.variables.len(),
            module.prime_signature
        );

        // Show top variables
        let mut vars = module.variables.clone();
        vars.sort_by(|a, b| b.complexity_score.cmp(&a.complexity_score));
        for var in vars.iter().take(3) {
            println!(
                "   {} : {} (usage: {}, complexity: {})",
                var.name, var.var_type, var.usage_count, var.complexity_score
            );
        }
    }

    find_duplicate_variables(&modules);

    let similarities = find_similar_modules(&modules);
    println!("\n=== Similar Modules ===");
    for (m1, m2, sim) in similarities.iter().take(10) {
        println!("🔗 {} ↔ {} (similarity: {:.2})", m1, m2, sim);
    }

    println!("\n✨ Variable complexity analysis complete!");
}
