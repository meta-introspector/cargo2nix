use crate::rust_block_analyzer::{BlockAnalyzer, TraitMapping};

pub fn generate_monster_trait_model(
    analyzer: &BlockAnalyzer,
    complexity_index: u8,
) -> Result<String, String> {
    let mut model_content = String::new();
    let traits = analyzer.extract_all_traits();
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
    model_content.push_str("    \"monster_elements = \", show(monster_elements), \"\\n\",\n");
    model_content.push_str("    \"hecke_values = \", show(hecke_values), \"\\n\"\n");
    model_content.push_str("];\n");

    Ok(model_content)
}

pub fn generate_trait_data_file(analyzer: &BlockAnalyzer) -> String {
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
