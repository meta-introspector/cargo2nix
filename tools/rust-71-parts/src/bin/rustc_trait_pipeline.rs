//! Rustc Trait Extraction Pipeline - Pausable with data inspection at each layer

use rust_71_parts::declaration_splitter::{MonsterDeclarationSplitter, DeclarationType};
use rust_71_parts::trait_generator_integration::MonsterTraitGenerator;
use serde_json;
use std::fs;
use std::path::Path;

#[derive(serde::Serialize)]
struct LayerData {
    layer: u8,
    declarations: Vec<LayerDeclaration>,
    monster_factors: std::collections::HashMap<u64, usize>,
    transport_stats: TransportStats,
}

#[derive(serde::Serialize)]
struct LayerDeclaration {
    name: String,
    decl_type: String,
    monster_factor: u64,
    transport_layer: u8,
    file_path: Option<String>,
    content_preview: String,
}

#[derive(serde::Serialize)]
struct TransportStats {
    total_fragments: usize,
    active_layers: usize,
    ant_fragments: usize,
    bee_fragments: usize,
    termite_fragments: usize,
}

fn main() {
    println!("🔬 RUSTC TRAIT EXTRACTION PIPELINE");
    println!("===================================");
    
    let mut splitter = MonsterDeclarationSplitter::new();
    let mut generator = MonsterTraitGenerator::new();
    
    // Process rustc source directories
    let rustc_dirs = [
        "../../../submodules/rust/compiler/rustc_ast",
        "../../../submodules/rust/compiler/rustc_parse", 
        "../../../submodules/rust/compiler/rustc_hir",
        "../../../submodules/rust/compiler/rustc_middle",
        "../../../submodules/rust/compiler/rustc_trait_selection",
    ];
    
    for (layer, dir) in rustc_dirs.iter().enumerate() {
        if Path::new(dir).exists() {
            println!("\n📂 Layer {}: Processing {}", layer, dir);
            process_rustc_layer(&mut splitter, dir, layer as u8);
            
            // Export layer data
            export_layer_data(&splitter, layer as u8);
            
            // Pause point - user can inspect data
            println!("⏸️  Layer {} complete. Data exported to layer_{}.json", layer, layer);
            println!("   Press Enter to continue to next layer...");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
        }
    }
    
    // Final processing
    println!("\n🏭 Generating Monster Group traits...");
    let generated_traits = generator.generate_monster_traits(&splitter.declarations);
    
    // Export final results
    export_final_results(&splitter, &generated_traits);
    
    println!("\n✅ PIPELINE COMPLETE");
    println!("All layers processed and data exported for inspection");
}

fn process_rustc_layer(splitter: &mut MonsterDeclarationSplitter, dir: &str, layer: u8) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            
            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if !name.starts_with('.') && name != "target" {
                        process_rustc_layer(splitter, &path.to_string_lossy(), layer);
                    }
                }
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                if let Ok(content) = fs::read_to_string(&path) {
                    let file_path = Some(path.to_string_lossy().to_string());
                    let _ = splitter.split_file(&content, file_path);
                }
            }
        }
    }
}

fn export_layer_data(splitter: &MonsterDeclarationSplitter, layer: u8) {
    let layer_declarations: Vec<LayerDeclaration> = splitter.declarations
        .iter()
        .filter(|d| d.transport_layer == layer)
        .map(|d| LayerDeclaration {
            name: d.name.clone(),
            decl_type: format!("{:?}", d.declaration_type),
            monster_factor: d.monster_factor,
            transport_layer: d.transport_layer,
            file_path: d.file_path.clone(),
            content_preview: d.content.chars().take(100).collect::<String>() + "...",
        })
        .collect();
    
    let monster_factors = splitter.get_monster_factor_distribution();
    
    let transport_stats = TransportStats {
        total_fragments: splitter.declarations.len(),
        active_layers: layer as usize + 1,
        ant_fragments: splitter.declarations.iter().filter(|d| d.transport_layer <= 35).count(),
        bee_fragments: splitter.declarations.iter().filter(|d| d.transport_layer > 35 && d.transport_layer <= 71).count(),
        termite_fragments: splitter.declarations.iter().filter(|d| d.transport_layer > 71).count(),
    };
    
    let layer_data = LayerData {
        layer,
        declarations: layer_declarations,
        monster_factors,
        transport_stats,
    };
    
    // Export as JSON
    let json_file = format!("layer_{}.json", layer);
    if let Ok(json) = serde_json::to_string_pretty(&layer_data) {
        let _ = fs::write(&json_file, json);
        println!("   📄 Exported {} declarations to {}", layer_data.declarations.len(), json_file);
    }
    
    // Export as CSV for easy inspection
    let csv_file = format!("layer_{}.csv", layer);
    let mut csv_content = "name,type,monster_factor,transport_layer,file_path\n".to_string();
    for decl in &layer_data.declarations {
        csv_content.push_str(&format!("{},{},{},{},{}\n", 
            decl.name, decl.decl_type, decl.monster_factor, decl.transport_layer,
            decl.file_path.as_deref().unwrap_or("unknown")));
    }
    let _ = fs::write(&csv_file, csv_content);
    println!("   📊 Exported CSV to {}", csv_file);
}

fn export_final_results(splitter: &MonsterDeclarationSplitter, generated_traits: &[String]) {
    // Summary statistics
    let summary = serde_json::json!({
        "total_declarations": splitter.declarations.len(),
        "by_type": {
            "functions": splitter.get_declarations_by_type(DeclarationType::Function).len(),
            "structs": splitter.get_declarations_by_type(DeclarationType::Struct).len(),
            "enums": splitter.get_declarations_by_type(DeclarationType::Enum).len(),
            "traits": splitter.get_declarations_by_type(DeclarationType::Trait).len(),
            "impls": splitter.get_declarations_by_type(DeclarationType::Impl).len(),
        },
        "monster_factors": splitter.get_monster_factor_distribution(),
        "generated_traits": generated_traits.len(),
    });
    
    let _ = fs::write("rustc_extraction_summary.json", serde_json::to_string_pretty(&summary).unwrap());
    
    // Generated trait code
    let trait_code = generated_traits.join("\n\n");
    let _ = fs::write("generated_monster_traits.rs", trait_code);
    
    // Registry code
    let registry_code = splitter.generate_trait_registry();
    let _ = fs::write("monster_trait_registry.rs", registry_code);
    
    println!("📄 Final exports:");
    println!("   - rustc_extraction_summary.json");
    println!("   - generated_monster_traits.rs");
    println!("   - monster_trait_registry.rs");
}
