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
    crate_distribution: HashMap<String, u64>,
}

#[derive(Debug)]
struct CrateAnalysis {
    crate_name: String,
    total_complexity: f64,
    symbol_count: usize,
    top_symbols: Vec<(String, u64)>,
}

fn phi_hash(symbol: &str) -> u64 {
    let mut hash = 5381u64;
    for byte in symbol.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(byte as u64);
    }
    hash % 196883
}

fn euler_phi(n: u64) -> u64 {
    if n <= 1 {
        return n;
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

fn scan_comprehensive(base_paths: &[&str]) -> HashMap<String, SymbolUsage> {
    let mut global_symbols = HashMap::new();

    for &base_path in base_paths {
        println!("🔍 Scanning {}", base_path);
        scan_directory(Path::new(base_path), &mut global_symbols);
    }

    global_symbols
}

fn scan_directory(dir: &Path, global_symbols: &mut HashMap<String, SymbolUsage>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // Skip target directories and hidden dirs
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if !name.starts_with('.') && name != "target" {
                        scan_directory(&path, global_symbols);
                    }
                }
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                process_rust_file(&path, global_symbols);
            }
        }
    }
}

fn process_rust_file(path: &Path, global_symbols: &mut HashMap<String, SymbolUsage>) {
    if let Ok(content) = fs::read_to_string(path) {
        let crate_name = extract_crate_name(path);
        let module_name = format!(
            "{}::{}",
            crate_name,
            path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
        );

        for line in content.lines() {
            // Extract use statements
            if let Some(symbols) = extract_use_symbols(line) {
                for symbol in symbols {
                    update_symbol_usage(&symbol, &module_name, &crate_name, 1, global_symbols);
                }
            }

            // Count symbol usage in code
            count_symbols_in_line(line, &module_name, &crate_name, global_symbols);
        }
    }
}

fn extract_crate_name(path: &Path) -> String {
    // Try to find Cargo.toml to get real crate name
    let mut current = path.parent();
    while let Some(dir) = current {
        let cargo_toml = dir.join("Cargo.toml");
        if cargo_toml.exists() {
            if let Ok(content) = fs::read_to_string(&cargo_toml) {
                for line in content.lines() {
                    if line.trim().starts_with("name") && line.contains("=") {
                        let name = line
                            .split('=')
                            .nth(1)
                            .map(|s| s.trim().trim_matches('"'))
                            .unwrap_or("unknown");
                        return name.to_string();
                    }
                }
            }
        }
        current = dir.parent();
    }

    // Fallback to directory name
    path.ancestors()
        .nth(2)
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string()
}

fn extract_use_symbols(line: &str) -> Option<Vec<String>> {
    let trimmed = line.trim();
    if !trimmed.starts_with("use ") {
        return None;
    }

    let use_part = &trimmed[4..].trim_end_matches(';');
    let mut symbols = Vec::new();

    if use_part.contains('{') {
        if let Some(start) = use_part.find('{') {
            if let Some(end) = use_part.find('}') {
                let items = &use_part[start + 1..end];
                for item in items.split(',') {
                    let symbol = item.trim().split("::").last().unwrap_or(item.trim());
                    if !symbol.is_empty() && symbol != "self" {
                        symbols.push(symbol.to_string());
                    }
                }
            }
        }
    } else {
        let symbol = use_part.split("::").last().unwrap_or(use_part);
        if !symbol.is_empty() && symbol != "*" && symbol != "self" {
            symbols.push(symbol.to_string());
        }
    }

    if symbols.is_empty() {
        None
    } else {
        Some(symbols)
    }
}

fn count_symbols_in_line(
    line: &str,
    module_name: &str,
    crate_name: &str,
    global_symbols: &mut HashMap<String, SymbolUsage>,
) {
    // Common Rust symbols to track
    let rust_symbols = [
        "Vec",
        "HashMap",
        "String",
        "Option",
        "Result",
        "Box",
        "Rc",
        "Arc",
        "Serialize",
        "Deserialize",
        "Clone",
        "Debug",
        "Display",
        "Error",
        "spawn",
        "Runtime",
        "Future",
        "Stream",
        "async",
        "await",
        "println",
        "format",
        "panic",
        "unwrap",
        "expect",
        "match",
    ];

    for &symbol in &rust_symbols {
        let count = line.matches(symbol).count() as u64;
        if count > 0 {
            update_symbol_usage(symbol, module_name, crate_name, count, global_symbols);
        }
    }
}

fn update_symbol_usage(
    symbol: &str,
    module_name: &str,
    crate_name: &str,
    count: u64,
    global_symbols: &mut HashMap<String, SymbolUsage>,
) {
    let entry = global_symbols
        .entry(symbol.to_string())
        .or_insert_with(|| SymbolUsage {
            symbol: symbol.to_string(),
            phi_value: phi_hash(symbol),
            usage_count: 0,
            complexity_score: 0.0,
            used_in_modules: Vec::new(),
            crate_distribution: HashMap::new(),
        });

    entry.usage_count += count;
    entry.used_in_modules.push(module_name.to_string());
    *entry
        .crate_distribution
        .entry(crate_name.to_string())
        .or_insert(0) += count;
}

