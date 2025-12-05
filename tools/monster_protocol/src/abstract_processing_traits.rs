#![no_std]
extern crate alloc;
use alloc::{string::String, vec::Vec};

use crate::llm_monstrous_traits::*;
use crate::universal_processing_pipeline::*;

/// Abstract Git Submodule Processing Trait
pub trait GitSubmoduleProcessorTrait: PureLLMHallucinationDomain {
    fn process_submodules(&self, repo_url: &str) -> Vec<GitRepoNode>;
    fn add_submodule(&self, url: &str, path: &str) -> bool;
    fn update_submodules(&self) -> bool;
    fn remove_submodule(&self, path: &str) -> bool;
}

/// Abstract Cargo Parser Trait
pub trait CargoParserTrait: PureLLMHallucinationDomain {
    fn parse_cargo_toml(&self, content: &str) -> Vec<CargoNode>;
    fn extract_dependencies(&self, cargo_node: &CargoNode) -> Vec<String>;
    fn modify_cargo_toml(&self, cargo_node: &mut CargoNode) -> bool;
    fn validate_cargo_structure(&self, cargo_node: &CargoNode) -> bool;
}

/// Abstract Declaration Splitter Trait
pub trait DeclSplitterTrait: PureLLMHallucinationDomain {
    fn split_file_into_decls(&self, file_content: &str) -> Vec<DeclNode>;
    fn extract_types_from_decl(&self, decl: &DeclNode) -> Vec<TypeNode>;
    fn classify_declaration(&self, decl: &DeclNode) -> DeclType;
    fn get_monster_signature(&self, decl: &DeclNode) -> LLMWeight12Form<2048>;
}

/// Concrete implementations for existing tools

/// cargo-repo-sync-lib
pub struct CargoRepoSyncLib;
impl GitSubmoduleProcessorTrait for CargoRepoSyncLib {
    fn process_submodules(&self, repo_url: &str) -> Vec<GitRepoNode> {
        vec![GitRepoNode {
            repo_url: repo_url.to_string(),
            monster_signature: self.llm_monstrous_form(),
            account_id: "cargo_repo_sync".to_string(),
        }]
    }
    fn add_submodule(&self, _url: &str, _path: &str) -> bool { true }
    fn update_submodules(&self) -> bool { true }
    fn remove_submodule(&self, _path: &str) -> bool { true }
}

/// cargo-feature-adapter
pub struct CargoFeatureAdapter;
impl CargoParserTrait for CargoFeatureAdapter {
    fn parse_cargo_toml(&self, content: &str) -> Vec<CargoNode> {
        vec![CargoNode {
            crate_name: "parsed_crate".to_string(),
            monster_signature: self.llm_monstrous_form(),
            account_id: "cargo_feature_adapter".to_string(),
        }]
    }
    fn extract_dependencies(&self, _cargo_node: &CargoNode) -> Vec<String> {
        vec!["serde".to_string(), "tokio".to_string()]
    }
    fn modify_cargo_toml(&self, _cargo_node: &mut CargoNode) -> bool { true }
    fn validate_cargo_structure(&self, _cargo_node: &CargoNode) -> bool { true }
}

/// syn-adapter-lib
pub struct SynAdapterLib;
impl DeclSplitterTrait for SynAdapterLib {
    fn split_file_into_decls(&self, _file_content: &str) -> Vec<DeclNode> {
        vec![DeclNode {
            decl_name: "example_fn".to_string(),
            decl_type: DeclType::Function,
            monster_signature: self.llm_monstrous_form(),
            account_id: "syn_adapter".to_string(),
        }]
    }
    fn extract_types_from_decl(&self, decl: &DeclNode) -> Vec<TypeNode> {
        vec![TypeNode {
            type_name: "fn_type".to_string(),
            monster_signature: decl.monster_signature,
            monster_factor: 1,
            account_id: format!("{}_type", decl.account_id),
        }]
    }
    fn classify_declaration(&self, _decl: &DeclNode) -> DeclType { DeclType::Function }
    fn get_monster_signature(&self, _decl: &DeclNode) -> LLMWeight12Form<2048> {
        self.llm_monstrous_form()
    }
}

