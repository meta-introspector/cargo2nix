use anyhow::{Context, Result};
use std::collections::HashSet;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use lazy_static::lazy_static;
use monster_math_traits::{Declaration, RustAstParser};
use once_cell::sync::Lazy;
use prelude_generator::use_extractor::expand_macros_and_parse;
use prelude_generator::use_extractor::rustc_info::get_rustc_info;
use quote::ToTokens;
use regex::Regex;
use split_expanded_lib::{ErrorSample, RustcInfo as SplitExpandedRustcInfo};
use syn::{
    self, visit::Visit, ItemConst, ItemEnum, ItemFn, ItemMod, ItemStatic, ItemStruct, ItemTrait,
    ItemType, ItemUnion,
};
use tokio::runtime::Runtime;

lazy_static! {
    static ref TOKIO_RUNTIME: Runtime = Runtime::new().expect("Failed to create Tokio runtime");
}

/// A real implementation of `RustAstParser` using `syn` and macro expansion.
#[derive(Debug, Default)]
pub struct RealRustAstParser;

impl RealRustAstParser {
    fn extract_declarations_from_syn_file(
        syn_file: &syn::File,
        file_path: &Path,
    ) -> Vec<Declaration> {
        let mut visitor = AstDeclarationVisitor::new(file_path.to_path_buf());
        visitor.visit_file(syn_file);
        visitor.declarations
    }
}

impl RustAstParser for RealRustAstParser {
    fn parse_rust_code(&self, code: &str) -> Vec<Declaration> {
        TOKIO_RUNTIME.block_on(async {
            let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");
            let temp_crate_path = temp_dir.path();
            let temp_src_dir = temp_crate_path.join("src");
            fs::create_dir_all(&temp_src_dir).expect("Failed to create temporary src directory");

            let lib_rs_path = temp_src_dir.join("lib.rs");
            fs::write(&lib_rs_path, code).expect("Failed to write code to temporary lib.rs");

            let cargo_toml_path = temp_crate_path.join("Cargo.toml");
            fs::write(
                &cargo_toml_path,
                r#"[package]
name = "temp_crate"
version = "0.1.0"
edition = "2021"

[lib]
path = "src/lib.rs"

[dependencies]
anyhow = "1.0"
tokio = { version = "1", features = ["full"] }
syn = { version = "2.0", features = ["full", "extra-traits"] }
quote = "1.0"
proc-macro2 = { version = "1.0", features = ["span-locations"] }
serde = { version = "1.0", features = ["derive"] }
lazy_static = "1.4.0"
once_cell = "1.19.0"
regex = "1"
split-expanded-lib = { path = "../../tools/rust-bootstrap-nix/split-expanded-lib" }
prelude-generator = { path = "../../tools/rust-bootstrap-nix/prelude-generator" }
"#,
            )
            .expect("Failed to write Cargo.toml");

            let rustc_info = get_rustc_info().expect("Failed to get rustc info");
            let cache_dir = temp_crate_path.join(".prelude_cache");
            fs::create_dir_all(&cache_dir).expect("Failed to create cache directory");

            // Create a dummy writer for expand_macros_and_parse
            let mut writer = Vec::new(); // Use Vec<u8> as a buffer for the writer

            let (syn_file, error_sample) = expand_macros_and_parse(
                &mut writer,
                &lib_rs_path,
                temp_crate_path,
                &cargo_toml_path,
                &rustc_info,
                &cache_dir,
            )
            .await
            .expect("Failed to expand macros and parse code");

            if let Some(error) = error_sample {
                eprintln!("Error during macro expansion: {:?}", error);
                return Vec::new();
            }

            RealRustAstParser::extract_declarations_from_syn_file(&syn_file, &lib_rs_path)
        })
    }
}

/// A `syn::visit::Visit` implementation to extract `Declaration`s from a `syn::File`.
#[derive(Debug, Default)]
struct AstDeclarationVisitor {
    declarations: Vec<Declaration>,
    current_file_path: PathBuf,
}

impl AstDeclarationVisitor {
    fn new(current_file_path: PathBuf) -> Self {
        AstDeclarationVisitor {
            declarations: Vec::new(),
            current_file_path,
        }
    }

    fn add_declaration(&mut self, kind: String, name: String, item_attrs: &[syn::Attribute]) {
        let is_public = item_attrs.iter().any(|attr| attr.path().is_ident("pub"));
        let mut attributes = HashSet::new();
        for attr in item_attrs {
            attributes.insert(attr.to_token_stream().to_string());
        }

        self.declarations.push(Declaration {
            kind,
            name,
            path: self.current_file_path.to_string_lossy().to_string(),
            semantic_hash: None,
            monster_factors: None,
            bag_of_words: None,
            eight_d_coordinate: None,
            deps: HashSet::new(), // To be filled later by dependency analysis
            is_public,
            attributes,
        });
    }
}

impl<'ast> Visit<'ast> for AstDeclarationVisitor {
    fn visit_item_const(&mut self, i: &'ast ItemConst) {
        self.add_declaration("const".to_string(), i.ident.to_string(), &i.attrs);
        syn::visit::visit_item_const(self, i);
    }

    fn visit_item_enum(&mut self, i: &'ast ItemEnum) {
        self.add_declaration("enum".to_string(), i.ident.to_string(), &i.attrs);
        syn::visit::visit_item_enum(self, i);
    }

    fn visit_item_fn(&mut self, i: &'ast ItemFn) {
        self.add_declaration("fn".to_string(), i.sig.ident.to_string(), &i.attrs);
        syn::visit::visit_item_fn(self, i);
    }

    fn visit_item_mod(&mut self, i: &'ast ItemMod) {
        self.add_declaration("mod".to_string(), i.ident.to_string(), &i.attrs);
        syn::visit::visit_item_mod(self, i);
    }

    fn visit_item_static(&mut self, i: &'ast ItemStatic) {
        self.add_declaration("static".to_string(), i.ident.to_string(), &i.attrs);
        syn::visit::visit_item_static(self, i);
    }

    fn visit_item_struct(&mut self, i: &'ast ItemStruct) {
        self.add_declaration("struct".to_string(), i.ident.to_string(), &i.attrs);
        syn::visit::visit_item_struct(self, i);
    }

    fn visit_item_trait(&mut self, i: &'ast ItemTrait) {
        self.add_declaration("trait".to_string(), i.ident.to_string(), &i.attrs);
        syn::visit::visit_item_trait(self, i);
    }

    fn visit_item_type(&mut self, i: &'ast ItemType) {
        self.add_declaration("type_alias".to_string(), i.ident.to_string(), &i.attrs);
        syn::visit::visit_item_type(self, i);
    }

    fn visit_item_union(&mut self, i: &'ast ItemUnion) {
        self.add_declaration("union".to_string(), i.ident.to_string(), &i.attrs);
        syn::visit::visit_item_union(self, i);
    }
}
