use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
struct CodeFragment {
    content: String,
    phi_signature: u64,
    functions: Vec<String>,
    source_file: String,
    line_range: (usize, usize),
}

#[derive(Debug, Clone)]
struct TraitLayer {
    name: String,
    level: usize,
    functions: Vec<String>,
    dependencies: Vec<String>,
    phi_signature: u64,
    minimal_addition: String,
}

#[derive(Debug)]
struct LatticeStructure {
    layers: Vec<TraitLayer>,
    consumed_code: Vec<CodeFragment>,
    similarity_graph: HashMap<u64, Vec<u64>>,
}

fn phi_hash(s: &str) -> u64 {
    let mut hash = 5381u64;
    for byte in s.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(byte as u64);
    }
    hash % 196883
}

fn extract_functions(content: &str) -> Vec<String> {
    let mut functions = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("fn ") {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 2 {
                let func_name = parts[1].split('(').next().unwrap_or(parts[1]).to_string();
                functions.push(func_name);
            }
        }
    }
    functions
}

fn find_code_fragments(dir: &Path) -> Vec<CodeFragment> {
    let mut fragments = Vec::new();
    
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                fragments.extend(find_code_fragments(&path));
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                if let Ok(content) = fs::read_to_string(&path) {
                    let functions = extract_functions(&content);
                    let phi_sig = phi_hash(&content);
                    
                    fragments.push(CodeFragment {
                        content: content.clone(),
                        phi_signature: phi_sig,
                        functions,
                        source_file: path.to_string_lossy().to_string(),
                        line_range: (1, content.lines().count()),
                    });
                }
            }
        }
    }
    
    fragments
}

fn find_similar_fragments(fragments: &[CodeFragment], threshold: u64) -> HashMap<u64, Vec<u64>> {
    let mut similarity_graph = HashMap::new();
    
    for i in 0..fragments.len() {
        for j in (i + 1)..fragments.len() {
            let phi_diff = if fragments[i].phi_signature > fragments[j].phi_signature {
                fragments[i].phi_signature - fragments[j].phi_signature
            } else {
                fragments[j].phi_signature - fragments[i].phi_signature
            };
            
            if phi_diff < threshold {
                similarity_graph.entry(fragments[i].phi_signature)
                    .or_insert_with(Vec::new)
                    .push(fragments[j].phi_signature);
                
                similarity_graph.entry(fragments[j].phi_signature)
                    .or_insert_with(Vec::new)
                    .push(fragments[i].phi_signature);
            }
        }
    }
    
    similarity_graph
}

fn extract_minimal_additions(base_functions: &[String], extended_functions: &[String]) -> Vec<String> {
    extended_functions.iter()
        .filter(|f| !base_functions.contains(f))
        .cloned()
        .collect()
}

fn build_trait_lattice(fragments: &[CodeFragment], similarity_graph: &HashMap<u64, Vec<u64>>) -> Vec<TraitLayer> {
    let mut layers = Vec::new();
    let mut processed = Vec::new();
    
    // Level 0: Base traits (fragments with smallest function count)
    let mut base_candidates: Vec<_> = fragments.iter().collect();
    base_candidates.sort_by_key(|f| f.functions.len());
    
    // Take smallest 20% as base traits
    let base_count = (fragments.len() / 5).max(1);
    for fragment in base_candidates.iter().take(base_count) {
        let layer = TraitLayer {
            name: format!("BaseTrait_{}", fragment.phi_signature % 1000),
            level: 0,
            functions: fragment.functions.clone(),
            dependencies: Vec::new(),
            phi_signature: fragment.phi_signature,
            minimal_addition: format!("base: {}", fragment.functions.join(", ")),
        };
        layers.push(layer);
        processed.push(fragment.phi_signature);
    }
    
    // Level 1+: Build layers based on extensions
    let mut level = 1;
    while processed.len() < fragments.len() && level < 5 {
        let mut added_this_level = false;
        
        for fragment in fragments {
            if processed.contains(&fragment.phi_signature) { continue; }
            
            // Find if this extends any processed fragment
            if let Some(similar) = similarity_graph.get(&fragment.phi_signature) {
                let mut best_base = None;
                let mut min_additions = usize::MAX;
                
                for &similar_phi in similar {
                    if processed.contains(&similar_phi) {
                        if let Some(base_fragment) = fragments.iter().find(|f| f.phi_signature == similar_phi) {
                            let additions = extract_minimal_additions(&base_fragment.functions, &fragment.functions);
                            if additions.len() < min_additions && !additions.is_empty() {
                                min_additions = additions.len();
                                best_base = Some((base_fragment, additions));
                            }
                        }
                    }
                }
                
                if let Some((base, additions)) = best_base {
                    let layer = TraitLayer {
                        name: format!("Layer{}Trait_{}", level, fragment.phi_signature % 1000),
                        level,
                        functions: additions.clone(),
                        dependencies: vec![format!("BaseTrait_{}", base.phi_signature % 1000)],
                        phi_signature: fragment.phi_signature,
                        minimal_addition: additions.join(", "),
                    };
                    
                    layers.push(layer);
                    processed.push(fragment.phi_signature);
                    added_this_level = true;
                }
            }
        }
        
        if !added_this_level { break; }
        level += 1;
    }
    
    layers
}

