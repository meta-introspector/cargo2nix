use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug)]
struct ToolProfile {
    name: String,
    functions: Vec<String>,
    imports: Vec<String>,
    prime_signature: Vec<u64>,
    complexity: u64,
    merge_candidates: Vec<String>,
}

fn get_primes() -> Vec<u64> {
    vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97,
    ]
}

fn hash_string(s: &str) -> u64 {
    let mut hash = 5381u64;
    for byte in s.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(byte as u64);
    }
    hash % 10000 + 100
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

fn analyze_tool(path: &Path) -> Option<ToolProfile> {
    let content = fs::read_to_string(path).ok()?;
    let name = path.file_stem()?.to_str()?.to_string();
    let primes = get_primes();

    let mut functions = Vec::new();
    let mut imports = Vec::new();
    let mut all_factors = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();

        // Extract functions
        if trimmed.starts_with("fn ") {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 2 {
                let func_name = parts[1].split('(').next().unwrap_or(parts[1]).to_string();
                functions.push(func_name.clone());

                // Add prime factors for function
                let func_hash = hash_string(&func_name);
                let factors = prime_factorize(func_hash, &primes);
                all_factors.extend(factors);
            }
        }

        // Extract imports
        if trimmed.starts_with("use ") {
            let import = trimmed[4..].trim_end_matches(';');
            imports.push(import.to_string());

            // Add prime factors for import
            let import_hash = hash_string(import);
            let factors = prime_factorize(import_hash, &primes);
            all_factors.extend(factors);
        }
    }

    all_factors.sort();
    all_factors.dedup();

    let complexity = all_factors
        .iter()
        .fold(1u64, |acc, &x| acc.saturating_mul(x));

    Some(ToolProfile {
        name,
        functions,
        imports,
        prime_signature: all_factors,
        complexity,
        merge_candidates: Vec::new(),
    })
}

fn find_merge_candidates(tools: &mut [ToolProfile]) {
    for i in 0..tools.len() {
        for j in (i + 1)..tools.len() {
            let common_primes = tools[i]
                .prime_signature
                .iter()
                .filter(|p| tools[j].prime_signature.contains(p))
                .count();

            let total_primes =
                (tools[i].prime_signature.len() + tools[j].prime_signature.len()) as f64;
            let similarity = if total_primes > 0.0 {
                (2.0 * common_primes as f64) / total_primes
            } else {
                0.0
            };

            if similarity > 0.8 {
                tools[i].merge_candidates.push(tools[j].name.clone());
                tools[j].merge_candidates.push(tools[i].name.clone());
            }
        }
    }
}

fn generate_merged_tool(tools: &[ToolProfile], merge_group: &[String]) -> String {
    let mut merged_functions = Vec::new();
    let mut merged_imports = Vec::new();

    for tool in tools {
        if merge_group.contains(&tool.name) {
            merged_functions.extend(tool.functions.clone());
            merged_imports.extend(tool.imports.clone());
        }
    }

    merged_imports.sort();
    merged_imports.dedup();
    merged_functions.sort();
    merged_functions.dedup();

    let merged_name = merge_group.join("_");

    format!(
        "// Merged tool: {}\n// Original tools: {:?}\n\n{}\n\n{}\n\nfn main() {{\n    println!(\"Merged tool combining: {:?}\");\n}}\n",
        merged_name,
        merge_group,
        merged_imports.iter().map(|i| format!("use {};", i)).collect::<Vec<_>>().join("\n"),
        merged_functions.iter().map(|f| format!("fn {}() {{ /* merged implementation */ }}", f)).collect::<Vec<_>>().join("\n"),
        merge_group
    )
}

fn scan_tools_directory() -> Vec<ToolProfile> {
    let mut tools = Vec::new();

    // Scan current directory for analyzer tools
    if let Ok(entries) = fs::read_dir("./src/bin") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "rs") {
                if let Some(name) = path.file_stem().and_then(|n| n.to_str()) {
                    if name.contains("analyzer")
                        || name.contains("matcher")
                        || name.contains("linker")
                    {
                        if let Some(tool) = analyze_tool(&path) {
                            tools.push(tool);
                        }
                    }
                }
            }
        }
    }

    tools
}

fn main() {
    println!("=== Monster Tool Merger ===");

    let mut tools = scan_tools_directory();
    println!("📦 Found {} analysis tools", tools.len());

    find_merge_candidates(&mut tools);

    println!("\n=== Tool Analysis ===");
    tools.sort_by(|a, b| b.complexity.cmp(&a.complexity));

    for tool in &tools {
        println!(
            "🔧 {} (complexity: {}, functions: {}, primes: {})",
            tool.name,
            tool.complexity,
            tool.functions.len(),
            tool.prime_signature.len()
        );

        if !tool.merge_candidates.is_empty() {
            println!("   🔗 Merge candidates: {:?}", tool.merge_candidates);
        }
    }

    // Find merge groups
    let mut merge_groups = Vec::new();
    let mut processed = Vec::new();

    for tool in &tools {
        if processed.contains(&tool.name) {
            continue;
        }

        if !tool.merge_candidates.is_empty() {
            let mut group = vec![tool.name.clone()];
            group.extend(tool.merge_candidates.clone());
            group.sort();
            group.dedup();

            merge_groups.push(group.clone());
            processed.extend(group);
        }
    }

    println!("\n=== Merge Recommendations ===");
    for (i, group) in merge_groups.iter().enumerate() {
        if group.len() > 1 {
            println!("{}. Merge Group: {:?}", i + 1, group);

            // Generate merged tool
            let merged_code = generate_merged_tool(&tools, group);
            let merged_filename = format!("merged_tool_{}.rs", i + 1);

            if let Err(e) = fs::write(&merged_filename, merged_code) {
                println!("   ❌ Failed to write {}: {}", merged_filename, e);
            } else {
                println!("   ✅ Generated {}", merged_filename);
            }
        }
    }

    // Show individual tools that don't need merging
    println!("\n=== Standalone Tools ===");
    for tool in &tools {
        if !processed.contains(&tool.name) {
            println!("🔧 {} (unique functionality)", tool.name);
        }
    }

    println!("\n✨ Monster tool merger complete!");
    println!("📊 Generated {} merged tools", merge_groups.len());
}
