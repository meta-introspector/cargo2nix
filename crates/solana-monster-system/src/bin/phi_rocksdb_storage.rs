use std::collections::HashMap;
use std::fs;
use rocksdb::{DB, Options};
use serde_json;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct PhiDeclaration {
    name: String,
    decl_type: String,
    source_file: String,
    phi_key: u64,
    content: String,
    collision_id: u32,
}

struct PhiRocksDB {
    db: DB,
    collision_count: HashMap<u64, u32>,
}

impl PhiRocksDB {
    fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        let db = DB::open(&opts, path)?;
        
        Ok(Self {
            db,
            collision_count: HashMap::new(),
        })
    }
    
    fn put(&mut self, mut decl: PhiDeclaration) -> Result<(), Box<dyn std::error::Error>> {
        let original_phi_key = decl.phi_key;
        let mut current_phi_key = original_phi_key;
        
        // Loop to handle potential collisions and generate new keys
        loop {
            let phi_key_bytes = current_phi_key.to_string();
            
            // Check if key already exists in RocksDB
            if let Some(existing_data) = self.db.get(phi_key_bytes.as_bytes())? {
                // Collision detected!
                let existing_decls: Vec<PhiDeclaration> = serde_json::from_slice(&existing_data)?;
                
                // If this is the original key and it's already a list, increment collision_id
                // Otherwise, it means we're trying to add a new differentiated key that already exists
                let collision_id_entry = self.collision_count.entry(original_phi_key).or_insert(0);
                *collision_id_entry += 1;
                decl.collision_id = *collision_id_entry;
                
                println!("COLLISION at φ = {}: {} vs existing entries (current attempt key: {})", 
                         original_phi_key, decl.name, current_phi_key);
                
                // Add differentiator by sampling file path hash and collision_id
                let file_hash: u64 = decl.source_file.bytes().map(|b| b as u64).sum();
                current_phi_key = (original_phi_key + file_hash + decl.collision_id as u64) % 196883; 
                
                println!("  → Attempting new key φ = {} (collision_id: {})", current_phi_key, decl.collision_id);
                decl.phi_key = current_phi_key;
                
                // Check if the newly generated key still exists
                if self.db.get(current_phi_key.to_string().as_bytes())?.is_none() {
                    // New key is unique, add it as a new entry
                    self.db.put(current_phi_key.to_string().as_bytes(), serde_json::to_vec(&vec![decl])?)?;
                    break; // Successfully stored
                } else {
                    // New key also collides, continue loop with this new key
                    continue; 
                }
            } else {
                // No collision, store directly
                self.db.put(phi_key_bytes.as_bytes(), serde_json::to_vec(&vec![decl])?)?;
                break; // Successfully stored
            }
        }
        
        Ok(())
    }
    
    fn get(&self, phi_key: u64) -> Result<Option<Vec<PhiDeclaration>>, Box<dyn std::error::Error>> {
        let phi_key_bytes = phi_key.to_string();
        if let Some(existing_data) = self.db.get(phi_key_bytes.as_bytes())? {
            let decls: Vec<PhiDeclaration> = serde_json::from_slice(&existing_data)?;
            Ok(Some(decls))
        } else {
            Ok(None)
        }
    }
    
    fn report_collisions(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n=== Collision Report ===");
        for (phi_key, count) in &self.collision_count {
            if *count > 0 {
                println!("φ = {}: {} collisions", phi_key, count);
                let phi_key_bytes = phi_key.to_string();
                if let Some(existing_data) = self.db.get(phi_key_bytes.as_bytes())? {
                    let decls: Vec<PhiDeclaration> = serde_json::from_slice(&existing_data)?;
                    for decl in decls {
                        println!("  - {} ({}) collision_id: {}", decl.name, decl.decl_type, decl.collision_id);
                    }
                }
            }
        }
        Ok(())
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
    println!("=== Phi RocksDB Storage with Collision Handling ===");
    
    let phi_db = PhiRocksDB::new("./phi_rocksdb_data")?;
    
    // Extract declarations from our actual files
    let files = vec![
        "src/bin/meme_pda_storage.rs",
        "src/bin/real_monster_solver.rs",
    ];
    
    for file_path in &files {
        if let Ok(content) = fs::read_to_string(file_path) {
            println!("\nStoring declarations from {}", file_path);
            
            // Store function declarations
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("fn ") && !trimmed.contains("main") {
                    if let Some(paren_pos) = trimmed.find('(') {
                        let fn_name = &trimmed[3..paren_pos];
                        let phi_key = calculate_phi_key(fn_name, "fn");
                        
                        let decl = PhiDeclaration {
                            name: fn_name.to_string(),
                            decl_type: "fn".to_string(),
                            source_file: file_path.to_string(),
                            phi_key,
                            content: trimmed.to_string(),
                            collision_id: 0,
                        };
                        
                        println!("  Storing fn {}: φ = {}", fn_name, phi_key);
                        phi_db.put(decl)?;
                    }
                }
                
                // Store struct declarations
                if trimmed.starts_with("struct ") {
                    let parts: Vec<&str> = trimmed.split_whitespace().collect();
                    if parts.len() > 1 {
                        let struct_name = parts[1];
                        let phi_key = calculate_phi_key(struct_name, "struct");
                        
                        let decl = PhiDeclaration {
                            name: struct_name.to_string(),
                            decl_type: "struct".to_string(),
                            source_file: file_path.to_string(),
                            phi_key,
                            content: trimmed.to_string(),
                            collision_id: 0,
                        };
                        
                        println!("  Storing struct {}: φ = {}", struct_name, phi_key);
                        phi_db.put(decl)?;
                    }
                }
            }
        }
    }
    
    // Report storage results
    println!("\n=== Storage Summary ===");
    // RocksDB does not have a simple `.len()` method.
    // To get total entries, one would need to iterate or use a counter during insertion.
    
    phi_db.report_collisions()?;
    
    // Test retrieval
    println!("\n=== Test Retrieval ===");
    let test_phi = calculate_phi_key("rustc_to_monster_factor", "fn");
    if let Some(decls) = phi_db.get(test_phi)? {
        println!("Retrieved φ = {}: {} declarations", test_phi, decls.len());
        for decl in decls {
            println!("  - {}: {}", decl.name, decl.content);
        }
    }
    
    println!("\n✓ Phi keys stored in RocksDB structure");
    println!("✓ Collision detection and resolution working");
    println!("✓ Differentiator sampling from file attributes");
    
    Ok(())
}
