use std::collections::HashMap;

#[derive(Debug)]
struct Module {
    name: String,
    imports: Vec<u64>, // phi values of imported symbols
    exports: Vec<u64>, // phi values of exported symbols
    import_sum: u64,
    export_sum: u64,
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

fn extract_modules() -> Vec<Module> {
    vec![
        Module {
            name: "serde".to_string(),
            imports: vec![],
            exports: vec![
                phi_hash("Serialize"),
                phi_hash("Deserialize"),
                phi_hash("to_string"),
            ],
            import_sum: 0,
            export_sum: 0,
        },
        Module {
            name: "tokio".to_string(),
            imports: vec![],
            exports: vec![phi_hash("spawn"), phi_hash("Runtime"), phi_hash("main")],
            import_sum: 0,
            export_sum: 0,
        },
        Module {
            name: "app1".to_string(),
            imports: vec![phi_hash("Serialize"), phi_hash("spawn")],
            exports: vec![phi_hash("process"), phi_hash("handle")],
            import_sum: 0,
            export_sum: 0,
        },
        Module {
            name: "app2".to_string(),
            imports: vec![phi_hash("Deserialize"), phi_hash("Runtime")],
            exports: vec![phi_hash("execute"), phi_hash("manage")],
            import_sum: 0,
            export_sum: 0,
        },
        Module {
            name: "similar_app".to_string(),
            imports: vec![phi_hash("Serialize"), phi_hash("spawn")],
            exports: vec![phi_hash("run"), phi_hash("control")],
            import_sum: 0,
            export_sum: 0,
        },
    ]
}

fn calculate_sums(modules: &mut [Module]) {
    for module in modules {
        module.import_sum = module.imports.iter().map(|&phi| euler_phi(phi)).sum();
        module.export_sum = module.exports.iter().map(|&phi| euler_phi(phi)).sum();
    }
}

fn find_matches(modules: &[Module]) {
    println!("=== Import/Export Phi Matching ===");

    for i in 0..modules.len() {
        for j in (i + 1)..modules.len() {
            let m1 = &modules[i];
            let m2 = &modules[j];

            // Import-Export matching
            let import_export_overlap = m1
                .imports
                .iter()
                .filter(|&imp| m2.exports.contains(imp))
                .count();

            let export_import_overlap = m1
                .exports
                .iter()
                .filter(|&exp| m2.imports.contains(exp))
                .count();

            // Similar imports
            let import_overlap = m1
                .imports
                .iter()
                .filter(|&imp| m2.imports.contains(imp))
                .count();

            // Similar exports
            let export_overlap = m1
                .exports
                .iter()
                .filter(|&exp| m2.exports.contains(exp))
                .count();

            // Phi sum similarity
            let import_sum_diff = if m1.import_sum > m2.import_sum {
                m1.import_sum - m2.import_sum
            } else {
                m2.import_sum - m1.import_sum
            };

            let export_sum_diff = if m1.export_sum > m2.export_sum {
                m1.export_sum - m2.export_sum
            } else {
                m2.export_sum - m1.export_sum
            };

            if import_export_overlap > 0
                || export_import_overlap > 0
                || import_overlap > 0
                || export_overlap > 0
                || import_sum_diff < 1000
                || export_sum_diff < 1000
            {
                println!("🔗 {} ↔ {}", m1.name, m2.name);
                if import_export_overlap > 0 {
                    println!("   📥→📤 Import-Export match: {}", import_export_overlap);
                }
                if export_import_overlap > 0 {
                    println!("   📤→📥 Export-Import match: {}", export_import_overlap);
                }
                if import_overlap > 0 {
                    println!("   📥📥 Similar imports: {}", import_overlap);
                }
                if export_overlap > 0 {
                    println!("   📤📤 Similar exports: {}", export_overlap);
                }
                println!("   🧮 Import φ-sum: {} vs {}", m1.import_sum, m2.import_sum);
                println!("   🧮 Export φ-sum: {} vs {}", m1.export_sum, m2.export_sum);
            }
        }
    }
}

fn main() {
    println!("=== Fast Phi Import/Export Matcher ===");

    let mut modules = extract_modules();
    calculate_sums(&mut modules);

    println!("📦 Modules analyzed:");
    for module in &modules {
        println!(
            "  {} (imports: {}, exports: {}, φ-sums: {}/{})",
            module.name,
            module.imports.len(),
            module.exports.len(),
            module.import_sum,
            module.export_sum
        );
    }

    find_matches(&modules);

    println!("\n✨ Fast phi matching complete!");
}
