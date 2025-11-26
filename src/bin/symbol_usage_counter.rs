use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug)]
struct SymbolUsage {
    symbol: String,
    phi_value: u64,
    usage_count: u64,
    complexity_score: f64,
    used_in_modules: Vec<String>,
}

#[derive(Debug)]
struct ModuleImports {
    module_name: String,
    symbol_usage: HashMap<String, u64>, // symbol -> usage count
    total_complexity: f64,
}

fn phi_hash(symbol: &str) -> u64 {
    let mut hash = 5381u64;
    for byte in symbol.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(byte as u64);
    }
    hash % 196883
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

fn scan_rust_files(dir: &Path) -> HashMap<String, ModuleImports> {
    let mut modules = HashMap::new();
    
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                modules.extend(scan_rust_files(&path));
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                if let Ok(content) = fs::read_to_string(&path) {
                    let module_name = path.file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("unknown")
                        .to_string();
                    
                    let mut symbol_usage = HashMap::new();
                    
                    for line in content.lines() {
                        // Count use statements
                        if let Some(symbols) = extract_use_symbols(line) {
                            for symbol in symbols {
                                *symbol_usage.entry(symbol).or_insert(0) += 1;
                            }
                        }
                        
                        // Count actual symbol usage in code
                        count_symbol_usage_in_line(line, &mut symbol_usage);
                    }
                    
                    modules.insert(module_name.clone(), ModuleImports {
                        module_name,
                        symbol_usage,
                        total_complexity: 0.0,
                    });
                }
            }
        }
    }
    
    modules
}

fn extract_use_symbols(line: &str) -> Option<Vec<String>> {
    let trimmed = line.trim();
    if !trimmed.starts_with("use ") { return None; }
    
    let use_part = &trimmed[4..].trim_end_matches(';');
    let mut symbols = Vec::new();
    
    if use_part.contains('{') {
        // use std::{collections::HashMap, fs};
        if let Some(start) = use_part.find('{') {
            if let Some(end) = use_part.find('}') {
                let items = &use_part[start+1..end];
                for item in items.split(',') {
                    let symbol = item.trim().split("::").last().unwrap_or(item.trim());
                    if !symbol.is_empty() {
                        symbols.push(symbol.to_string());
                    }
                }
            }
        }
    } else {
        // use serde::Serialize;
        let symbol = use_part.split("::").last().unwrap_or(use_part);
        if !symbol.is_empty() && symbol != "*" {
            symbols.push(symbol.to_string());
        }
    }
    
    if symbols.is_empty() { None } else { Some(symbols) }
}

fn count_symbol_usage_in_line(line: &str, symbol_usage: &mut HashMap<String, u64>) {
    // Simple heuristic: count occurrences of known symbols
    let known_symbols = ["Serialize", "Deserialize", "spawn", "Runtime", "HashMap", "Vec", "String"];
    
    for &symbol in &known_symbols {
        let count = line.matches(symbol).count() as u64;
        if count > 0 {
            *symbol_usage.entry(symbol.to_string()).or_insert(0) += count;
        }
    }
}

fn calculate_symbol_complexity(modules: &HashMap<String, ModuleImports>) -> HashMap<String, SymbolUsage> {
    let mut symbol_stats = HashMap::new();
    
    for module in modules.values() {
        for (symbol, &count) in &module.symbol_usage {
            let entry = symbol_stats.entry(symbol.clone()).or_insert_with(|| SymbolUsage {
                symbol: symbol.clone(),
                phi_value: phi_hash(symbol),
                usage_count: 0,
                complexity_score: 0.0,
                used_in_modules: Vec::new(),
            });
            
            entry.usage_count += count;
            entry.used_in_modules.push(module.module_name.clone());
        }
    }
    
    // Calculate complexity scores
    for symbol_usage in symbol_stats.values_mut() {
        let phi_val = euler_phi(symbol_usage.phi_value);
        let module_spread = symbol_usage.used_in_modules.len() as f64;
        let usage_frequency = symbol_usage.usage_count as f64;
        
        // Complexity = phi * log(usage) * sqrt(module_spread)
        symbol_usage.complexity_score = (phi_val as f64) * usage_frequency.ln() * module_spread.sqrt();
    }
    
    symbol_stats
}

fn calculate_module_complexity(modules: &mut HashMap<String, ModuleImports>, symbol_stats: &HashMap<String, SymbolUsage>) {
    for module in modules.values_mut() {
        let mut total = 0.0;
        for (symbol, &count) in &module.symbol_usage {
            if let Some(symbol_usage) = symbol_stats.get(symbol) {
                total += symbol_usage.complexity_score * (count as f64);
            }
        }
        module.total_complexity = total;
    }
}

fn main() {
    println!("=== Per-Symbol Import Complexity Usage Counter ===");
    
    let mut modules = scan_rust_files(Path::new("./src"));
    println!("📦 Scanned {} modules", modules.len());
    
    let symbol_stats = calculate_symbol_complexity(&modules);
    calculate_module_complexity(&mut modules, &symbol_stats);
    
    println!("\n=== Symbol Usage Statistics ===");
    let mut sorted_symbols: Vec<_> = symbol_stats.values().collect();
    sorted_symbols.sort_by(|a, b| b.complexity_score.partial_cmp(&a.complexity_score).unwrap());
    
    for symbol in sorted_symbols.iter().take(10) {
        println!("🔧 {} (φ: {}, usage: {}, modules: {}, complexity: {:.1})",
                symbol.symbol, symbol.phi_value, symbol.usage_count, 
                symbol.used_in_modules.len(), symbol.complexity_score);
    }
    
    println!("\n=== Module Import Complexity ===");
    let mut sorted_modules: Vec<_> = modules.values().collect();
    sorted_modules.sort_by(|a, b| b.total_complexity.partial_cmp(&a.total_complexity).unwrap());
    
    for module in sorted_modules.iter().take(10) {
        println!("📁 {} (complexity: {:.1}, symbols: {})",
                module.module_name, module.total_complexity, module.symbol_usage.len());
        
        // Show top symbols in this module
        let mut module_symbols: Vec<_> = module.symbol_usage.iter().collect();
        module_symbols.sort_by(|a, b| b.1.cmp(a.1));
        
        for (symbol, &count) in module_symbols.iter().take(3) {
            if let Some(stats) = symbol_stats.get(*symbol) {
                println!("   {} (used {} times, φ: {})", symbol, count, stats.phi_value);
            }
        }
    }
    
    println!("\n✨ Symbol complexity analysis complete!");
}
