//! Test pipeline with available code

use rust_71_parts::declaration_splitter::{MonsterDeclarationSplitter, DeclarationType};
use rust_71_parts::trait_generator_integration::MonsterTraitGenerator;
use serde_json;
use std::fs;

fn main() {
    println!("🔬 TRAIT EXTRACTION PIPELINE TEST");
    println!("==================================");
    
    let mut splitter = MonsterDeclarationSplitter::new();
    let mut generator = MonsterTraitGenerator::new();
    
    // Test with our own code
    let test_sources = [
        ("../src/declaration_splitter.rs", 0),
        ("../src/trait_feature_extractor.rs", 1), 
        ("../src/trait_generator_integration.rs", 2),
    ];
    
    for (file_path, layer) in test_sources {
        if let Ok(content) = fs::read_to_string(file_path) {
            println!("\n📂 Layer {}: Processing {}", layer, file_path);
            let _ = splitter.split_file(&content, Some(file_path.to_string()));
            
            // Export layer data
            export_layer_data(&splitter, layer, file_path);
        }
    }
    
    // Generate traits
    println!("\n🏭 Generating Monster Group traits...");
    let generated_traits = generator.generate_monster_traits(&splitter.declarations);
    
    // Export results
    export_results(&splitter, &generated_traits);
    
    println!("\n✅ PIPELINE TEST COMPLETE");
    println!("Extracted {} declarations across {} layers", 
             splitter.declarations.len(), test_sources.len());
}

fn export_layer_data(splitter: &MonsterDeclarationSplitter, layer: u8, file_path: &str) {
    let layer_data = serde_json::json!({
        "layer": layer,
        "file": file_path,
        "declarations": splitter.declarations.iter().map(|d| {
            serde_json::json!({
                "name": d.name,
                "type": format!("{:?}", d.declaration_type),
                "monster_factor": d.monster_factor,
                "transport_layer": d.transport_layer,
            })
        }).collect::<Vec<_>>(),
        "stats": {
            "total": splitter.declarations.len(),
            "traits": splitter.get_declarations_by_type(DeclarationType::Trait).len(),
            "structs": splitter.get_declarations_by_type(DeclarationType::Struct).len(),
            "functions": splitter.get_declarations_by_type(DeclarationType::Function).len(),
        }
    });
    
    let json_file = format!("test_layer_{}.json", layer);
    let _ = fs::write(&json_file, serde_json::to_string_pretty(&layer_data).unwrap());
    println!("   📄 Exported to {}", json_file);
}

fn export_results(splitter: &MonsterDeclarationSplitter, generated_traits: &[String]) {
    // Summary
    let summary = serde_json::json!({
        "total_declarations": splitter.declarations.len(),
        "by_type": {
            "traits": splitter.get_declarations_by_type(DeclarationType::Trait).len(),
            "structs": splitter.get_declarations_by_type(DeclarationType::Struct).len(),
            "functions": splitter.get_declarations_by_type(DeclarationType::Function).len(),
            "enums": splitter.get_declarations_by_type(DeclarationType::Enum).len(),
            "impls": splitter.get_declarations_by_type(DeclarationType::Impl).len(),
        },
        "monster_factors": splitter.get_monster_factor_distribution(),
        "generated_traits": generated_traits.len(),
    });
    
    let _ = fs::write("test_summary.json", serde_json::to_string_pretty(&summary).unwrap());
    
    // Generated code
    let trait_code = generated_traits.join("\n\n");
    let _ = fs::write("test_generated_traits.rs", trait_code);
    
    println!("📄 Exported: test_summary.json, test_generated_traits.rs");
}
