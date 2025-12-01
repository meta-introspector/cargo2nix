use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
struct UnresolvedUsage {
    name: String,
    usage_type: String, // "call", "use", "type"
    source_file: String,
    line_number: usize,
}

#[derive(Debug, Clone)]
struct CargoImport {
    crate_name: String,
    exports: Vec<String>,
    phi_values: HashMap<String, u64>,
}

struct RustcUsageResolver {
    unresolved: Vec<UnresolvedUsage>,            // All ? placeholders
    cargo_imports: HashMap<String, CargoImport>, // Available crates
    resolved: HashMap<String, (String, u64)>,    // name -> (crate, phi)
}

impl RustcUsageResolver {
    fn new() -> Self {
        Self {
            unresolved: Vec::new(),
            cargo_imports: HashMap::new(),
            resolved: HashMap::new(),
        }
    }

    fn index_rustc_file(&mut self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;
        println!("Indexing rustc file: {}", file_path);

        for (line_num, line) in content.lines().enumerate() {
            let trimmed = line.trim();

            // Find function calls: something()
            if let Some(call_pos) = trimmed.find('(') {
                let before_paren = &trimmed[..call_pos];
                if let Some(last_space) = before_paren.rfind(' ') {
                    let call_name = &before_paren[last_space + 1..];
                    if call_name.chars().all(|c| c.is_alphanumeric() || c == '_')
                        && !call_name.is_empty()
                    {
                        self.unresolved.push(UnresolvedUsage {
                            name: call_name.to_string(),
                            usage_type: "call".to_string(),
                            source_file: file_path.to_string(),
                            line_number: line_num + 1,
                        });
                        println!("  ? call: {} (line {})", call_name, line_num + 1);
                    }
                }
            }

            // Find use statements
            if trimmed.starts_with("use ") {
                let use_part = trimmed
                    .strip_prefix("use ")
                    .unwrap_or("")
                    .strip_suffix(';')
                    .unwrap_or("");
                if let Some(last_colon) = use_part.rfind("::") {
                    let item = &use_part[last_colon + 2..];
                    self.unresolved.push(UnresolvedUsage {
                        name: item.to_string(),
                        usage_type: "use".to_string(),
                        source_file: file_path.to_string(),
                        line_number: line_num + 1,
                    });
                    println!("  ? use: {} (line {})", item, line_num + 1);
                }
            }
        }

        Ok(())
    }

    fn add_cargo_crate(&mut self, crate_name: &str, exports: Vec<&str>) {
        let mut phi_values = HashMap::new();
        for export in &exports {
            let phi = calculate_phi_key(export, "fn");
            phi_values.insert(export.to_string(), phi);
        }

        let cargo_import = CargoImport {
            crate_name: crate_name.to_string(),
            exports: exports.iter().map(|s| s.to_string()).collect(),
            phi_values,
        };

        self.cargo_imports
            .insert(crate_name.to_string(), cargo_import);
        println!(
            "Added cargo crate: {} with {} exports",
            crate_name,
            exports.len()
        );
    }

    fn resolve_all_usages(&mut self) {
        println!("\n=== Resolving ? placeholders ===");

        for usage in &self.unresolved {
            let mut best_match = None;
            let mut best_score = 0;

            // Find best matching crate
            for (crate_name, cargo) in &self.cargo_imports {
                for export in &cargo.exports {
                    let score = similarity_score(&usage.name, export);
                    if score > best_score {
                        best_score = score;
                        best_match = Some((crate_name.clone(), export.clone()));
                    }
                }
            }

            if let Some((crate_name, export_name)) = best_match {
                if best_score > 50 {
                    // Threshold for good match
                    let phi = self.cargo_imports[&crate_name].phi_values[&export_name];
                    self.resolved
                        .insert(usage.name.clone(), (crate_name.clone(), phi));
                    println!(
                        "  {} → {}::{} (φ = {}, score: {})",
                        usage.name, crate_name, export_name, phi, best_score
                    );
                } else {
                    println!("  {} → UNRESOLVED (best score: {})", usage.name, best_score);
                }
            } else {
                println!("  {} → NO MATCH", usage.name);
            }
        }
    }

    fn calculate_total_phi_sum(&self) -> u64 {
        self.resolved.values().map(|(_, phi)| phi).sum()
    }
}

fn similarity_score(a: &str, b: &str) -> u32 {
    if a == b {
        return 100;
    }
    if a.contains(b) || b.contains(a) {
        return 80;
    }

    let common_chars = a.chars().filter(|c| b.contains(*c)).count();
    (common_chars * 100 / a.len().max(b.len()).max(1)) as u32
}

fn calculate_phi_key(name: &str, decl_type: &str) -> u64 {
    let name_hash = name.bytes().map(|b| b as u64).sum::<u64>();
    let type_factor = match decl_type {
        "fn" => 5,
        "struct" => 2,
        "enum" => 3,
        _ => 1,
    };
    let monster_element = (name_hash * type_factor + 71) % 196883;
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
    println!("=== Rustc Usage Resolver ===");

    let mut resolver = RustcUsageResolver::new();

    // Index rustc files (using our actual files as examples)
    resolver.index_rustc_file("src/bin/real_monster_solver.rs")?;
    resolver.index_rustc_file("src/bin/meme_pda_storage.rs")?;

    // Add cargo crates from our submodules
    resolver.add_cargo_crate(
        "serde",
        vec!["serialize", "deserialize", "Serialize", "Deserialize"],
    );
    resolver.add_cargo_crate("tokio", vec!["spawn", "block_on", "Runtime", "sleep"]);
    resolver.add_cargo_crate(
        "std",
        vec!["println", "format", "write", "read_to_string", "HashMap"],
    );
    resolver.add_cargo_crate(
        "minizinc",
        vec!["solve", "constraint", "variable", "output"],
    );

    // Resolve all ? placeholders
    resolver.resolve_all_usages();

    println!("\n=== Resolution Summary ===");
    println!("Total unresolved usages: {}", resolver.unresolved.len());
    println!("Successfully resolved: {}", resolver.resolved.len());
    println!("Total phi sum: {}", resolver.calculate_total_phi_sum());

    println!("\n✓ Rustc usage indexing complete");
    println!("✓ ? placeholders resolved via cargo matching");
    println!("✓ Solver matched closest implementations");

    Ok(())
}
