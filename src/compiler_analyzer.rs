use crate::trait_extractor::{TraitExtractor, CodeBlock, TraitSignature};
use std::collections::HashSet;

pub struct CompilerAnalyzer {
    extractor: TraitExtractor,
}

impl CompilerAnalyzer {
    pub fn new() -> Self {
        Self {
            extractor: TraitExtractor::new(),
        }
    }

    pub fn analyze_rust_compiler_blocks(&mut self) {
        // Extract core compiler traits
        self.extractor.extract_trait(
            "Parser",
            vec!["TokenStream".to_string()],
            vec!["AST".to_string()],
        );
        
        self.extractor.extract_trait(
            "TypeChecker", 
            vec!["AST".to_string()],
            vec!["TypedAST".to_string()],
        );
        
        self.extractor.extract_trait(
            "CodeGenerator",
            vec!["TypedAST".to_string()],
            vec!["LLVM_IR".to_string()],
        );

        // Add compiler blocks
        self.extractor.add_code_block(CodeBlock {
            id: "rustc_parse".to_string(),
            consumes: ["source_code".to_string()].into_iter().collect(),
            produces: ["ast".to_string()].into_iter().collect(),
            external_deps: ["syn".to_string(), "proc_macro2".to_string()].into_iter().collect(),
        });

        self.extractor.add_code_block(CodeBlock {
            id: "rustc_hir".to_string(),
            consumes: ["ast".to_string()].into_iter().collect(),
            produces: ["hir".to_string()].into_iter().collect(),
            external_deps: ["rustc_hir".to_string()].into_iter().collect(),
        });
    }

    pub fn generate_dummy_externals(&self) -> Vec<String> {
        let externals = ["syn", "proc_macro2", "rustc_hir", "serde", "tokio"];
        externals.iter()
            .map(|ext| self.extractor.create_dummy_external(ext))
            .collect()
    }

    pub fn verify_with_minizinc(&self) -> crate::minizinc_data::MinizincInput {
        self.extractor.to_minizinc_constraints()
    }

    pub fn enumerate_trait_types(&self) -> Vec<String> {
        vec![
            "Parser".to_string(),
            "TypeChecker".to_string(), 
            "CodeGenerator".to_string(),
            "Optimizer".to_string(),
            "Linker".to_string(),
        ]
    }
}
