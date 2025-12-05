use solana_monster_system::{Declaration, DeclarationTrait};
use std::fs;
use std::path::Path;
use syn::{File, ImplItem, Item, ItemEnum, ItemStruct, Type};
use syn_adapter_lib::{LibSynAdapter, SynAdapter}; // Added syn-adapter-lib imports

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

fn extract_types_from_file(file: &File) -> Vec<Declaration> {
    let mut types = Vec::new();

    for item in &file.items {
        match item {
            Item::Struct(item_struct) => {
                let name = item_struct.ident.to_string();
                let (fields, methods) = extract_struct_details(item_struct, file);

                types.push(Declaration::new_struct(
                    item_struct.clone(),
                    fields,
                    methods,
                    phi_hash(&name),
                ));
            }
            Item::Enum(item_enum) => {
                let name = item_enum.ident.to_string();
                let (variants, methods) = extract_enum_details(item_enum, file);

                types.push(Declaration::new_enum(
                    item_enum.clone(),
                    variants,
                    methods,
                    phi_hash(&name),
                ));
            }
            _ => {} // Ignore other items like functions, modules, etc.
        }
    }
    types
}

fn extract_struct_details(item_struct: &ItemStruct, file: &File) -> (Vec<String>, Vec<String>) {
    let mut fields = Vec::new();
    let mut methods = Vec::new();

    // Extract fields
    for field in &item_struct.fields {
        if let Some(ident) = &field.ident {
            fields.push(ident.to_string());
        }
    }

    // Look for impl blocks for this struct
    for item in &file.items {
        if let Item::Impl(item_impl) = item {
            if let Some(self_ty) = &item_impl.self_ty {
                // self_ty is &Box<Type>
                let self_ty_deref: &Type = &**self_ty; // self_ty_deref is &Type
                if let Type::Path(type_path) = self_ty_deref {
                    if type_path
                        .path
                        .segments
                        .last()
                        .map_or(false, |segment| segment.ident == item_struct.ident)
                    {
                        for impl_item in &item_impl.items {
                            if let ImplItem::Fn(impl_item_fn) = impl_item {
                                methods.push(impl_item_fn.sig.ident.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    (fields, methods)
}

fn extract_enum_details(item_enum: &ItemEnum, file: &File) -> (Vec<String>, Vec<String>) {
    let mut variants = Vec::new();
    let mut methods = Vec::new();

    // Extract variants
    for variant in &item_enum.variants {
        variants.push(variant.ident.to_string());
    }

    // Look for impl blocks for this enum
    for item in &file.items {
        if let Item::Impl(item_impl) = item {
            if let Some(self_ty) = &item_impl.self_ty {
                // self_ty is &Box<Type>
                let self_ty_deref: &Type = &**self_ty; // self_ty_deref is &Type
                if let Type::Path(type_path) = self_ty_deref {
                    if type_path
                        .path
                        .segments
                        .last()
                        .map_or(false, |segment| segment.ident == item_enum.ident)
                    {
                        for impl_item in &item_impl.items {
                            if let ImplItem::Fn(impl_item_fn) = impl_item {
                                methods.push(impl_item_fn.sig.ident.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    (variants, methods)
}

fn generate_trait_pairing(declaration: &Declaration) -> TraitPairing {
    let trait_name = format!("{}Trait", declaration.name());

    // Generate trait methods
    let mut trait_methods = Vec::new();

    // Add getter methods for fields
    for field in declaration.fields() {
        trait_methods.push(format!("    fn get_{}(&self) -> &str;", field));
        trait_methods.push(format!("    fn set_{}(&mut self, value: String);", field));
    }

    // Add existing methods
    for method in declaration.methods() {
        trait_methods.push(format!("    fn {}(&self);", method));
    }

    // Add Monster Group methods
    trait_methods.push("    fn phi_signature(&self) -> u64;".to_string());
    trait_methods.push("    fn monster_element(&self) -> u64;".to_string());

    let trait_code = format!(
        "/// Auto-generated trait for {}\n/// Phi signature: {}\ntrait {} {{\n{}\n}}",
        declaration.name(),
        declaration.phi_signature(),
        trait_name,
        trait_methods.join("\n")
    );

    // Generate implementation
    let mut impl_methods = Vec::new();

    for field in declaration.fields() {
        impl_methods.push(format!(
            "    fn get_{}(&self) -> &str {{ &self.{} }}",
            field, field
        ));
        impl_methods.push(format!(
            "    fn set_{}(&mut self, value: String) {{ self.{} = value; }}",
            field, field
        ));
    }

    for method in declaration.methods() {
        impl_methods.push(format!(
            "    fn {}(&self) {{ /* original implementation */ }}",
            method
        ));
    }

    impl_methods.push(format!(
        "    fn phi_signature(&self) -> u64 {{ {} }}",
        declaration.phi_signature()
    ));

    impl_methods.push(format!(
        "    fn monster_element(&self) -> u64 {{ {} % 196883 }}",
        declaration.phi_signature()
    ));

    let impl_code = format!(
        "impl {} for {} {{\n{}\n}}",
        trait_name,
        declaration.name(),
        impl_methods.join("\n")
    );

    TraitPairing {
        original_type: declaration.name().to_string(),
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
    output.push_str(
        "fn find_similar_types<T: DeclarationTrait>(types: &[T]) -> Vec<(usize, usize)> {\n",
    );
    output.push_str("    let mut similar = Vec::new();\n");
    output.push_str("    for i in 0..types.len() {\n");
    output.push_str("        for j in (i+1)..types.len() {\n");
    output.push_str(
        "            let diff = if types[i].phi_signature() > types[j].phi_signature() {\n",
    );
    output.push_str("                types[i].phi_signature() - types[j].phi_signature()\n");
    output.push_str("            } else {\n");
    output.push_str("                types[j].phi_signature() - types[i].phi_signature()\n");
    output.push_str("            };\n");
    output.push_str("            if diff < 1000 { similar.push((i, j)); }\n");
    output.push_str("        }\n");
    output.push_str("    }\n");
    output.push_str("    similar\n");
    output.push_str("}\n\n");

    output
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_rust_file>", args[0]);
        return;
    }
    let input_file_path_str = &args[1];
    let target_file_path = Path::new(input_file_path_str);

    let mut all_types: Vec<Declaration> = Vec::new();

    let adapter = LibSynAdapter::new(); // Initialize LibSynAdapter

    let syn_file = match adapter.parse_file(target_file_path) {
        // Parse file
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error parsing file {:?}: {}", target_file_path, e);
            return;
        }
    };

    let types = extract_types_from_file(&syn_file); // Pass syn_file to extraction
    all_types.extend(types);

    // Generate trait pairings
    let mut pairings = Vec::new();
    for declaration in &all_types {
        let pairing = generate_trait_pairing(declaration);
        pairings.push(pairing);
    }

    // Show some examples

    // Generate pure traits file
    let traits_code = generate_pure_traits_file(&pairings);

    let _ = fs::write("generated_pure_traits.rs", traits_code);

    // Find similar types
    let mut similar_pairs = Vec::new();
    for i in 0..all_types.len() {
        for j in (i + 1)..all_types.len() {
            let diff = if all_types[i].phi_signature() > all_types[j].phi_signature() {
                all_types[i].phi_signature() - all_types[j].phi_signature()
            } else {
                all_types[j].phi_signature() - all_types[i].phi_signature()
            };

            if diff < 1000 {
                similar_pairs.push((
                    all_types[i].name().to_string(),
                    all_types[j].name().to_string(),
                ));
            }
        }
    }
}
