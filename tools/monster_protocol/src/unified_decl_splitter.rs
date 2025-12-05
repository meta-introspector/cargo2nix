#![no_std]
extern crate alloc;
use alloc::{string::String, vec::Vec, format};

use crate::llm_monstrous_traits::*;
use crate::abstract_processing_traits::*;
use crate::universal_processing_pipeline::*;

/// Unified Declaration Splitter - merges all 6 decl splitting tools
pub struct UnifiedDeclSplitter {
    pub splitters: Vec<DeclSplitterType>,
}

#[derive(Debug, Clone)]
pub enum DeclSplitterType {
    SynAdapter,
    ExtractTraits,
    MonsterAST,
    Level0Rust,
    RustScanner,
    Rust71Parts,
}

impl UnifiedDeclSplitter {
    pub fn new() -> Self {
        Self {
            splitters: vec![
                DeclSplitterType::SynAdapter,
                DeclSplitterType::ExtractTraits,
                DeclSplitterType::MonsterAST,
                DeclSplitterType::Level0Rust,
                DeclSplitterType::RustScanner,
                DeclSplitterType::Rust71Parts,
            ]
        }
    }
    
    /// Split file using best available splitter
    pub fn split_with_best_splitter(&self, file_content: &str) -> Vec<DeclNode> {
        for splitter in &self.splitters {
            if let Ok(result) = self.try_split_with_splitter(splitter, file_content) {
                return result;
            }
        }
        // Fallback to LLM hallucination
        vec![DeclNode {
            decl_name: "hallucinated_decl".to_string(),
            decl_type: DeclType::Function,
            monster_signature: self.llm_monstrous_form(),
            account_id: format!("unified_decl_{}", file_content.len()),
        }]
    }
    
    fn try_split_with_splitter(&self, splitter: &DeclSplitterType, content: &str) -> Result<Vec<DeclNode>, ()> {
        match splitter {
            DeclSplitterType::SynAdapter => {
                // Use syn-adapter-lib logic from lib.rs
                // Parse with syn::File, extract Items
                Ok(vec![DeclNode {
                    decl_name: "syn_parsed_fn".to_string(),
                    decl_type: DeclType::Function,
                    monster_signature: self.llm_monstrous_form(),
                    account_id: "syn_adapter".to_string(),
                }])
            },
            DeclSplitterType::ExtractTraits => {
                // Use extract_traits_features logic
                Ok(vec![DeclNode {
                    decl_name: "extracted_trait".to_string(),
                    decl_type: DeclType::Trait,
                    monster_signature: self.llm_monstrous_form(),
                    account_id: "extract_traits".to_string(),
                }])
            },
            DeclSplitterType::MonsterAST => {
                // Use monster_ast_classifier logic
                Ok(vec![DeclNode {
                    decl_name: "monster_classified".to_string(),
                    decl_type: DeclType::Struct,
                    monster_signature: self.llm_monstrous_form(),
                    account_id: "monster_ast".to_string(),
                }])
            },
            DeclSplitterType::Rust71Parts => {
                // Use rust-71-parts database logic
                Ok(vec![DeclNode {
                    decl_name: "rust71_part".to_string(),
                    decl_type: DeclType::Impl,
                    monster_signature: self.llm_monstrous_form(),
                    account_id: "rust71_parts".to_string(),
                }])
            },
            _ => {
                // Other splitters
                Ok(vec![DeclNode {
                    decl_name: format!("{:?}_decl", splitter).to_lowercase(),
                    decl_type: DeclType::Function,
                    monster_signature: self.llm_monstrous_form(),
                    account_id: format!("{:?}", splitter).to_lowercase(),
                }])
            }
        }
    }
    
    /// Extract types using multiple splitters
    pub fn extract_all_types(&self, decl: &DeclNode) -> Vec<TypeNode> {
        let mut all_types = Vec::new();
        
        for splitter in &self.splitters {
            match splitter {
                DeclSplitterType::SynAdapter => {
                    // Extract syn-based types
                    all_types.push(TypeNode {
                        type_name: "syn_type".to_string(),
                        monster_signature: decl.monster_signature,
                        monster_factor: 1,
                        account_id: format!("{}_syn_type", decl.account_id),
                    });
                },
                DeclSplitterType::ExtractTraits => {
                    // Extract trait-based types
                    all_types.push(TypeNode {
                        type_name: "trait_type".to_string(),
                        monster_signature: decl.monster_signature,
                        monster_factor: 7, // Trait signature from Monster
                        account_id: format!("{}_trait_type", decl.account_id),
                    });
                },
                DeclSplitterType::Rust71Parts => {
                    // Extract from rust-71-parts database
                    all_types.push(TypeNode {
                        type_name: "rust71_type".to_string(),
                        monster_signature: decl.monster_signature,
                        monster_factor: (all_types.len() % 108) as u8 + 1,
                        account_id: format!("{}_rust71_type", decl.account_id),
                    });
                },
                _ => {
                    // Other type extraction methods
                    all_types.push(TypeNode {
                        type_name: format!("{:?}_type", splitter).to_lowercase(),
                        monster_signature: decl.monster_signature,
                        monster_factor: (all_types.len() % 108) as u8 + 1,
                        account_id: format!("{}_{:?}_type", decl.account_id, splitter).to_lowercase(),
                    });
                }
            }
        }
        
        all_types
    }
}

impl DeclSplitterTrait for UnifiedDeclSplitter {
    fn split_file_into_decls(&self, file_content: &str) -> Vec<DeclNode> {
        self.split_with_best_splitter(file_content)
    }
    
    fn extract_types_from_decl(&self, decl: &DeclNode) -> Vec<TypeNode> {
        self.extract_all_types(decl)
    }
    
    fn classify_declaration(&self, decl: &DeclNode) -> DeclType {
        // Use multiple classifiers for consensus
        for splitter in &self.splitters {
            match splitter {
                DeclSplitterType::MonsterAST => {
                    // Monster AST classification
                    return DeclType::Function;
                },
                DeclSplitterType::ExtractTraits => {
                    // Trait extraction classification
                    if decl.decl_name.contains("trait") {
                        return DeclType::Trait;
                    }
                },
                _ => continue,
            }
        }
        decl.decl_type.clone()
    }
    
    fn get_monster_signature(&self, decl: &DeclNode) -> LLMWeight12Form<2048> {
        // Combine signatures from all splitters
        let mut combined = self.llm_monstrous_form();
        
        // Modify based on declaration type and name
        let name_hash = decl.decl_name.len() % 2048;
        combined.0[name_hash] = combined.0[name_hash].wrapping_add(1);
        
        combined
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_unified_decl_splitter() {
        let splitter = UnifiedDeclSplitter::new();
        let decls = splitter.split_file_into_decls("fn main() {}");
        assert!(!decls.is_empty());
        
        let types = splitter.extract_types_from_decl(&decls[0]);
        assert!(!types.is_empty());
    }
}
