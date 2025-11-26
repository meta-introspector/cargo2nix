use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
struct CrateUsage {
    crate_name: String,
    used_by: Vec<String>,
    usage_pattern: Vec<String>,
    phi_signature: u64,
    usage_frequency: u64,
}

#[derive(Debug)]
struct UsageMatch {
    crate1: String,
    crate2: String,
    phi_similarity: f64,
    usage_overlap: f64,
    match_confidence: f64,
}

fn euler_phi(n: u64) -> u64 {
    if n <= 1 { return n; }
    let mut result = n;
    let mut num = n;
    let mut p = 2;
    
    while p * p <= num {
        if num % p == 0 {
            while num % p == 0 { num /= p; }
            result -= result / p;
        }
        p += 1;
    }
    if num > 1 { result -= result / num; }
    result
}

fn calculate_usage_phi(usage_pattern: &[String]) -> u64 {
    let mut hash = 1u64;
    for usage in usage_pattern {
        let usage_hash = usage.bytes().fold(5381u64, |acc, b| acc.wrapping_mul(33).wrapping_add(b as u64));
        hash = hash.wrapping_mul(usage_hash % 1009); // Prime modulus
    }
    euler_phi(hash % 196883)
}

fn extract_crate_usage() -> Vec<CrateUsage> {
    let mut usage_map: HashMap<String, CrateUsage> = HashMap::new();
    
    // Scan Cargo.toml files for dependencies
    scan_cargo_files(Path::new("."), &mut usage_map);
    
    // Scan Rust files for use statements
    scan_rust_files(Path::new("."), &mut usage_map);
    
    usage_map.into_values().collect()
}

fn scan_cargo_files(dir: &Path, usage_map: &mut HashMap<String, CrateUsage>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                scan_cargo_files(&path, usage_map);
            } else if path.file_name().map_or(false, |name| name == "Cargo.toml") {
                if let Ok(content) = fs::read_to_string(&path) {
                    let parent_crate = path.parent()
                        .and_then(|p| p.file_name())
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown")
                        .to_string();
                    
                    for line in content.lines() {
                        if let Some(dep) = extract_dependency(line) {
                            let entry = usage_map.entry(dep.clone()).or_insert_with(|| CrateUsage {
                                crate_name: dep.clone(),
                                used_by: Vec::new(),
                                usage_pattern: Vec::new(),
                                phi_signature: 0,
                                usage_frequency: 0,
                            });
                            
                            entry.used_by.push(parent_crate.clone());
                            entry.usage_pattern.push("dependency".to_string());
                            entry.usage_frequency += 1;
                        }
                    }
                }
            }
        }
    }
}

fn scan_rust_files(dir: &Path, usage_map: &mut HashMap<String, CrateUsage>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                scan_rust_files(&path, usage_map);
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                if let Ok(content) = fs::read_to_string(&path) {
                    for line in content.lines() {
                        if let Some((crate_name, usage_type)) = extract_use_statement(line) {
                            let entry = usage_map.entry(crate_name.clone()).or_insert_with(|| CrateUsage {
                                crate_name: crate_name.clone(),
                                used_by: Vec::new(),
                                usage_pattern: Vec::new(),
                                phi_signature: 0,
                                usage_frequency: 0,
                            });
                            
                            entry.usage_pattern.push(usage_type);
                            entry.usage_frequency += 1;
                        }
                    }
                }
            }
        }
    }
}

fn extract_dependency(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if trimmed.contains("=") && !trimmed.starts_with("#") && !trimmed.starts_with("[") {
        let parts: Vec<&str> = trimmed.split('=').collect();
        if parts.len() >= 2 {
            let dep_name = parts[0].trim().trim_matches('"');
            if !dep_name.is_empty() && dep_name != "version" && dep_name != "path" {
                return Some(dep_name.to_string());
            }
        }
    }
    None
}

