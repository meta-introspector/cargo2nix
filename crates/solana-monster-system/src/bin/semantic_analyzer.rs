use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, serde::Serialize)] // Add serde::Serialize for JSON export
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
        n if n.starts_with("num-") || n.starts_with("rand") => "math-algorithms",

        _ => "utility",
    }
    .to_string();

    let purpose = match category.as_str() {
        "rust-core" => "Fundamental Rust language and toolchain components.",
        "rust-ecosystem" => "Libraries and tools extending Rust's core capabilities.",
        "build-tools" => "Tools for building, compiling, and managing Rust projects.",
        "async-concurrency" => "Libraries for asynchronous programming and parallel execution.",
        "serialization" => "Handling data serialization and deserialization.",
        "crypto-security" => "Cryptographic operations and security-related functionalities.",
        "networking" => "Network communication, HTTP clients and servers.",
        "cli-terminal" => "Building command-line interfaces and interacting with terminals.",
        "testing-dev" => "Tools and frameworks for testing and development workflows.",
        "system-os" => "Low-level operating system interactions and system programming.",
        "memory-alloc" => "Memory management and allocation strategies.",
        "parsing-text" => "Parsing, text processing, and regular expressions.",
        "web-ui" => "Web-related functionalities, WASM, and user interface components.",
        "math-algorithms" => "Mathematical operations and common algorithms.",
        _ => "General purpose utility library or component.",
    }
    .to_string();

    let language = "Rust".to_string(); // Assuming all repos are Rust for this context

    let priority = match category.as_str() {
        "rust-core" | "build-tools" => 1,
        "async-concurrency" | "serialization" | "crypto-security" | "networking" => 2,
        _ => 3,
    };

    RepoSemantics {
        name: name.to_string(),
        category,
        purpose,
        language,
        priority,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string("unique_repos.txt")?;
    let mut all_semantics: HashMap<String, RepoSemantics> = HashMap::new();

    for line in content.lines() {
        if !line.trim().is_empty() {
            let semantics = analyze_semantics(line.trim());
            all_semantics.insert(semantics.name.clone(), semantics);
        }
    }

    let json_output = serde_json::to_string_pretty(&all_semantics)?;
    fs::write("repo_semantics.json", json_output)?;

    println!(
        "Semantic analysis complete. {} repositories analyzed.",
        all_semantics.len()
    );
    println!("Output saved to repo_semantics.json");

    Ok(())
}
