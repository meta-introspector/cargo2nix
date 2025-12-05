// Per-Declaration Phi Sum Calculator
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct DeclPhi {
    name: String,
    decl_type: String,
    phi_value: u64,
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

fn calculate_decl_phi(decl_name: &str, decl_type: &str) -> u64 {
    let name_hash = decl_name.bytes().map(|b| b as u64).sum::<u64>();
    let type_factor = match decl_type {
        "struct" => 2,
        "enum" => 3,
        "fn" => 5,
        "trait" => 7,
        "impl" => 11,
        "const" => 13,
        "static" => 17,
        "mod" => 19,
        "use" => 23,
        _ => 1,
    };

    let monster_element = (name_hash * type_factor + 71) % 196883;
    euler_phi(monster_element)
}

fn main() {
    println!("=== Per-Declaration Phi Sum Calculator ===");

    // Serde crate declarations
    let serde_decls = vec![
        ("Serialize", "trait"),
        ("Deserialize", "trait"),
        ("serialize", "fn"),
        ("deserialize", "fn"),
        ("SerializeStruct", "struct"),
    ];

    // Tokio crate declarations
    let tokio_decls = vec![
        ("Runtime", "struct"),
        ("spawn", "fn"),
        ("block_on", "fn"),
        ("Future", "trait"),
        ("Poll", "enum"),
    ];

    // My app declarations
    let my_app_decls = vec![
        ("App", "struct"),
        ("main", "fn"),
        ("config", "const"),
        ("std::collections::HashMap", "use"),
        ("serde::Serialize", "use"),
        ("tokio::spawn", "use"),
    ];

    let mut all_decls = HashMap::new();

    // Calculate phi for each declaration
    println!("\n=== Serde Crate Declarations ===");
    let mut serde_total = 0;
    for (name, decl_type) in &serde_decls {
        let phi = calculate_decl_phi(name, decl_type);
        println!("{} {}: φ = {}", decl_type, name, phi);
        all_decls.insert(format!("serde::{}", name), phi);
        serde_total += phi;
    }
    println!("Serde total: φ = {}", serde_total);

    println!("\n=== Tokio Crate Declarations ===");
    let mut tokio_total = 0;
    for (name, decl_type) in &tokio_decls {
        let phi = calculate_decl_phi(name, decl_type);
        println!("{} {}: φ = {}", decl_type, name, phi);
        all_decls.insert(format!("tokio::{}", name), phi);
        tokio_total += phi;
    }
    println!("Tokio total: φ = {}", tokio_total);

    println!("\n=== My App Declarations ===");
    let mut app_total = 0;
    let mut imported_phi = 0;

    for (name, decl_type) in &my_app_decls {
        let phi = calculate_decl_phi(name, decl_type);
        println!("{} {}: φ = {}", decl_type, name, phi);

        if decl_type == &"use" {
            // When importing, add the phi of the imported declaration
            if name.starts_with("serde::") {
                let imported_name = format!("serde::{}", name.strip_prefix("serde::").unwrap());
                if let Some(&import_phi) = all_decls.get(&imported_name) {
                    imported_phi += import_phi;
                    println!("  → imports φ = {}", import_phi);
                }
            }
            if name.starts_with("tokio::") {
                let imported_name = format!("tokio::{}", name.strip_prefix("tokio::").unwrap());
                if let Some(&import_phi) = all_decls.get(&imported_name) {
                    imported_phi += import_phi;
                    println!("  → imports φ = {}", import_phi);
                }
            }
        }

        app_total += phi;
    }

    let total_app_phi = app_total + imported_phi;
    println!("My app own: φ = {}", app_total);
    println!("Imported: φ = {}", imported_phi);
    println!("Total: φ = {}", total_app_phi);

    println!("\n✓ Each declaration has its own phi value");
    println!("✓ Importing a declaration imports its phi");
    println!("✓ Total complexity = own phi + imported phi");
}