fn extract_use_statement(line: &str) -> Option<(String, String)> {
    let trimmed = line.trim();
    if trimmed.starts_with("use ") {
        let use_part = &trimmed[4..];
        let crate_part = use_part.split("::").next().unwrap_or(use_part);
        let crate_name = crate_part.split('{').next().unwrap_or(crate_part).trim();
        
        let usage_type = if use_part.contains("::") {
            if use_part.contains("{") { "multi_import" }
            else if use_part.contains("*") { "glob_import" }
            else { "specific_import" }
        } else { "crate_import" };
        
        Some((crate_name.to_string(), usage_type.to_string()))
    } else {
        None
    }
}

fn calculate_phi_signatures(usages: &mut [CrateUsage]) {
    for usage in usages {
        usage.phi_signature = calculate_usage_phi(&usage.usage_pattern);
    }
}

fn find_usage_matches(usages: &[CrateUsage]) -> Vec<UsageMatch> {
    let mut matches = Vec::new();
    
    for i in 0..usages.len() {
        for j in (i + 1)..usages.len() {
            let usage1 = &usages[i];
            let usage2 = &usages[j];
            
            if usage1.crate_name == usage2.crate_name { continue; }
            
            let phi_diff = if usage1.phi_signature > usage2.phi_signature {
                usage1.phi_signature - usage2.phi_signature
            } else {
                usage2.phi_signature - usage1.phi_signature
            };
            
            let phi_similarity = 1.0 - (phi_diff as f64 / 196883.0);
            
            let common_patterns = usage1.usage_pattern.iter()
                .filter(|p| usage2.usage_pattern.contains(p))
                .count();
            
            let total_patterns = (usage1.usage_pattern.len() + usage2.usage_pattern.len()) as f64;
            let usage_overlap = if total_patterns > 0.0 {
                (2.0 * common_patterns as f64) / total_patterns
            } else { 0.0 };
            
            let freq_ratio = if usage2.usage_frequency > 0 {
                (usage1.usage_frequency as f64 / usage2.usage_frequency as f64).min(1.0)
            } else { 0.0 };
            
            let match_confidence = (phi_similarity * 0.4) + (usage_overlap * 0.4) + (freq_ratio * 0.2);
            
            if match_confidence > 0.7 {
                matches.push(UsageMatch {
                    crate1: usage1.crate_name.clone(),
                    crate2: usage2.crate_name.clone(),
                    phi_similarity,
                    usage_overlap,
                    match_confidence,
                });
            }
        }
    }
    
    matches.sort_by(|a, b| b.match_confidence.partial_cmp(&a.match_confidence).unwrap());
    matches
}

fn main() {
    println!("=== Phi Usage Pattern Matcher ===");
    
    let mut usages = extract_crate_usage();
    println!("📦 Found {} crates with usage patterns", usages.len());
    
    calculate_phi_signatures(&mut usages);
    
    println!("\n=== Crate Usage Analysis ===");
    for usage in &usages {
        if usage.usage_frequency > 2 {
            println!("📦 {} (φ: {}, freq: {}, patterns: {})", 
                    usage.crate_name, usage.phi_signature, usage.usage_frequency, usage.usage_pattern.len());
            println!("   Used by: {:?}", usage.used_by);
            println!("   Patterns: {:?}", usage.usage_pattern);
        }
    }
    
    let matches = find_usage_matches(&usages);
    println!("\n=== Automatic Implementation Matches ===");
    println!("🎯 Found {} potential matches", matches.len());
    
    for (i, m) in matches.iter().take(20).enumerate() {
        println!("{}. {} ↔ {} (confidence: {:.2}, φ-sim: {:.2}, overlap: {:.2})",
                i + 1, m.crate1, m.crate2, m.match_confidence, m.phi_similarity, m.usage_overlap);
    }
    
    // Group by phi signature
    let mut phi_groups: HashMap<u64, Vec<&CrateUsage>> = HashMap::new();
    for usage in &usages {
        phi_groups.entry(usage.phi_signature).or_default().push(usage);
    }
    
    println!("\n=== Phi Signature Groups ===");
    for (phi_sig, group) in phi_groups {
        if group.len() > 1 {
            println!("🧮 φ = {}: {} crates with identical usage patterns", phi_sig, group.len());
            for usage in group {
                println!("   {}", usage.crate_name);
            }
        }
    }
    
    println!("\n✨ Phi-based usage analysis complete!");
}
