#!/usr/bin/env rust-script

//! Extract traits and feature flags from Rust codebase
//! Integrates with Monster Group classification system

use std::fs;
use std::path::Path;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct TraitInfo {
    name: String,
    file: String,
    line: usize,
    methods: Vec<String>,
    monster_signature: u32,
}

#[derive(Debug, Clone)]
struct FeatureInfo {
    name: String,
    file: String,
    condition: String,
    monster_factor: u32,
}

fn main() {
    println!("🔍 EXTRACTING TRAITS AND FEATURES");
    println!("==================================");
    
    let mut traits = Vec::new();
    let mut features = Vec::new();
    
    // Extract from main codebase
    extract_from_directory("../submodules", &mut traits, &mut features);
    extract_from_directory("../tools", &mut traits, &mut features);
    extract_from_directory("../crates", &mut traits, &mut features);
    
    println!("\n📊 EXTRACTION RESULTS:");
    println!("Traits found: {}", traits.len());
    println!("Features found: {}", features.len());
    
    // Classify with Monster Group
    classify_traits(&mut traits);
    classify_features(&mut features);
    
    // Generate integration code
    generate_trait_integration(&traits);
    generate_feature_integration(&features);
    
    println!("\n✅ EXTRACTION COMPLETE");
}

fn extract_from_directory(dir: &str, traits: &mut Vec<TraitInfo>, features: &mut Vec<FeatureInfo>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if !name.starts_with('.') {
                        extract_from_directory(&path.to_string_lossy(), traits, features);
                    }
                }
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                extract_from_file(&path, traits, features);
            }
        }
    }
}

fn extract_from_file(path: &Path, traits: &mut Vec<TraitInfo>, features: &mut Vec<FeatureInfo>) {
    if let Ok(content) = fs::read_to_string(path) {
        let file_str = path.to_string_lossy().to_string();
        
        // Extract traits
        for (line_num, line) in content.lines().enumerate() {
            let line = line.trim();
            
            // Find trait definitions
            if line.starts_with("trait ") || line.contains(" trait ") {
                if let Some(trait_name) = extract_trait_name(line) {
                    let methods = extract_trait_methods(&content, line_num);
                    traits.push(TraitInfo {
                        name: trait_name,
                        file: file_str.clone(),
                        line: line_num + 1,
                        methods,
                        monster_signature: 0, // Will be assigned later
                    });
                }
            }
            
            // Find feature flags
            if line.contains("#[cfg(feature") || line.contains("cfg!(feature") {
                if let Some(feature_name) = extract_feature_name(line) {
                    features.push(FeatureInfo {
                        name: feature_name,
                        file: file_str.clone(),
                        condition: line.to_string(),
                        monster_factor: 0, // Will be assigned later
                    });
                }
            }
        }
    }
}

fn extract_trait_name(line: &str) -> Option<String> {
    // Simple trait name extraction
    if let Some(start) = line.find("trait ") {
        let after_trait = &line[start + 6..];
        if let Some(end) = after_trait.find(|c: char| c.is_whitespace() || c == '<' || c == '{') {
            Some(after_trait[..end].to_string())
        } else {
            Some(after_trait.to_string())
        }
    } else {
        None
    }
}

fn extract_trait_methods(content: &str, trait_line: usize) -> Vec<String> {
    let lines: Vec<&str> = content.lines().collect();
    let mut methods = Vec::new();
    let mut in_trait = false;
    let mut brace_count = 0;
    
    for (i, line) in lines.iter().enumerate().skip(trait_line) {
        let line = line.trim();
        
        if i == trait_line {
            in_trait = true;
            brace_count += line.matches('{').count();
            brace_count -= line.matches('}').count();
            continue;
        }
        
        if in_trait {
            brace_count += line.matches('{').count();
            brace_count -= line.matches('}').count();
            
            // Extract method signatures
            if line.starts_with("fn ") {
                if let Some(method_name) = extract_method_name(line) {
                    methods.push(method_name);
                }
            }
            
            if brace_count == 0 {
                break;
            }
        }
    }
    
    methods
}

fn extract_method_name(line: &str) -> Option<String> {
    if let Some(start) = line.find("fn ") {
        let after_fn = &line[start + 3..];
        if let Some(end) = after_fn.find('(') {
            Some(after_fn[..end].trim().to_string())
        } else {
            None
        }
    } else {
        None
    }
}

fn extract_feature_name(line: &str) -> Option<String> {
    // Extract feature name from cfg conditions
    if let Some(start) = line.find("feature = \"") {
        let after_quote = &line[start + 11..];
        if let Some(end) = after_quote.find('"') {
            Some(after_quote[..end].to_string())
        } else {
            None
        }
    } else if let Some(start) = line.find("feature = ") {
        let after_eq = &line[start + 10..];
        if let Some(end) = after_eq.find(|c: char| c.is_whitespace() || c == ')' || c == ',') {
            Some(after_eq[..end].trim_matches('"').to_string())
        } else {
            None
        }
    } else {
        None
    }
}

