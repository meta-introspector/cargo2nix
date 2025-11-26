use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
struct TypeInfo {
    name: String,
    fields: Vec<String>,
    methods: Vec<String>,
    phi_signature: u64,
}

#[derive(Debug)]
struct TraitPairing {
    original_type: String,
    trait_name: String,
    trait_code: String,
    impl_code: String,
}

fn phi_hash(s: &str) -> u64 {
    let mut hash = 5381u64;
    for byte in s.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(byte as u64);
    }
    hash % 196883
}

fn extract_types_from_file(path: &Path) -> Vec<TypeInfo> {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };
    
    let mut types = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        
        // Extract structs
        if trimmed.starts_with("struct ") {
            let name = extract_type_name(trimmed, "struct");
            let (fields, methods) = extract_struct_details(&lines, i);
            
            types.push(TypeInfo {
                phi_signature: phi_hash(&name),
                name,
                fields,
                methods,
            });
        }
        
        // Extract enums
        if trimmed.starts_with("enum ") {
            let name = extract_type_name(trimmed, "enum");
            let (variants, methods) = extract_enum_details(&lines, i);
            
            types.push(TypeInfo {
                phi_signature: phi_hash(&name),
                name,
                fields: variants,
                methods,
            });
        }
    }
    
    types
}

fn extract_type_name(line: &str, keyword: &str) -> String {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 2 {
        parts[1].split('<').next().unwrap_or(parts[1])
               .split('{').next().unwrap_or(parts[1])
               .to_string()
    } else {
        "Unknown".to_string()
    }
}

fn extract_struct_details(lines: &[&str], start: usize) -> (Vec<String>, Vec<String>) {
    let mut fields = Vec::new();
    let mut methods = Vec::new();
    let mut in_struct = false;
    let mut brace_count = 0;
    
    for i in start..lines.len() {
        let line = lines[i].trim();
        
        if line.contains('{') {
            in_struct = true;
            brace_count += line.matches('{').count();
        }
        
        if in_struct {
            brace_count -= line.matches('}').count();
            
            // Extract fields
            if line.contains(':') && !line.starts_with("//") && !line.contains("fn") {
                let field = line.split(':').next().unwrap_or("").trim().to_string();
                if !field.is_empty() {
                    fields.push(field);
                }
            }
            
            if brace_count == 0 {
                break;
            }
        }
    }
    
    // Look for impl blocks
    for i in start..lines.len().min(start + 50) {
        let line = lines[i].trim();
        if line.starts_with("fn ") {
            let func_name = extract_function_name(line);
            methods.push(func_name);
        }
    }
    
    (fields, methods)
}

fn extract_enum_details(lines: &[&str], start: usize) -> (Vec<String>, Vec<String>) {
    let mut variants = Vec::new();
    let mut methods = Vec::new();
    let mut in_enum = false;
    let mut brace_count = 0;
    
    for i in start..lines.len() {
        let line = lines[i].trim();
        
        if line.contains('{') {
            in_enum = true;
            brace_count += line.matches('{').count();
        }
        
        if in_enum {
            brace_count -= line.matches('}').count();
            
            // Extract variants
            if !line.is_empty() && !line.starts_with("//") && !line.contains('{') && !line.contains('}') {
                let variant = line.split(',').next().unwrap_or("").trim().to_string();
                if !variant.is_empty() {
                    variants.push(variant);
                }
            }
            
            if brace_count == 0 {
                break;
            }
        }
    }
    
    (variants, methods)
}

fn extract_function_name(line: &str) -> String {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 2 {
        parts[1].split('(').next().unwrap_or(parts[1]).to_string()
    } else {
        "unknown".to_string()
    }
}

fn generate_trait_pairing(type_info: &TypeInfo) -> TraitPairing {
    let trait_name = format!("{}Trait", type_info.name);
    
    // Generate trait methods
    let mut trait_methods = Vec::new();
    
    // Add getter methods for fields
    for field in &type_info.fields {
        trait_methods.push(format!("    fn get_{}(&self) -> &str;", field));
        trait_methods.push(format!("    fn set_{}(&mut self, value: String);", field));
    }
    
    // Add existing methods
    for method in &type_info.methods {
        trait_methods.push(format!("    fn {}(&self);", method));
    }
    
    // Add Monster Group methods
    trait_methods.push("    fn phi_signature(&self) -> u64;".to_string());
    trait_methods.push("    fn monster_element(&self) -> u64;".to_string());
    
    let trait_code = format!(
        "/// Auto-generated trait for {}\n/// Phi signature: {}\ntrait {} {{\n{}\n}}",
        type_info.name,
        type_info.phi_signature,
        trait_name,
        trait_methods.join("\n")
    );
    
    // Generate implementation
    let mut impl_methods = Vec::new();
    
    for field in &type_info.fields {
        impl_methods.push(format!(
            "    fn get_{}(&self) -> &str {{ &self.{} }}",
            field, field
        ));
        impl_methods.push(format!(
            "    fn set_{}(&mut self, value: String) {{ self.{} = value; }}",
            field, field
        ));
    }
    
    for method in &type_info.methods {
        impl_methods.push(format!(
            "    fn {}(&self) {{ /* original implementation */ }}",
            method
        ));
    }
    
    impl_methods.push(format!(
        "    fn phi_signature(&self) -> u64 {{ {} }}",
        type_info.phi_signature
    ));
    
    impl_methods.push(format!(
        "    fn monster_element(&self) -> u64 {{ {} % 196883 }}",
        type_info.phi_signature
    ));
    
    let impl_code = format!(
        "impl {} for {} {{\n{}\n}}",
        trait_name,
        type_info.name,
        impl_methods.join("\n")
    );
    
    TraitPairing {
        original_type: type_info.name.clone(),
        trait_name,
        trait_code,
        impl_code,
    }
}

