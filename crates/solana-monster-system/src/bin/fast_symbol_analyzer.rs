use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug)]
struct SymbolStats {
    symbol: String,
    phi_value: u64,
    total_usage: u64,
    crate_count: u64,
    complexity: f64,
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

fn quick_scan() -> HashMap<String, SymbolStats> {
    let mut symbols = HashMap::new();
    
    // Key directories to scan
    let dirs = ["./src", "./tools", "./submodules/BLAKE3/src"];
    
    for dir in &dirs {
        if Path::new(dir).exists() {
            println!("📁 Scanning {}", dir);
            scan_dir(Path::new(dir), &mut symbols);
        }
    }
    
    symbols
}

fn scan_dir(dir: &Path, symbols: &mut HashMap<String, SymbolStats>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                scan_dir(&path, symbols);
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                scan_file(&path, symbols);
            }
        }
    }
}

fn scan_file(path: &Path, symbols: &mut HashMap<String, SymbolStats>) {
    if let Ok(content) = fs::read_to_string(path) {
        let crate_name = path.ancestors().nth(2)
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");
        
        // Track common Rust symbols
        let rust_symbols = [
            "Vec", "HashMap", "String", "Option", "Result", "Box", "Rc", "Arc",
            "Serialize", "Deserialize", "Clone", "Debug", "Display", "Error",
            "spawn", "Runtime", "Future", "async", "await", "println", "format",
            "unwrap", "expect", "match", "if", "let", "fn", "struct", "enum", "trait",
        ];
        
        for &symbol in &rust_symbols {
            let count = content.matches(symbol).count() as u64;
            if count > 0 {
                let entry = symbols.entry(symbol.to_string()).or_insert_with(|| SymbolStats {
                    symbol: symbol.to_string(),
                    phi_value: phi_hash(symbol),
                    total_usage: 0,
                    crate_count: 0,
                    complexity: 0.0,
                });
                
                entry.total_usage += count;
                entry.crate_count += 1;
            }
        }
    }
}

fn calculate_complexities(symbols: &mut HashMap<String, SymbolStats>) {
    for stats in symbols.values_mut() {
        let phi_val = euler_phi(stats.phi_value);
        stats.complexity = (phi_val as f64) * 
                          (stats.total_usage as f64).ln() * 
                          (stats.crate_count as f64).sqrt();
    }
}

fn main() {
    println!("=== Fast Rustc + Cargo2nix Symbol Analyzer ===");
    
    let mut symbols = quick_scan();
    println!("📊 Analyzed {} symbols", symbols.len());
    
    calculate_complexities(&mut symbols);
    
    println!("\n=== Top 15 Most Complex Symbols ===");
    let mut sorted: Vec<_> = symbols.values().collect();
    sorted.sort_by(|a, b| b.complexity.partial_cmp(&a.complexity).unwrap());
    
    for (i, symbol) in sorted.iter().take(15).enumerate() {
        println!("{}. {} (φ: {}, usage: {}, crates: {}, complexity: {:.0})",
                i + 1, symbol.symbol, symbol.phi_value, symbol.total_usage,
                symbol.crate_count, symbol.complexity);
    }
    
    println!("\n=== Most Used Symbols ===");
    sorted.sort_by(|a, b| b.total_usage.cmp(&a.total_usage));
    
    for (i, symbol) in sorted.iter().take(10).enumerate() {
        println!("{}. {} (used {} times across {} crates)",
                i + 1, symbol.symbol, symbol.total_usage, symbol.crate_count);
    }
    
    println!("\n=== Most Distributed Symbols ===");
    sorted.sort_by(|a, b| b.crate_count.cmp(&a.crate_count));
    
    for (i, symbol) in sorted.iter().take(10).enumerate() {
        println!("{}. {} (in {} crates, {} total uses)",
                i + 1, symbol.symbol, symbol.crate_count, symbol.total_usage);
    }
    
    // Serde analysis
    let serde_symbols = ["Serialize", "Deserialize"];
    println!("\n=== Serde Usage Analysis ===");
    for &symbol in &serde_symbols {
        if let Some(stats) = symbols.get(symbol) {
            println!("🔧 {} (φ: {}, usage: {}, complexity: {:.0})",
                    symbol, stats.phi_value, stats.total_usage, stats.complexity);
        }
    }
    
    println!("\n✨ Fast analysis complete!");
}
