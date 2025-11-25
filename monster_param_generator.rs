use crate::rust_block_analyzer::{BlockAnalyzer, RustBlock};
use std::path::PathBuf;
use std::fs;

#[derive(Debug, Clone)]
pub struct ExtractedTrait {
    pub trait_name: String,
    pub block_id: String,
    pub hash_value: u32,
}

pub fn generate_monster_data_file(
    extracted_traits: Vec<ExtractedTrait>, 
    output_path: &PathBuf
) -> Result<(), Box<dyn std::error::Error>> {
    let mut content = String::new();
    
    // Monster Group constants
    content.push_str("monster_order = 196883;\n");
    content.push_str("hecke_eigenvalues = [196883, -5472];\n");
    content.push_str("ramanujan_tau = [1, -24, 252, -1472, 4830, -6048, -16744, 84480];\n\n");
    
    // Trait data arrays
    content.push_str(&format!("num_traits = {};\n", extracted_traits.len()));
    content.push_str("trait_names = [\n");
    
    for (i, trait_info) in extracted_traits.iter().enumerate() {
        let escaped_name = trait_info.trait_name
            .replace("\\", "\\\\")
            .replace("\"", "\\\"")
            .replace("\n", "\\n");
        content.push_str(&format!("    \"{}\"", escaped_name));
        if i < extracted_traits.len() - 1 {
            content.push_str(",\n");
        } else {
            content.push_str("\n");
        }
    }
    content.push_str("];\n\n");
    
    // Hash values for Monster Group mapping
    content.push_str("trait_hashes = [");
    for (i, trait_info) in extracted_traits.iter().enumerate() {
        content.push_str(&trait_info.hash_value.to_string());
        if i < extracted_traits.len() - 1 { content.push_str(", "); }
    }
    content.push_str("];\n");
    
    fs::write(output_path, content)?;
    Ok(())
}

pub fn generate_monster_selection_model(output_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let model_content = r###"% Monster Group Trait Selection Model
array[int] of string: trait_names;
array[int] of int: trait_hashes;
int: num_traits = length(trait_names);
int: monster_order;
array[int] of int: hecke_eigenvalues;

% Decision variables
var 1..num_traits: selected_trait_index;
array[1..num_traits] of var 0..monster_order-1: monster_elements;
array[1..num_traits] of var 1..2: hecke_indices;

% Monster Group constraints
constraint all_different(monster_elements);
constraint sum(i in 1..num_traits) (monster_elements[i]) mod 24 = 0;

% Selected trait must satisfy Monster Group properties
constraint monster_elements[selected_trait_index] = trait_hashes[selected_trait_index] mod monster_order;

% The selected trait
string: selected_trait = trait_names[selected_trait_index];
int: selected_element = monster_elements[selected_trait_index];

solve minimize abs(trait_hashes[selected_trait_index] - monster_elements[selected_trait_index]);

output [
    "selected_trait = ", selected_trait, "\n",
    "monster_element = ", show(selected_element), "\n",
    "hecke_value = ", show(hecke_eigenvalues[hecke_indices[selected_trait_index]]), "\n"
];
"###;

    fs::write(output_path, model_content)?;
    Ok(())
}

pub fn extract_traits_from_blocks(analyzer: &BlockAnalyzer) -> Vec<ExtractedTrait> {
    let mut extracted = Vec::new();
    
    for (block_id, block) in &analyzer.compiler_blocks {
        for trait_name in &block.traits_consumed {
            let hash = trait_name.bytes().fold(0u32, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u32));
            extracted.push(ExtractedTrait {
                trait_name: trait_name.clone(),
                block_id: block_id.clone(),
                hash_value: hash,
            });
        }
        for trait_name in &block.traits_produced {
            let hash = trait_name.bytes().fold(0u32, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u32));
            extracted.push(ExtractedTrait {
                trait_name: trait_name.clone(),
                block_id: block_id.clone(),
                hash_value: hash,
            });
        }
    }
    
    extracted
}