fn generate_pure_traits_file(pairings: &[TraitPairing]) -> String {
    let mut output = String::new();
    
    output.push_str("// Auto-generated pure traits from Monster Group analysis\n");
    output.push_str("// Each type is paired with a trait for maximum flexibility\n\n");
    
    // Add traits
    for pairing in pairings {
        output.push_str(&pairing.trait_code);
        output.push_str("\n\n");
    }
    
    // Add implementations
    output.push_str("// Implementations\n\n");
    for pairing in pairings {
        output.push_str(&pairing.impl_code);
        output.push_str("\n\n");
    }
    
    // Add Monster Group utilities
    output.push_str("// Monster Group utilities\n");
    output.push_str("fn find_similar_types<T: PhiSignatureTrait>(types: &[T]) -> Vec<(usize, usize)> {\n");
    output.push_str("    let mut similar = Vec::new();\n");
    output.push_str("    for i in 0..types.len() {\n");
    output.push_str("        for j in (i+1)..types.len() {\n");
    output.push_str("            let diff = if types[i].phi_signature() > types[j].phi_signature() {\n");
    output.push_str("                types[i].phi_signature() - types[j].phi_signature()\n");
    output.push_str("            } else {\n");
    output.push_str("                types[j].phi_signature() - types[i].phi_signature()\n");
    output.push_str("            };\n");
    output.push_str("            if diff < 1000 { similar.push((i, j)); }\n");
    output.push_str("        }\n");
    output.push_str("    }\n");
    output.push_str("    similar\n");
    output.push_str("}\n\n");
    
    // Add universal trait
    output.push_str("trait PhiSignatureTrait {\n");
    output.push_str("    fn phi_signature(&self) -> u64;\n");
    output.push_str("    fn monster_element(&self) -> u64;\n");
    output.push_str("}\n");
    
    output
}

fn main() {
    println!("=== Trait Generator with Quote ===");
    
    let mut all_types = Vec::new();
    
    // Scan source files
    if let Ok(entries) = fs::read_dir("./src") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "rs") {
                let types = extract_types_from_file(&path);
                all_types.extend(types);
            }
        }
    }
    
    println!("📦 Found {} types", all_types.len());
    
    // Generate trait pairings
    let mut pairings = Vec::new();
    for type_info in &all_types {
        let pairing = generate_trait_pairing(type_info);
        pairings.push(pairing);
    }
    
    println!("🔧 Generated {} trait pairings", pairings.len());
    
    // Show some examples
    println!("\n=== Sample Trait Pairings ===");
    for (i, pairing) in pairings.iter().take(3).enumerate() {
        println!("{}. {} -> {}", i + 1, pairing.original_type, pairing.trait_name);
        println!("   Phi: {}", phi_hash(&pairing.original_type));
    }
    
    // Generate pure traits file
    let traits_code = generate_pure_traits_file(&pairings);
    
    match fs::write("generated_pure_traits.rs", traits_code) {
        Ok(_) => println!("✅ Generated generated_pure_traits.rs"),
        Err(e) => println!("❌ Failed to write file: {}", e),
    }
    
    // Find similar types
    let mut similar_pairs = Vec::new();
    for i in 0..all_types.len() {
        for j in (i + 1)..all_types.len() {
            let diff = if all_types[i].phi_signature > all_types[j].phi_signature {
                all_types[i].phi_signature - all_types[j].phi_signature
            } else {
                all_types[j].phi_signature - all_types[i].phi_signature
            };
            
            if diff < 1000 {
                similar_pairs.push((all_types[i].name.clone(), all_types[j].name.clone()));
            }
        }
    }
    
    println!("\n=== Similar Types (by phi signature) ===");
    for (type1, type2) in similar_pairs.iter().take(5) {
        println!("🔗 {} ↔ {} (similar phi signatures)", type1, type2);
    }
    
    println!("\n✨ Trait generation complete!");
    println!("📊 {} types converted to trait pairings", pairings.len());
}
