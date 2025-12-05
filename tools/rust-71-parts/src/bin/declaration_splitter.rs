//! Declaration Splitter with Monster Group Transport

use rust_71_parts::declaration_splitter::{MonsterDeclarationSplitter, DeclarationType};
use std::fs;
use std::path::Path;

fn main() {
    println!("🔪 MONSTER GROUP DECLARATION SPLITTER");
    println!("=====================================");
    
    let mut splitter = MonsterDeclarationSplitter::new();
    let mut total_files = 0;
    let mut processed_files = 0;
    
    // Process key directories
    let directories = [
        "../../../rust-bootstrap-core/src",
        "../../../tools",
        "../../../crates",
        "/mnt/data1/nix/time/2025/06/01/solfunmeme-dioxus/src",
    ];
    
    for dir in &directories {
        if Path::new(dir).exists() {
            println!("\n📂 Processing: {}", dir);
            let (files, processed) = process_directory(dir, &mut splitter);
            total_files += files;
            processed_files += processed;
        }
    }
    
    println!("\n📊 SPLITTING RESULTS:");
    println!("====================");
    println!("Files scanned: {}", total_files);
    println!("Files processed: {}", processed_files);
    println!("Total declarations: {}", splitter.declarations.len());
    
    // Show breakdown by type
    for decl_type in [
        DeclarationType::Function,
        DeclarationType::Struct, 
        DeclarationType::Enum,
        DeclarationType::Trait,
        DeclarationType::Impl,
    ] {
        let count = splitter.get_declarations_by_type(decl_type.clone()).len();
        println!("{:?}: {}", decl_type, count);
    }
    
    // Show Monster Group factor distribution
    println!("\n🧮 MONSTER FACTOR DISTRIBUTION:");
    println!("===============================");
    let distribution = splitter.get_monster_factor_distribution();
    let mut sorted_factors: Vec<_> = distribution.iter().collect();
    sorted_factors.sort_by_key(|(factor, _)| *factor);
    
    for (factor, count) in sorted_factors.iter().take(10) {
        println!("Factor {}: {} declarations", factor, count);
    }
    
    // Generate trait registry
    println!("\n🎭 TRAIT REGISTRY:");
    println!("==================");
    let trait_registry = splitter.generate_trait_registry();
    println!("{}", trait_registry);
    
    // Show sample declarations
    println!("\n🔬 SAMPLE DECLARATIONS:");
    println!("=======================");
    for (i, decl) in splitter.declarations.iter().take(5).enumerate() {
        println!("{}. {} ({:?}) - Factor: {}, Layer: {}", 
                 i + 1, decl.name, decl.declaration_type, 
                 decl.monster_factor, decl.transport_layer);
    }
    
    println!("\n✅ DECLARATION SPLITTING COMPLETE");
    println!("All declarations classified with Monster Group factors");
}

fn process_directory(dir: &str, splitter: &mut MonsterDeclarationSplitter) -> (usize, usize) {
    let mut total_files = 0;
    let mut processed_files = 0;
    
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            
            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if !name.starts_with('.') && name != "target" {
                        let (sub_total, sub_processed) = process_directory(&path.to_string_lossy(), splitter);
                        total_files += sub_total;
                        processed_files += sub_processed;
                    }
                }
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                total_files += 1;
                
                if let Ok(content) = fs::read_to_string(&path) {
                    let file_path = Some(path.to_string_lossy().to_string());
                    if splitter.split_file(&content, file_path).is_ok() {
                        processed_files += 1;
                    }
                }
            }
        }
    }
    
    (total_files, processed_files)
}
