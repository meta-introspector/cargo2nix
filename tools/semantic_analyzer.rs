use std::fs;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct RepoSemantics {
    name: String,
    category: String,
    purpose: String,
    language: String,
    priority: u8,
}

fn analyze_semantics(name: &str) -> RepoSemantics {
    let category = match name {
        // Core Rust
        "rust" | "rustc" | "rustfmt" | "rust-clippy" => "rust-core",
        n if n.starts_with("rust-") => "rust-ecosystem",
        
        // Build & Package Management
        "cargo" | "cargo2nix" | "cmake-rs" | "cc-rs" => "build-tools",
        
        // Async & Concurrency
        "tokio" | "async-std" | "futures-rs" | "crossbeam" | "rayon" => "async-concurrency",
        
        // Serialization & Data
        "serde" | "bincode" | "toml" | "json" | "yaml-rust2" => "serialization",
        
        // Cryptography & Security
        n if n.contains("crypto") || n.contains("hash") || n.contains("tls") => "crypto-security",
        "ring" | "rustls" | "openssl" => "crypto-security",
        
        // Networking & HTTP
        "hyper" | "reqwest" | "h2" | "http" | "warp" => "networking",
        
        // CLI & Terminal
        "clap" | "console" | "termcolor" | "crossterm" => "cli-terminal",
        
        // Testing & Development
        "criterion" | "proptest" | "quickcheck" | "trybuild" => "testing-dev",
        
        // System & OS
        "libc" | "nix" | "winapi-rs" | "windows-rs" => "system-os",
        
        // Memory & Allocation
        n if n.contains("alloc") || n.contains("arena") => "memory-alloc",
        
        // Parsing & Text
        "nom" | "regex" | "unicode-" => "parsing-text",
        
        // Graphics & UI
        "wasm-bindgen" | "gloo" => "web-ui",
        
        // Math & Algorithms
        "num-" | "rand" => "math-algorithms",
        
        _ => "utility"
    };
    
    let purpose = match name {
        "rust" => "Rust programming language compiler and standard library".to_string(),
        "cargo" => "Rust package manager and build system".to_string(),
        "serde" => "Serialization framework for Rust".to_string(),
        "tokio" => "Asynchronous runtime for Rust".to_string(),
        "clap" => "Command line argument parser".to_string(),
        "hyper" => "HTTP implementation for Rust".to_string(),
        "regex" => "Regular expression engine".to_string(),
        _ => infer_purpose(name, category)
    };
    
    let language = if name.starts_with("rust-") || 
                     ["serde", "tokio", "clap", "hyper", "regex"].contains(&name) {
        "rust"
    } else if name.contains("js") || name.contains("node") {
        "javascript"
    } else if name.contains("py") || name.contains("python") {
        "python"
    } else {
        "mixed"
    };
    
    let priority = match category {
        "rust-core" => 1,
        "build-tools" => 1,
        "async-concurrency" | "serialization" => 2,
        "networking" | "crypto-security" => 2,
        "cli-terminal" | "testing-dev" => 3,
        _ => 4
    };
    
    RepoSemantics {
        name: name.to_string(),
        category: category.to_string(),
        purpose: purpose.to_string(),
        language: language.to_string(),
        priority,
    }
}

fn infer_purpose(name: &str, category: &str) -> String {
    match category {
        "async-concurrency" => format!("{} - Asynchronous programming support", name),
        "serialization" => format!("{} - Data serialization/deserialization", name),
        "crypto-security" => format!("{} - Cryptographic operations", name),
        "networking" => format!("{} - Network communication", name),
        "cli-terminal" => format!("{} - Command line interface", name),
        "testing-dev" => format!("{} - Testing and development tools", name),
        "system-os" => format!("{} - System and OS integration", name),
        "parsing-text" => format!("{} - Text parsing and processing", name),
        _ => format!("{} - Utility library", name)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string("unique_repos.txt")?;
    let mut semantics = Vec::new();
    let mut categories = HashMap::new();
    
    for line in content.lines() {
        let repo = analyze_semantics(line.trim());
        *categories.entry(repo.category.clone()).or_insert(0) += 1;
        semantics.push(repo);
    }
    
    // Sort by priority then name
    semantics.sort_by(|a, b| a.priority.cmp(&b.priority).then(a.name.cmp(&b.name)));
    
    println!("Repository Semantic Analysis:");
    println!("============================");
    
    for (category, count) in &categories {
        println!("{}: {} repos", category, count);
    }
    
    println!("\nHigh Priority Repositories (1-2):");
    for repo in semantics.iter().filter(|r| r.priority <= 2) {
        println!("  {} [{}] - {}", repo.name, repo.category, repo.purpose);
    }
    
    // Export as JSON-like format
    let mut output = String::new();
    output.push_str("{\n");
    for (i, repo) in semantics.iter().enumerate() {
        if i > 0 { output.push_str(",\n"); }
        output.push_str(&format!(
            "  \"{}\": {{\"category\": \"{}\", \"purpose\": \"{}\", \"language\": \"{}\", \"priority\": {}}}",
            repo.name, repo.category, repo.purpose, repo.language, repo.priority
        ));
    }
    output.push_str("\n}");
    
    fs::write("repo_semantics.json", output)?;
    println!("\nSemantics saved to repo_semantics.json");
    
    Ok(())
}
