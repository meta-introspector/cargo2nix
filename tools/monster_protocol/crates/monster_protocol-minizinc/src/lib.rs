use monster_protocol_core::{BlockAnalyzer, ExtractedTrait};
use std::path::PathBuf;
use std::error::Error;
use std::fs;

// --- Trait Definition ---
pub trait MiniZincModelGenerator {
    /// Generates the MiniZinc model (.mzn) content based on an analyzer's state.
    fn generate_model(&self, analyzer: &BlockAnalyzer, complexity_index: u8) -> Result<String, String>;

    /// Generates the MiniZinc data (.dzn) content from a BlockAnalyzer.
    fn generate_data_file_from_analyzer(&self, analyzer: &BlockAnalyzer) -> String;

    /// Generates the MiniZinc data (.dzn) content from ExtractedTrait and writes to a file.
    fn generate_data_file_for_extracted_traits(
        &self,
        extracted_traits: Vec<ExtractedTrait>,
        output_path: &PathBuf,
    ) -> Result<(), Box<dyn Error>>;

    /// Generates the MiniZinc selection model (.mzn) and writes to a file.
    fn generate_selection_model(&self, output_path: &PathBuf) -> Result<(), Box<dyn Error>>;

    /// Extracts traits from BlockAnalyzer's state.
    fn extract_traits(&self, analyzer: &BlockAnalyzer) -> Vec<ExtractedTrait>;
}

// --- Default Implementation ---
pub struct DefaultMiniZincModelGenerator;

impl DefaultMiniZincModelGenerator {
    pub fn new() -> Self {
        DefaultMiniZincModelGenerator
    }
}

impl MiniZincModelGenerator for DefaultMiniZincModelGenerator {
    fn generate_model(&self, analyzer: &BlockAnalyzer, complexity_index: u8) -> Result<String, String> {
        let mut model_content = String::new();
        let traits = analyzer.extract_all_traits(); // Assuming BlockAnalyzer still has this method
        let num_traits = traits.len();

        // Monster Group constants
        model_content.push_str("int: monster_order = 196883;\n");
        model_content.push_str("int: hecke_pos = 196883;\n");
        model_content.push_str("int: hecke_neg = -5472;\n");
        model_content.push_str("int: modularity_prime = 24;\n\n"); // Ramanujan τ modular constraint

        // Arrays for trait data
        model_content.push_str(&format!("int: num_traits = {};\n", num_traits));
        model_content.push_str("array[1..num_traits] of int: trait_hashes;\n");
        
        // Decision variables
        let max_val = 2u32.pow(complexity_index as u32) - 1;
        model_content.push_str(&format!("array[1..num_traits] of var 0..{}: monster_elements;\n", 
            std::cmp::min(max_val, 196882)));
        model_content.push_str("array[1..num_traits] of var {hecke_neg, hecke_pos}: hecke_values;\n\n");

        // Monster Group constraints
        model_content.push_str("% Monster Group structure preservation\n");
        model_content.push_str("constraint all_different(monster_elements);\n");
        
        // Modular form constraint (Ramanujan τ)
        model_content.push_str("constraint sum(i in 1..num_traits) (monster_elements[i]) mod modularity_prime = 0;\n");
        
        // Trait consistency constraints
        for (i, _) in traits.iter().enumerate() {
            model_content.push_str(&format!(
                "constraint abs(trait_hashes[{}] - monster_elements[{}]) <= {};\n", 
                i + 1, i + 1, max_val
            ));
        }

        // Objective: minimize deviation while preserving Monster Group structure
        model_content.push_str("\nsolve minimize sum(i in 1..num_traits) (\n");
        model_content.push_str("    abs(trait_hashes[i] - monster_elements[i])\n");
        model_content.push_str(");\n\n");

        // Output
        model_content.push_str("output [\n");
        model_content.push_str("    \"monster_elements = \", show(monster_elements), \"\n\",\n");
        model_content.push_str("    \"hecke_values = \", show(hecke_values), \"\n\"
");
        model_content.push_str("];\n");

        Ok(model_content)
    }

    fn generate_data_file_from_analyzer(&self, analyzer: &BlockAnalyzer) -> String {
        let traits = analyzer.extract_all_traits();
        let mut data = String::new();
        
        data.push_str(&format!("num_traits = {};\n", traits.len()));
        data.push_str("trait_hashes = [");
        
        for (i, trait_name) in traits.iter().enumerate() {
            let hash = trait_name.bytes().fold(0u32, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u32));
            data.push_str(&hash.to_string());
            if i < traits.len() - 1 { data.push_str(", "); }
        }
        
        data.push_str("];\n");
        data
    }

    fn generate_data_file_for_extracted_traits(
        &self,
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
                .replace("\\", "\\\\") // Correctly escape backslashes
                .replace("\"", "\\\"") // Correctly escape double quotes
                .replace("\n", "\\n"); // Correctly escape newlines
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

    fn generate_selection_model(&self, output_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
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

    fn extract_traits(&self, analyzer: &BlockAnalyzer) -> Vec<ExtractedTrait> {
        let mut extracted = Vec::new();
        
        for (block_id, block) in analyzer.compiler_blocks() {
            for trait_name in &block.traits_consumed {
                let hash = trait_name.bytes().fold(0u32, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u32));
                extracted.push(ExtractedTrait {
                    trait_name: trait_name.clone(),
                    block_id: block_id.clone(),
                    hash_value: hash,
                });
            }
        }
        // Assuming BlockAnalyzer still holds tool_blocks and we want to extract traits from them too.
        for (block_id, block) in analyzer.tool_blocks() {
            for trait_name in &block.traits_produced { // Only produced traits from tool blocks for now
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
}

