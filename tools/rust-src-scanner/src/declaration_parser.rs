use std::path::{Path, PathBuf};
use std::fs;
use syn::{Item};
use quote::ToTokens;
use syn::spanned::Spanned; // Import Spanned trait
use syn::visit; // Import visit module
use syn::visit::Visit; // Import Visit trait

use crate::error::AppError;
use crate::semantic_id::SemanticId; // Import SemanticId

#[derive(Debug, Clone)]
pub struct Declaration {
    pub semantic_id: SemanticId, // Use SemanticId
    pub name: String,
    pub kind: String, // e.g., "fn", "struct", "enum", "const"
    pub file_path: PathBuf,
    pub line: usize,
    pub raw_code: String, // The raw code of the declaration
    // Add fields for associated mathematical objects later
}

pub struct DeclarationParser;

// AST visitor to collect metrics for SemanticId
#[derive(Default)]
struct DeclarationAnalyzer {
    ast_types_count: usize,
    expressions_count: usize,
    // Add other metrics as needed
}

impl<'ast> Visit<'ast> for DeclarationAnalyzer {
    fn visit_type(&mut self, i: &'ast syn::Type) {
        self.ast_types_count += 1;
        visit::visit_type(self, i); // Continue walking the type
    }

    fn visit_expr(&mut self, i: &'ast syn::Expr) {
        self.expressions_count += 1;
        visit::visit_expr(self, i); // Continue walking the expression
    }

    // Implement other visit methods to collect more metrics
}

impl DeclarationParser {
    pub fn parse_rust_file(path: &Path) -> Result<Vec<Declaration>, AppError> {
        let content = fs::read_to_string(path).map_err(AppError::Io)?;
        let ast = syn::parse_file(&content).map_err(|e| AppError::Other(e.to_string()))?;

        let mut declarations = Vec::new();
        let mut unique_decl_idx_counter = 0; // Counter for unique_idx within this file

        for item in ast.items {
            let mut semantic_id = SemanticId::new(unique_decl_idx_counter);
            unique_decl_idx_counter += 1;

            let raw_code = item.to_token_stream().to_string();
            let line_count = raw_code.lines().count();
            semantic_id.weight = line_count as f64; // Simple weight: number of lines

            // Create an analyzer for this item
            let mut analyzer = DeclarationAnalyzer::default();
            analyzer.visit_item(&item); // Directly call visit_item on the analyzer

            semantic_id.ast_types_count = analyzer.ast_types_count;
            semantic_id.expressions_count = analyzer.expressions_count;

            // Determine line number for depth calculation
            let line_start = item.span().start().line;
            semantic_id.depth = line_start; // Simple depth: line number for now

            match item {
                Item::Fn(item_fn) => {
                    declarations.push(Declaration {
                        semantic_id,
                        name: item_fn.sig.ident.to_string(),
                        kind: "fn".to_string(),
                        file_path: path.to_path_buf(),
                        line: item_fn.span().start().line,
                        raw_code,
                    });
                }
                Item::Struct(item_struct) => {
                    declarations.push(Declaration {
                        semantic_id,
                        name: item_struct.ident.to_string(),
                        kind: "struct".to_string(),
                        file_path: path.to_path_buf(),
                        line: item_struct.span().start().line,
                        raw_code,
                    });
                }
                Item::Enum(item_enum) => {
                    declarations.push(Declaration {
                        semantic_id,
                        name: item_enum.ident.to_string(),
                        kind: "enum".to_string(),
                        file_path: path.to_path_buf(),
                        line: item_enum.span().start().line,
                        raw_code,
                    });
                }
                Item::Const(item_const) => {
                    declarations.push(Declaration {
                        semantic_id,
                        name: item_const.ident.to_string(),
                        kind: "const".to_string(),
                        file_path: path.to_path_buf(),
                        line: item_const.span().start().line,
                        raw_code,
                    });
                }
                Item::Static(item_static) => {
                    declarations.push(Declaration {
                        semantic_id,
                        name: item_static.ident.to_string(),
                        kind: "static".to_string(),
                        file_path: path.to_path_buf(),
                        line: item_static.span().start().line,
                        raw_code,
                    });
                }
                // Add other item types as needed (e.g., Trait, Impl, Mod, Use)
                _ => {}
            }
        }
        Ok(declarations)
    }
}
