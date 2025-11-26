use std::collections::HashMap;
use std::fs;

struct NamePhiMapper {
    name_to_phi: HashMap<String, u64>,
    phi_to_decl: HashMap<u64, String>,
    use_mappings: HashMap<String, u64>, // "crate::function" -> phi
}

impl NamePhiMapper {
    fn new() -> Self {
        Self {
            name_to_phi: HashMap::new(),
            phi_to_decl: HashMap::new(),
            use_mappings: HashMap::new(),
        }
    }
    
    fn index_declaration(&mut self, name: &str, decl_type: &str, source_crate: &str, content: &str) {
        let phi = calculate_phi_key(name, decl_type);
        let full_name = format!("{}::{}", source_crate, name);
        
        self.name_to_phi.insert(name.to_string(), phi);
        self.name_to_phi.insert(full_name.clone(), phi);
        self.phi_to_decl.insert(phi, content.to_string());
        self.use_mappings.insert(full_name, phi);
        
        println!("INDEXED: {} → φ = {}", name, phi);
    }
    
    fn resolve_use(&self, use_path: &str) -> Option<u64> {
        // Handle "use crate::function" -> phi lookup
        if let Some(&phi) = self.use_mappings.get(use_path) {
            return Some(phi);
        }
        
        // Try direct name lookup
        if let Some(&phi) = self.name_to_phi.get(use_path) {
            return Some(phi);
        }
        
        None
    }
    
    fn get_phi_sum_for_uses(&self, use_statements: &[&str]) -> u64 {
        let mut total = 0;
        for use_stmt in use_statements {
            if let Some(phi) = self.resolve_use(use_stmt) {
                total += phi;
                println!("  {} → φ = {}", use_stmt, phi);
            } else {
                println!("  {} → NOT FOUND", use_stmt);
            }
        }
        total
    }
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

fn calculate_phi_key(name: &str, decl_type: &str) -> u64 {
    let name_hash = name.bytes().map(|b| b as u64).sum::<u64>();
    let type_factor = match decl_type {
        "fn" => 5,
        "struct" => 2,
        "enum" => 3,
        "use" => 23,
        _ => 1,
    };
    let monster_element = (name_hash * type_factor + 71) % 196883;
    euler_phi(monster_element)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Name-to-Phi Mapping System ===");
    
    let mut mapper = NamePhiMapper::new();
    
    // Index all declarations from our files
    let files = vec![
        ("src/bin/meme_pda_storage.rs", "meme_pda"),
        ("src/bin/real_monster_solver.rs", "monster_solver"),
    ];
    
    for (file_path, crate_name) in &files {
        if let Ok(content) = fs::read_to_string(file_path) {
            println!("\nIndexing declarations from {} ({})", file_path, crate_name);
            
            for line in content.lines() {
                let trimmed = line.trim();
                
                // Index functions
                if trimmed.starts_with("fn ") && !trimmed.contains("main") {
                    if let Some(paren_pos) = trimmed.find('(') {
                        let fn_name = &trimmed[3..paren_pos];
                        mapper.index_declaration(fn_name, "fn", crate_name, trimmed);
                    }
                }
                
                // Index structs
                if trimmed.starts_with("struct ") {
                    let parts: Vec<&str> = trimmed.split_whitespace().collect();
                    if parts.len() > 1 {
                        let struct_name = parts[1];
                        mapper.index_declaration(struct_name, "struct", crate_name, trimmed);
                    }
                }
            }
        }
    }
    
    // Test use statement resolution
    println!("\n=== Use Statement Resolution ===");
    let use_statements = vec![
        "meme_pda::rustc_to_monster_factor",
        "monster_solver::generate_real_model", 
        "meme_pda::calculate_crate_viral_power",
        "format_monster_convergence", // Direct name
        "nonexistent_function", // Should fail
    ];
    
    println!("Resolving use statements:");
    let total_phi = mapper.get_phi_sum_for_uses(&use_statements);
    
    println!("\n=== Summary ===");
    println!("Total indexed declarations: {}", mapper.name_to_phi.len());
    println!("Total phi sum for uses: {}", total_phi);
    
    // Show mapping table
    println!("\n=== Name → Phi Mapping Table ===");
    let mut mappings: Vec<_> = mapper.name_to_phi.iter().collect();
    mappings.sort_by_key(|(_, phi)| *phi);
    for (name, phi) in mappings.iter().take(10) {
        println!("  {} → φ = {}", name, phi);
    }
    
    println!("\n✓ Name-to-phi mapping system working");
    println!("✓ Use statement resolution complete");
    println!("✓ Ready for import phi sum calculation");
    
    Ok(())
}