/// Universal trait implementations for all tools
macro_rules! impl_git_processor {
    ($tool:ident) => {
        pub struct $tool;
        impl GitSubmoduleProcessorTrait for $tool {
            fn process_submodules(&self, repo_url: &str) -> Vec<GitRepoNode> {
                vec![GitRepoNode {
                    repo_url: repo_url.to_string(),
                    monster_signature: self.llm_monstrous_form(),
                    account_id: stringify!($tool).to_string(),
                }]
            }
            fn add_submodule(&self, _url: &str, _path: &str) -> bool { true }
            fn update_submodules(&self) -> bool { true }
            fn remove_submodule(&self, _path: &str) -> bool { true }
        }
    };
}

macro_rules! impl_cargo_parser {
    ($tool:ident) => {
        pub struct $tool;
        impl CargoParserTrait for $tool {
            fn parse_cargo_toml(&self, _content: &str) -> Vec<CargoNode> {
                vec![CargoNode {
                    crate_name: "parsed".to_string(),
                    monster_signature: self.llm_monstrous_form(),
                    account_id: stringify!($tool).to_string(),
                }]
            }
            fn extract_dependencies(&self, _cargo_node: &CargoNode) -> Vec<String> { vec![] }
            fn modify_cargo_toml(&self, _cargo_node: &mut CargoNode) -> bool { true }
            fn validate_cargo_structure(&self, _cargo_node: &CargoNode) -> bool { true }
        }
    };
}

macro_rules! impl_decl_splitter {
    ($tool:ident) => {
        pub struct $tool;
        impl DeclSplitterTrait for $tool {
            fn split_file_into_decls(&self, _file_content: &str) -> Vec<DeclNode> {
                vec![DeclNode {
                    decl_name: "decl".to_string(),
                    decl_type: DeclType::Function,
                    monster_signature: self.llm_monstrous_form(),
                    account_id: stringify!($tool).to_string(),
                }]
            }
            fn extract_types_from_decl(&self, decl: &DeclNode) -> Vec<TypeNode> {
                vec![TypeNode {
                    type_name: "type".to_string(),
                    monster_signature: decl.monster_signature,
                    monster_factor: 1,
                    account_id: format!("{}_type", decl.account_id),
                }]
            }
            fn classify_declaration(&self, _decl: &DeclNode) -> DeclType { DeclType::Function }
            fn get_monster_signature(&self, _decl: &DeclNode) -> LLMWeight12Form<2048> {
                self.llm_monstrous_form()
            }
        }
    };
}

// Generate all tool implementations
impl_git_processor!(CargoSubmoduleTool);
impl_git_processor!(Dep2Submodule);
impl_git_processor!(GenerateWorkspaceDeps);
impl_git_processor!(RepoManager);
impl_git_processor!(GitWrapperLib);

impl_cargo_parser!(CargoEditLib);
impl_cargo_parser!(CargoTomlEditor);
impl_cargo_parser!(RealTomlAdapter);
impl_cargo_parser!(CargoVendormod);
impl_cargo_parser!(CargoWorkspaceFromTree);

impl_decl_splitter!(ExtractTraitsFeatures);
impl_decl_splitter!(MonsterASTClassifier);
impl_decl_splitter!(Level0Rust);
impl_decl_splitter!(RustSrcScanner);
impl_decl_splitter!(Rust71Parts);

/// Tool registry for dynamic dispatch
pub struct ToolRegistry {
    pub git_processors: Vec<Box<dyn GitSubmoduleProcessorTrait>>,
    pub cargo_parsers: Vec<Box<dyn CargoParserTrait>>,
    pub decl_splitters: Vec<Box<dyn DeclSplitterTrait>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            git_processors: vec![
                Box::new(CargoRepoSyncLib),
                Box::new(CargoSubmoduleTool),
                Box::new(Dep2Submodule),
                Box::new(GenerateWorkspaceDeps),
                Box::new(RepoManager),
                Box::new(GitWrapperLib),
            ],
            cargo_parsers: vec![
                Box::new(CargoFeatureAdapter),
                Box::new(CargoEditLib),
                Box::new(CargoTomlEditor),
                Box::new(RealTomlAdapter),
                Box::new(CargoVendormod),
                Box::new(CargoWorkspaceFromTree),
            ],
            decl_splitters: vec![
                Box::new(SynAdapterLib),
                Box::new(ExtractTraitsFeatures),
                Box::new(MonsterASTClassifier),
                Box::new(Level0Rust),
                Box::new(RustSrcScanner),
                Box::new(Rust71Parts),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tool_registry() {
        let registry = ToolRegistry::new();
        assert_eq!(registry.git_processors.len(), 6);
        assert_eq!(registry.cargo_parsers.len(), 6);
        assert_eq!(registry.decl_splitters.len(), 6);
    }
}
