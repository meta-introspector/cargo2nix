// Cargo Crate Phi Sum Calculator
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
struct CratePhi {
    name: String,
    phi_value: u64,
    dependencies: Vec<String>,
    total_phi_sum: u64,
}

fn euler_phi(n: u64) -> u64 {
    if n <= 1 { return 1; }
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

fn analyze_crate_ast(code: &str) -> u64 {
    let struct_count = code.matches("struct ").count() as u32;
    let enum_count = code.matches("enum ").count() as u32;
    let fn_count = code.matches("fn ").count() as u32;
    let impl_count = code.matches("impl ").count() as u32;
    let trait_count = code.matches("trait ").count() as u32;
    let use_count = code.matches("use ").count() as u32;
    
    let binary_exp = (struct_count + impl_count) / 2;
    let ternary_exp = (enum_count + trait_count) / 3;
    let quinary_exp = fn_count / 5;
    
    let monster_element = (2_u64.pow(binary_exp.min(20)) + 
                          3_u64.pow(ternary_exp.min(15)) + 
                          5_u64.pow(quinary_exp.min(8)) +
                          11 * use_count as u64) % 196883;
    
    euler_phi(monster_element)
}

fn calculate_total_phi_sum(crate_name: &str, crates: &HashMap<String, CratePhi>, visited: &mut HashMap<String, u64>) -> u64 {
    if let Some(&cached) = visited.get(crate_name) {
        return cached;
    }
    
    if let Some(crate_info) = crates.get(crate_name) {
        let mut total = crate_info.phi_value;
        
        for dep in &crate_info.dependencies {
            total += calculate_total_phi_sum(dep, crates, visited);
        }
        
        visited.insert(crate_name.to_string(), total);
        total
    } else {
        0
    }
}

fn main() {
    println!("=== Cargo Crate Phi Sum Calculator ===");
    
    let mut crates = HashMap::new();
    
    // Simulate crate analysis
    crates.insert("serde".to_string(), CratePhi {
        name: "serde".to_string(),
        phi_value: analyze_crate_ast("struct Serialize; trait Deserialize; fn serialize(); impl Serialize for String;"),
        dependencies: vec![],
        total_phi_sum: 0,
    });
    
    crates.insert("tokio".to_string(), CratePhi {
        name: "tokio".to_string(), 
        phi_value: analyze_crate_ast("struct Runtime; enum Poll; fn spawn(); fn block_on(); use std::future::Future;"),
        dependencies: vec![],
        total_phi_sum: 0,
    });
    
    crates.insert("reqwest".to_string(), CratePhi {
        name: "reqwest".to_string(),
        phi_value: analyze_crate_ast("struct Client; enum Method; fn get(); fn post(); use serde::Serialize;"),
        dependencies: vec!["serde".to_string(), "tokio".to_string()],
        total_phi_sum: 0,
    });
    
    crates.insert("my_app".to_string(), CratePhi {
        name: "my_app".to_string(),
        phi_value: analyze_crate_ast("struct App; fn main(); use reqwest::Client; use serde::Deserialize;"),
        dependencies: vec!["reqwest".to_string(), "serde".to_string()],
        total_phi_sum: 0,
    });
    
    // Calculate total phi sums
    let mut visited = HashMap::new();
    for (name, _) in &crates {
        let total = calculate_total_phi_sum(name, &crates, &mut visited);
        println!("Crate '{}': phi_sum = {}", name, total);
    }
    
    println!("\n=== Dependency Phi Inheritance ===");
    println!("serde: φ = {} (no deps)", crates["serde"].phi_value);
    println!("tokio: φ = {} (no deps)", crates["tokio"].phi_value);
    println!("reqwest: φ = {} + serde({}) + tokio({}) = {}", 
             crates["reqwest"].phi_value,
             crates["serde"].phi_value,
             crates["tokio"].phi_value,
             visited["reqwest"]);
    println!("my_app: φ = {} + reqwest_total({}) + serde({}) = {}",
             crates["my_app"].phi_value,
             visited["reqwest"],
             crates["serde"].phi_value,
             visited["my_app"]);
    
    println!("\n✓ When you import a crate, you import its phi sum");
    println!("✓ Transitive dependency phi values accumulate");
}
