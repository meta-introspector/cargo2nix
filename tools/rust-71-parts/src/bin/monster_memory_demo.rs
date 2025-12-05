//! Demonstration of Content Addressable Memory with Monster Group Semantics

use rust_71_parts::content_addressable_memory::*;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let db_path = args.get(1).unwrap_or(&"./monster_memory.db".to_string()).clone();
    
    println!("🧬 Monster Group Content Addressable Memory Demo");
    println!("Database: {}", db_path);
    
    let mut memory = MonsterMemory::new(&db_path)?;
    
    // Store some Rust code snippets with Monster semantics
    let snippets = vec![
        ("trait Display { fn fmt(&self) -> String; }", "std/fmt.rs", 42, 10),
        ("fn main() { println!(\"Hello\"); }", "main.rs", 1, 0),
        ("const PI: f64 = 3.14159;", "math.rs", 15, 5),
        ("impl Clone for Vec<T> { fn clone(&self) -> Self { self.clone() } }", "vec.rs", 100, 20),
    ];
    
    println!("\n📦 Storing snippets with Monster coordinates...");
    let mut hashes = Vec::new();
    
    for (snippet, file, line, col) in snippets {
        let location = SourceLocation {
            file: file.to_string(),
            line,
            column: col,
        };
        
        let hash = memory.store(snippet, location)?;
        hashes.push(hash);
        
        println!("✓ Stored: {} -> {:x}", snippet, hash[0]);
    }
    
    println!("\n🔍 Retrieving by content hash...");
    for hash in &hashes {
        if let Some(node) = memory.get_by_hash(hash)? {
            println!("📍 Hash {:x}: {} (class: {}, factor: {})", 
                hash[0], 
                node.snippet,
                node.conjugacy_class,
                node.enum_vector.to_monster_factor()
            );
        }
    }
    
    println!("\n🎯 Testing semantic similarity search...");
    if let Some(first_node) = memory.get_by_hash(&hashes[0])? {
        let similar = memory.find_similar(&first_node.monster_coordinates, 1000.0);
        println!("Found {} similar nodes to: {}", similar.len(), first_node.snippet);
        
        for similar_node in similar {
            println!("  📊 Similar: {} (distance in Monster space)", similar_node.snippet);
        }
    }
    
    println!("\n🧮 Testing enum vector meta-programming...");
    let trait_vec = EnumVector::Trait(10);
    let func_vec = EnumVector::Function(20);
    let composed = trait_vec.compose(&func_vec);
    
    println!("Trait(10) ⊕ Function(20) = {:?}", composed);
    println!("Identity: {:?}", EnumVector::identity());
    println!("Inverse of Trait(10): {:?}", trait_vec.inverse());
    
    println!("\n✨ Monster Group Content Addressable Memory operational!");
    println!("Each snippet exists in 196,883-dimensional Monster space");
    println!("Enums as vectors enable constant-space meta-programming");
    
    Ok(())
}