fn calculate_complexities(global_symbols: &mut HashMap<String, SymbolUsage>) {
    for symbol_usage in global_symbols.values_mut() {
        let phi_val = euler_phi(symbol_usage.phi_value);
        let module_spread = symbol_usage.used_in_modules.len() as f64;
        let usage_frequency = symbol_usage.usage_count as f64;
        let crate_spread = symbol_usage.crate_distribution.len() as f64;

        // Enhanced complexity: phi * log(usage) * sqrt(modules) * log(crates)
        symbol_usage.complexity_score = (phi_val as f64)
            * usage_frequency.ln()
            * module_spread.sqrt()
            * (crate_spread + 1.0).ln();
    }
}

fn analyze_by_crate(global_symbols: &HashMap<String, SymbolUsage>) -> Vec<CrateAnalysis> {
    let mut crate_analysis = HashMap::new();

    for symbol_usage in global_symbols.values() {
        for (crate_name, &usage_count) in &symbol_usage.crate_distribution {
            let entry = crate_analysis
                .entry(crate_name.clone())
                .or_insert_with(|| CrateAnalysis {
                    crate_name: crate_name.clone(),
                    total_complexity: 0.0,
                    symbol_count: 0,
                    top_symbols: Vec::new(),
                });

            entry.total_complexity += symbol_usage.complexity_score * (usage_count as f64);
            entry.symbol_count += 1;
            entry
                .top_symbols
                .push((symbol_usage.symbol.clone(), usage_count));
        }
    }

    // Sort top symbols for each crate
    for analysis in crate_analysis.values_mut() {
        analysis.top_symbols.sort_by(|a, b| b.1.cmp(&a.1));
        analysis.top_symbols.truncate(5);
    }

    let mut result: Vec<_> = crate_analysis.into_values().collect();
    result.sort_by(|a, b| b.total_complexity.partial_cmp(&a.total_complexity).unwrap());
    result
}

fn main() {
    println!("=== Comprehensive Rustc + Cargo2nix Symbol Analyzer ===");

    let scan_paths = [
        ".",                       // Current cargo2nix
        "./submodules",            // All submodules
        "./tools",                 // All tools
        "./minizinc-introspector", // MiniZinc integration
    ];

    let mut global_symbols = scan_comprehensive(&scan_paths);
    println!("📊 Found {} unique symbols", global_symbols.len());

    calculate_complexities(&mut global_symbols);

    println!("\n=== Top 20 Most Complex Symbols ===");
    let mut sorted_symbols: Vec<_> = global_symbols.values().collect();
    sorted_symbols.sort_by(|a, b| b.complexity_score.partial_cmp(&a.complexity_score).unwrap());

    for (i, symbol) in sorted_symbols.iter().take(20).enumerate() {
        println!(
            "{}. {} (φ: {}, usage: {}, modules: {}, crates: {}, complexity: {:.0})",
            i + 1,
            symbol.symbol,
            symbol.phi_value,
            symbol.usage_count,
            symbol.used_in_modules.len(),
            symbol.crate_distribution.len(),
            symbol.complexity_score
        );
    }

    let crate_analyses = analyze_by_crate(&global_symbols);

    println!("\n=== Top 15 Most Complex Crates ===");
    for (i, analysis) in crate_analyses.iter().take(15).enumerate() {
        println!(
            "{}. {} (complexity: {:.0}, symbols: {})",
            i + 1,
            analysis.crate_name,
            analysis.total_complexity,
            analysis.symbol_count
        );

        println!(
            "   Top symbols: {:?}",
            analysis
                .top_symbols
                .iter()
                .map(|(s, c)| format!("{}({})", s, c))
                .collect::<Vec<_>>()
        );
    }

    println!("\n=== Cross-Crate Symbol Distribution ===");
    for symbol in sorted_symbols.iter().take(10) {
        if symbol.crate_distribution.len() > 3 {
            println!(
                "🔧 {} used across {} crates:",
                symbol.symbol,
                symbol.crate_distribution.len()
            );
            let mut crate_usage: Vec<_> = symbol.crate_distribution.iter().collect();
            crate_usage.sort_by(|a, b| b.1.cmp(a.1));
            for (crate_name, &count) in crate_usage.iter().take(5) {
                println!("   {} ({})", crate_name, count);
            }
        }
    }

    println!("\n✨ Comprehensive analysis complete!");
    println!("📈 Total symbols analyzed: {}", global_symbols.len());
    println!("📈 Total crates analyzed: {}", crate_analyses.len());
}
