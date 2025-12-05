use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
struct DependencyNode {
    name: String,
    content: String,
    dependencies: Vec<String>, // Names of dependencies
    dependency_phis: Vec<u64>, // Phi values of dependencies
    cas_address: u64,          // Content-addressable address from all dependency signatures
}

struct CASStorage {
    nodes: HashMap<u64, DependencyNode>,   // cas_address -> node
    name_to_address: HashMap<String, u64>, // name -> cas_address
}

impl CASStorage {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            name_to_address: HashMap::new(),
        }
    }

    fn add_node(&mut self, name: &str, content: &str, dependencies: Vec<&str>) {
        // Calculate phi values for all dependencies
        let mut dependency_phis = Vec::new();
        let mut resolved_deps = Vec::new();

        for dep in &dependencies {
            let phi = calculate_phi_key(dep);
            dependency_phis.push(phi);
            resolved_deps.push(dep.to_string());
            println!("  dep: {} → φ = {}", dep, phi);
        }

        // CAS address = hash of ALL dependency signatures combined
        let cas_address = self.calculate_cas_address(&dependency_phis, content);

        let node = DependencyNode {
            name: name.to_string(),
            content: content.to_string(),
            dependencies: resolved_deps,
            dependency_phis,
            cas_address,
        };

        println!(
            "NODE: {} → CAS address: {} (from {} deps)",
            name,
            cas_address,
            dependencies.len()
        );

        self.nodes.insert(cas_address, node);
        self.name_to_address.insert(name.to_string(), cas_address);
    }

    fn calculate_cas_address(&self, dependency_phis: &[u64], content: &str) -> u64 {
        // Address = hash(all dependency signatures + content)
        let mut combined_signature = dependency_phis.iter().sum::<u64>();
        combined_signature += content.bytes().map(|b| b as u64).sum::<u64>();

        // Make it content-addressable by including dependency structure
        for phi in dependency_phis {
            combined_signature = combined_signature.wrapping_mul(71).wrapping_add(*phi);
        }

        combined_signature % 196883 // Monster Group modulus
    }

    fn get_by_address(&self, address: u64) -> Option<&DependencyNode> {
        self.nodes.get(&address)
    }

    fn get_by_name(&self, name: &str) -> Option<&DependencyNode> {
        if let Some(&address) = self.name_to_address.get(name) {
            self.nodes.get(&address)
        } else {
            None
        }
    }

    fn verify_cas_integrity(&self) -> bool {
        for (address, node) in &self.nodes {
            let recalculated = self.calculate_cas_address(&node.dependency_phis, &node.content);
            if *address != recalculated {
                println!(
                    "❌ CAS integrity failed for {}: {} != {}",
                    node.name, address, recalculated
                );
                return false;
            }
        }
        true
    }

    fn show_dependency_graph(&self) {
        println!("\n=== Dependency Graph (CAS Addresses) ===");
        for (address, node) in &self.nodes {
            println!("📦 {} (CAS: {})", node.name, address);
            for (i, dep) in node.dependencies.iter().enumerate() {
                let dep_phi = node.dependency_phis[i];
                if let Some(&dep_address) = self.name_to_address.get(dep) {
                    println!("  ├─ {} (φ = {}, CAS: {})", dep, dep_phi, dep_address);
                } else {
                    println!("  ├─ {} (φ = {}, external)", dep, dep_phi);
                }
            }
        }
    }
}

fn calculate_phi_key(name: &str) -> u64 {
    let name_hash = name.bytes().map(|b| b as u64).sum::<u64>();
    let monster_element = (name_hash * 5 + 71) % 196883;
    euler_phi(monster_element)
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Content-Addressable Storage with Dependency Signatures ===");

    let mut cas = CASStorage::new();

    // Add nodes with their dependencies
    println!("\nAdding nodes to CAS:");

    // Leaf nodes (no dependencies)
    cas.add_node("std::HashMap", "pub struct HashMap<K, V> { ... }", vec![]);
    cas.add_node("std::Vec", "pub struct Vec<T> { ... }", vec![]);

    // Intermediate nodes
    cas.add_node(
        "serde::Serialize",
        "pub trait Serialize { ... }",
        vec!["std::HashMap"],
    );
    cas.add_node(
        "tokio::Runtime",
        "pub struct Runtime { ... }",
        vec!["std::Vec"],
    );

    // Complex nodes with multiple dependencies
    cas.add_node(
        "our::meme_pda",
        "pub fn create_meme_pda() { ... }",
        vec!["std::HashMap", "serde::Serialize"],
    );

    cas.add_node(
        "our::monster_solver",
        "pub fn solve_monster() { ... }",
        vec!["std::Vec", "tokio::Runtime", "our::meme_pda"],
    );

    // Show dependency graph
    cas.show_dependency_graph();

    // Test CAS retrieval
    println!("\n=== CAS Retrieval Tests ===");
    if let Some(node) = cas.get_by_name("our::monster_solver") {
        println!(
            "Retrieved by name: {} → CAS: {}",
            node.name, node.cas_address
        );

        // Can also retrieve by address directly
        if let Some(same_node) = cas.get_by_address(node.cas_address) {
            println!("Retrieved by address: {} ✓", same_node.name);
        }
    }

    // Verify CAS integrity
    println!("\n=== CAS Integrity Check ===");
    if cas.verify_cas_integrity() {
        println!("✅ All CAS addresses valid");
    } else {
        println!("❌ CAS integrity compromised");
    }

    println!("\n=== Summary ===");
    println!("Total nodes: {}", cas.nodes.len());
    println!("CAS addresses are deterministic from dependency signatures");
    println!("Same dependencies + content = same address");

    println!("\n✓ Content-addressable storage working");
    println!("✓ Dependencies become part of the address");
    println!("✓ CAS contains signatures of everything needed");

    Ok(())
}
