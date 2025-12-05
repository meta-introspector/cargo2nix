//! Declaration Splitter integrated with Monster Group AST Transport
//! Based on solfunmeme-dioxus declaration_splitter.rs

use crate::ast_transport::{AstFragment, TransportColony};
use crate::token_constants::CompressedToken;
use serde::{Deserialize, Serialize};
use syn::{spanned::Spanned, File, Item, ItemEnum, ItemFn, ItemImpl, ItemStruct, ItemTrait};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Declaration {
    pub name: String,
    pub declaration_type: DeclarationType,
    pub content: String,
    pub line_start: usize,
    pub line_end: usize,
    pub file_path: Option<String>,
    pub monster_factor: u64,
    pub transport_layer: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DeclarationType {
    Function,
    Struct,
    Enum,
    Trait,
    Impl,
    Module,
    Use,
    Const,
    Static,
    Type,
    Macro,
}

pub struct MonsterDeclarationSplitter {
    pub declarations: Vec<Declaration>,
    pub transport_colony: TransportColony,
}

impl MonsterDeclarationSplitter {
    pub fn new() -> Self {
        Self {
            declarations: Vec::new(),
            transport_colony: TransportColony::new(),
        }
    }

    pub fn split_file(&mut self, content: &str, file_path: Option<String>) -> Result<(), syn::Error> {
        let syntax_tree = syn::parse_file(content)?;
        self.extract_declarations(&syntax_tree, content, file_path);
        self.transport_declarations();
        Ok(())
    }

    fn extract_declarations(&mut self, syntax_tree: &File, content: &str, file_path: Option<String>) {
        let lines: Vec<&str> = content.lines().collect();
        
        for item in &syntax_tree.items {
            let declaration = match item {
                Item::Fn(item_fn) => self.function_to_declaration(item_fn, &lines, file_path.clone()),
                Item::Struct(item_struct) => self.struct_to_declaration(item_struct, &lines, file_path.clone()),
                Item::Enum(item_enum) => self.enum_to_declaration(item_enum, &lines, file_path.clone()),
                Item::Trait(item_trait) => self.trait_to_declaration(item_trait, &lines, file_path.clone()),
                Item::Impl(item_impl) => self.impl_to_declaration(item_impl, &lines, file_path.clone()),
                _ => continue,
            };
            
            self.declarations.push(declaration);
        }
    }

    fn function_to_declaration(&self, item_fn: &ItemFn, lines: &[&str], file_path: Option<String>) -> Declaration {
        let name = item_fn.sig.ident.to_string();
        let (start, end) = self.get_span_lines(&item_fn.span());
        let content = self.extract_content(lines, start, end);
        let monster_factor = self.calculate_monster_factor(&name, DeclarationType::Function);
        let transport_layer = self.calculate_transport_layer(start);

        Declaration {
            name,
            declaration_type: DeclarationType::Function,
            content,
            line_start: start,
            line_end: end,
            file_path,
            monster_factor,
            transport_layer,
        }
    }

    fn struct_to_declaration(&self, item_struct: &ItemStruct, lines: &[&str], file_path: Option<String>) -> Declaration {
        let name = item_struct.ident.to_string();
        let (start, end) = self.get_span_lines(&item_struct.span());
        let content = self.extract_content(lines, start, end);
        let monster_factor = self.calculate_monster_factor(&name, DeclarationType::Struct);
        let transport_layer = self.calculate_transport_layer(start);

        Declaration {
            name,
            declaration_type: DeclarationType::Struct,
            content,
            line_start: start,
            line_end: end,
            file_path,
            monster_factor,
            transport_layer,
        }
    }

    fn enum_to_declaration(&self, item_enum: &ItemEnum, lines: &[&str], file_path: Option<String>) -> Declaration {
        let name = item_enum.ident.to_string();
        let (start, end) = self.get_span_lines(&item_enum.span());
        let content = self.extract_content(lines, start, end);
        let monster_factor = self.calculate_monster_factor(&name, DeclarationType::Enum);
        let transport_layer = self.calculate_transport_layer(start);

        Declaration {
            name,
            declaration_type: DeclarationType::Enum,
            content,
            line_start: start,
            line_end: end,
            file_path,
            monster_factor,
            transport_layer,
        }
    }

    fn trait_to_declaration(&self, item_trait: &ItemTrait, lines: &[&str], file_path: Option<String>) -> Declaration {
        let name = item_trait.ident.to_string();
        let (start, end) = self.get_span_lines(&item_trait.span());
        let content = self.extract_content(lines, start, end);
        let monster_factor = self.calculate_monster_factor(&name, DeclarationType::Trait);
        let transport_layer = self.calculate_transport_layer(start);

        Declaration {
            name,
            declaration_type: DeclarationType::Trait,
            content,
            line_start: start,
            line_end: end,
            file_path,
            monster_factor,
            transport_layer,
        }
    }

    fn impl_to_declaration(&self, item_impl: &ItemImpl, lines: &[&str], file_path: Option<String>) -> Declaration {
        let name = if let Some((_, path, _)) = &item_impl.trait_ {
            format!("impl_{}", quote::quote!(#path).to_string())
        } else {
            format!("impl_{}", quote::quote!(#item_impl.self_ty).to_string())
        };
        let (start, end) = self.get_span_lines(&item_impl.span());
        let content = self.extract_content(lines, start, end);
        let monster_factor = self.calculate_monster_factor(&name, DeclarationType::Impl);
        let transport_layer = self.calculate_transport_layer(start);

        Declaration {
            name,
            declaration_type: DeclarationType::Impl,
            content,
            line_start: start,
            line_end: end,
            file_path,
            monster_factor,
            transport_layer,
        }
    }

    fn get_span_lines(&self, _span: &proc_macro2::Span) -> (usize, usize) {
        // proc_macro2::Span doesn't have start/end methods in this context
        // Use a simple fallback for now
        (1, 1)
    }

    fn extract_content(&self, lines: &[&str], start: usize, end: usize) -> String {
        if start == 0 || start > lines.len() || end > lines.len() {
            return String::new();
        }
        
        lines[(start - 1)..end].join("\n")
    }

    fn calculate_monster_factor(&self, name: &str, decl_type: DeclarationType) -> u64 {
        let monster_primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71];
        let type_offset = match decl_type {
            DeclarationType::Function => 0,
            DeclarationType::Struct => 3,
            DeclarationType::Enum => 6,
            DeclarationType::Trait => 9,
            DeclarationType::Impl => 12,
            _ => 0,
        };
        let name_hash = name.len() + name.chars().map(|c| c as usize).sum::<usize>();
        let index = (type_offset + name_hash) % monster_primes.len();
        monster_primes[index]
    }

    fn calculate_transport_layer(&self, line_start: usize) -> u8 {
        (line_start % 108) as u8
    }

    fn transport_declarations(&mut self) {
        for declaration in &self.declarations {
            let compressed_token = CompressedToken::new(
                declaration.transport_layer,
                (declaration.monster_factor % 256) as u8
            );
            let fragment = AstFragment::new(compressed_token, declaration.transport_layer);
            self.transport_colony.add_source(fragment);
        }
        
        self.transport_colony.build_all_layers();
    }

    pub fn get_declarations_by_type(&self, decl_type: DeclarationType) -> Vec<&Declaration> {
        self.declarations.iter()
            .filter(|d| d.declaration_type == decl_type)
            .collect()
    }

    pub fn get_monster_factor_distribution(&self) -> std::collections::HashMap<u64, usize> {
        let mut distribution = std::collections::HashMap::new();
        for declaration in &self.declarations {
            *distribution.entry(declaration.monster_factor).or_insert(0) += 1;
        }
        distribution
    }

    pub fn generate_trait_registry(&self) -> String {
        let traits = self.get_declarations_by_type(DeclarationType::Trait);
        let mut registry = String::new();
        
        registry.push_str("// Monster Group Trait Registry from Declaration Splitter\n");
        registry.push_str("use std::collections::HashMap;\n\n");
        registry.push_str("pub struct MonsterTraitRegistry {\n");
        registry.push_str("    traits_by_factor: HashMap<u64, Vec<&'static str>>,\n");
        registry.push_str("}\n\n");
        registry.push_str("impl MonsterTraitRegistry {\n");
        registry.push_str("    pub fn new() -> Self {\n");
        registry.push_str("        let mut registry = Self {\n");
        registry.push_str("            traits_by_factor: HashMap::new(),\n");
        registry.push_str("        };\n");

        let mut factor_groups: std::collections::HashMap<u64, Vec<&str>> = std::collections::HashMap::new();
        for trait_decl in traits {
            factor_groups.entry(trait_decl.monster_factor)
                .or_default()
                .push(&trait_decl.name);
        }

        for (factor, trait_names) in factor_groups {
            registry.push_str(&format!("        registry.traits_by_factor.insert({}, vec![", factor));
            for (i, name) in trait_names.iter().enumerate() {
                if i > 0 { registry.push_str(", "); }
                registry.push_str(&format!("\"{}\"", name));
            }
            registry.push_str("]);\n");
        }

        registry.push_str("        registry\n");
        registry.push_str("    }\n");
        registry.push_str("}\n");
        
        registry
    }
}
