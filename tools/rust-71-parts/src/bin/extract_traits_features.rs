//! Binary to extract traits and features using AST transport system

use rust_71_parts::trait_feature_extractor::TraitFeatureExtractor;
use std::fs;
use std::path::Path;

fn main() {
    println!("🔍 TRAIT & FEATURE EXTRACTOR");
    println!("Using AST Transport System with Monster Group Classification");
    println!("============================================================");
    
    let mut extractor = TraitFeatureExtractor::new();
    let mut total_files = 0;
    let mut processed_files = 0;
    
    // Extract from key directories
    let directories = [
        "../../../submodules",
        "../../../tools", 
        "../../../crates",
        "../../../rust-bootstrap-core/src",
    ];
    
    for dir in &directories {
        if Path::new(dir).exists() {
            println!("\n📂 Processing directory: {}", dir);
            let (files, processed) = process_directory(dir, &mut extractor);
            total_files += files;
            processed_files += processed;
        }
    }
    
    println!("\n📊 EXTRACTION RESULTS:");
    println!("======================");
    println!("Files scanned: {}", total_files);
    println!("Files processed: {}", processed_files);
    println!("Traits extracted: {}", extractor.get_traits().len());
    println!("Features extracted: {}", extractor.get_features().len());
    
    // Show transport statistics
    let stats = extractor.get_transport_stats();
    println!("\n🚚 TRANSPORT STATISTICS:");
    println!("========================");
    println!("Total fragments: {}", stats.total_fragments);
    println!("Active layers: {}/108", stats.active_layers);
    println!("Ant layers (0-35): {} fragments", 
             stats.fragments_per_layer[0..36].iter().sum::<usize>());
    println!("Bee layers (36-71): {} fragments", 
             stats.fragments_per_layer[36..72].iter().sum::<usize>());
    println!("Termite layers (72-107): {} fragments", 
             stats.fragments_per_layer[72..108].iter().sum::<usize>());
    
    // Generate registry code
    println!("\n🎭 GENERATING TRAIT REGISTRY:");
    println!("=============================");
    let trait_registry = extractor.generate_trait_registry();
    println!("{}", trait_registry);
    
    println!("\n🚩 GENERATING FEATURE REGISTRY:");
    println!("===============================");
    let feature_registry = extractor.generate_feature_registry();
    println!("{}", feature_registry);
    
    // Show sample extractions
    if !extractor.get_traits().is_empty() {
        println!("\n🔬 SAMPLE TRAITS:");
        for (i, trait_info) in extractor.get_traits().iter().take(5).enumerate() {
            println!("  {}. {} (signature: {}, layer: {})", 
                     i + 1, trait_info.name, trait_info.monster_signature, trait_info.layer);
        }
    }
    
    if !extractor.get_features().is_empty() {
        println!("\n🔬 SAMPLE FEATURES:");
        for (i, feature_info) in extractor.get_features().iter().take(5).enumerate() {
            println!("  {}. {} (factor: {}, layer: {})", 
                     i + 1, feature_info.name, feature_info.monster_factor, feature_info.layer);
        }
    }
    
    println!("\n✅ EXTRACTION COMPLETE");
    println!("Monster Group classification applied to all extracted items");
}

fn process_directory(dir: &str, extractor: &mut TraitFeatureExtractor) -> (usize, usize) {
    let mut total_files = 0;
    let mut processed_files = 0;
    
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            
            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if !name.starts_with('.') && name != "target" {
                        let (sub_total, sub_processed) = process_directory(&path.to_string_lossy(), extractor);
                        total_files += sub_total;
                        processed_files += sub_processed;
                    }
                }
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                total_files += 1;
                
                if let Ok(content) = fs::read_to_string(&path) {
                    // Only process files that likely contain traits or features
                    if content.contains("trait ") || content.contains("#[cfg(feature") || content.contains("cfg!(feature") {
                        if let Ok(()) = extractor.extract_from_source(&content) {
                            processed_files += 1;
                        }
                    }
                }
            }
        }
    }
    
    (total_files, processed_files)
}