fn generate_lattice_traits(lattice: &LatticeStructure) -> String {
    let mut output = String::new();
    
    output.push_str("// Auto-generated Lattice Trait System\n");
    output.push_str("// Each layer adds minimal functionality to previous layers\n\n");
    
    // Generate traits by level
    for level in 0..=lattice.layers.iter().map(|l| l.level).max().unwrap_or(0) {
        output.push_str(&format!("// === LEVEL {} TRAITS ===\n", level));
        
        for layer in lattice.layers.iter().filter(|l| l.level == level) {
            // Generate trait
            output.push_str(&format!("/// Level {} trait: {}\n", level, layer.minimal_addition));
            output.push_str(&format!("/// Phi signature: {}\n", layer.phi_signature));
            
            if !layer.dependencies.is_empty() {
                output.push_str(&format!("trait {}: {} {{\n", layer.name, layer.dependencies.join(" + ")));
            } else {
                output.push_str(&format!("trait {} {{\n", layer.name));
            }
            
            for function in &layer.functions {
                output.push_str(&format!("    fn {}(&self);\n", function));
            }
            
            output.push_str("    fn phi_signature(&self) -> u64;\n");
            output.push_str("    fn lattice_level(&self) -> usize;\n");
            output.push_str("}\n\n");
        }
    }
    
    // Generate lattice utilities
    output.push_str("// Lattice Navigation Utilities\n");
    output.push_str("trait LatticeNavigator {\n");
    output.push_str("    fn get_dependencies(&self) -> Vec<String>;\n");
    output.push_str("    fn get_extensions(&self) -> Vec<String>;\n");
    output.push_str("    fn can_consume(&self, other_phi: u64) -> bool;\n");
    output.push_str("}\n\n");
    
    // Generate consumer trait
    output.push_str("trait CodeConsumer {\n");
    output.push_str("    fn consume_similar_code(&mut self, fragments: &[CodeFragment]);\n");
    output.push_str("    fn extract_minimal_addition(&self, base: &dyn LatticeNavigator) -> Vec<String>;\n");
    output.push_str("    fn refactor_into_layer(&self, level: usize) -> TraitLayer;\n");
    output.push_str("}\n\n");
    
    output
}

fn consume_and_refactor(fragments: &[CodeFragment]) -> LatticeStructure {
    println!("🔍 Analyzing {} code fragments for consumption", fragments.len());
    
    let similarity_graph = find_similar_fragments(fragments, 5000);
    println!("🔗 Found {} similarity clusters", similarity_graph.len());
    
    let layers = build_trait_lattice(fragments, &similarity_graph);
    println!("🏗️  Built {} trait layers", layers.len());
    
    // Show consumption statistics
    for (level, count) in (0..=layers.iter().map(|l| l.level).max().unwrap_or(0))
        .map(|l| (l, layers.iter().filter(|layer| layer.level == l).count())) {
        println!("   Level {}: {} traits", level, count);
    }
    
    LatticeStructure {
        layers,
        consumed_code: fragments.to_vec(),
        similarity_graph,
    }
}

fn main() {
    println!("=== Lattice Trait Consumer ===");
    
    // Find all code fragments
    let fragments = find_code_fragments(Path::new("./src"));
    println!("📦 Found {} code fragments", fragments.len());
    
    // Consume and refactor into lattice
    let lattice = consume_and_refactor(&fragments);
    
    // Generate lattice trait system
    let lattice_code = generate_lattice_traits(&lattice);
    
    match fs::write("generated_lattice_traits.rs", lattice_code) {
        Ok(_) => println!("✅ Generated generated_lattice_traits.rs"),
        Err(e) => println!("❌ Failed to write lattice: {}", e),
    }
    
    // Show consumption results
    println!("\n=== Consumption Results ===");
    for layer in &lattice.layers {
        println!("🔧 {} (Level {}, φ: {}, adds: {})", 
                layer.name, layer.level, layer.phi_signature, layer.minimal_addition);
        
        if !layer.dependencies.is_empty() {
            println!("   Depends on: {:?}", layer.dependencies);
        }
    }
    
    // Show similarity clusters
    println!("\n=== Similarity Clusters ===");
    for (phi, similar) in lattice.similarity_graph.iter().take(5) {
        println!("🔗 φ {} connects to {} similar fragments", phi, similar.len());
    }
    
    println!("\n✨ Lattice consumption complete!");
    println!("📊 Consumed {} fragments into {} layered traits", 
            lattice.consumed_code.len(), lattice.layers.len());
}