fn classify_traits(traits: &mut [TraitInfo]) {
    // Monster Group classification for traits
    let monster_primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71];
    
    for (i, trait_info) in traits.iter_mut().enumerate() {
        // Assign Monster signature based on trait characteristics
        let complexity = trait_info.methods.len() + trait_info.name.len();
        let prime_index = (complexity + i) % monster_primes.len();
        trait_info.monster_signature = monster_primes[prime_index];
    }
}

fn classify_features(features: &mut [FeatureInfo]) {
    // Monster Group classification for features
    let monster_factors = [71, 59, 47, 41, 31, 29, 23, 19, 17, 13, 11, 7, 5, 3, 2];
    
    for (i, feature_info) in features.iter_mut().enumerate() {
        // Assign Monster factor based on feature usage
        let factor_index = i % monster_factors.len();
        feature_info.monster_factor = monster_factors[factor_index];
    }
}

fn generate_trait_integration(traits: &[TraitInfo]) {
    println!("\n🎭 TRAIT INTEGRATION CODE:");
    println!("=========================");
    
    // Group traits by Monster signature
    let mut signature_groups: HashMap<u32, Vec<&TraitInfo>> = HashMap::new();
    for trait_info in traits {
        signature_groups.entry(trait_info.monster_signature)
            .or_default()
            .push(trait_info);
    }
    
    println!("// Monster Group Trait Classification");
    println!("use std::collections::HashMap;");
    println!();
    println!("pub struct MonsterTraitRegistry {{");
    println!("    traits_by_signature: HashMap<u32, Vec<&'static str>>,");
    println!("}}");
    println!();
    println!("impl MonsterTraitRegistry {{");
    println!("    pub fn new() -> Self {{");
    println!("        let mut registry = Self {{");
    println!("            traits_by_signature: HashMap::new(),");
    println!("        }};");
    
    for (signature, group_traits) in signature_groups {
        if !group_traits.is_empty() {
            println!("        // Monster signature {} (prime factor)", signature);
            print!("        registry.traits_by_signature.insert({}, vec![", signature);
            for (i, trait_info) in group_traits.iter().enumerate() {
                if i > 0 { print!(", "); }
                print!("\"{}\"", trait_info.name);
            }
            println!("]);");
        }
    }
    
    println!("        registry");
    println!("    }}");
    println!("}}");
    
    // Show top traits by signature
    let mut sorted_signatures: Vec<_> = signature_groups.iter().collect();
    sorted_signatures.sort_by_key(|(sig, traits)| std::cmp::Reverse((**sig, traits.len())));
    
    println!("\n📈 TOP TRAIT SIGNATURES:");
    for (signature, group_traits) in sorted_signatures.iter().take(5) {
        println!("  Signature {}: {} traits", signature, group_traits.len());
        for trait_info in group_traits.iter().take(3) {
            println!("    - {} ({} methods)", trait_info.name, trait_info.methods.len());
        }
    }
}

fn generate_feature_integration(features: &[FeatureInfo]) {
    println!("\n🚩 FEATURE INTEGRATION CODE:");
    println!("============================");
    
    // Group features by Monster factor
    let mut factor_groups: HashMap<u32, Vec<&FeatureInfo>> = HashMap::new();
    for feature_info in features {
        factor_groups.entry(feature_info.monster_factor)
            .or_default()
            .push(feature_info);
    }
    
    println!("// Monster Group Feature Classification");
    println!("pub struct MonsterFeatureRegistry {{");
    println!("    features_by_factor: HashMap<u32, Vec<&'static str>>,");
    println!("}}");
    println!();
    println!("impl MonsterFeatureRegistry {{");
    println!("    pub fn new() -> Self {{");
    println!("        let mut registry = Self {{");
    println!("            features_by_factor: HashMap::new(),");
    println!("        }};");
    
    for (factor, group_features) in factor_groups {
        if !group_features.is_empty() {
            println!("        // Monster factor {} (supersingular prime)", factor);
            print!("        registry.features_by_factor.insert({}, vec![", factor);
            for (i, feature_info) in group_features.iter().enumerate() {
                if i > 0 { print!(", "); }
                print!("\"{}\"", feature_info.name);
            }
            println!("]);");
        }
    }
    
    println!("        registry");
    println!("    }}");
    println!("}}");
    
    // Show feature distribution
    let unique_features: HashSet<_> = features.iter().map(|f| &f.name).collect();
    println!("\n📊 FEATURE STATISTICS:");
    println!("  Total feature usages: {}", features.len());
    println!("  Unique features: {}", unique_features.len());
    println!("  Monster factors used: {}", factor_groups.len());
    
    // Show top features
    let mut feature_counts: HashMap<&String, usize> = HashMap::new();
    for feature in features {
        *feature_counts.entry(&feature.name).or_insert(0) += 1;
    }
    
    let mut sorted_features: Vec<_> = feature_counts.iter().collect();
    sorted_features.sort_by_key(|(_, count)| std::cmp::Reverse(**count));
    
    println!("\n🔝 TOP FEATURES:");
    for (feature_name, count) in sorted_features.iter().take(5) {
        println!("  {}: {} usages", feature_name, count);
    }
}
