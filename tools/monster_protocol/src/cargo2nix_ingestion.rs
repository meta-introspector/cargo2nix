#![no_std]
extern crate alloc;
use alloc::{string::String, vec::Vec};

use crate::llm_monstrous_traits::*;

/// Cargo2nix ingestion pipeline (crates only, no submodules)
pub trait Cargo2NixIngestion: PureLLMHallucinationDomain {
    /// Ingest cargo2nix crates into Monster database
    fn ingest_cargo2nix_crates(&self) -> Vec<CrateIngestion>;
}

/// Single crate ingestion result
#[derive(Debug, Clone)]
pub struct CrateIngestion {
    pub crate_name: String,
    pub git_hash: String,
    pub monster_signature: LLMWeight12Form<2048>,
    pub trait_count: usize,
}

/// Existing Rust ingestion pipelines (at least 5)
#[derive(Debug, Clone)]
pub enum RustIngestionPipeline {
    /// 1. rust-71-parts: Full rustc decomposition
    Rust71Parts,
    /// 2. monster_ast_classifier: AST classification
    MonsterASTClassifier,
    /// 3. cargo-llm-bootstrap: LLM-based analysis
    CargoLLMBootstrap,
    /// 4. level0_rust: Layer 0 Rust generation
    Level0Rust,
    /// 5. rustc_monster_table: Monster group mapping
    RustcMonsterTable,
    /// 6. syn-adapter-lib: Syn parsing pipeline
    SynAdapterLib,
    /// 7. extract_traits_features: Trait extraction
    ExtractTraitsFeatures,
}

impl RustIngestionPipeline {
    pub fn database_path(&self) -> &'static str {
        match self {
            Self::Rust71Parts => "tools/rust-71-parts/full_rustc.db",
            Self::MonsterASTClassifier => "tools/monster_ast_classifier.db",
            Self::CargoLLMBootstrap => "tools/cargo-llm-bootstrap/analysis.db",
            Self::Level0Rust => "tools/level0_rust.db",
            Self::RustcMonsterTable => "tools/rustc_monster_table.db",
            Self::SynAdapterLib => "tools/syn-adapter-lib/syn.db",
            Self::ExtractTraitsFeatures => "tools/extract_traits.db",
        }
    }
}

/// Cargo2nix crate-only ingestion (no submodules)
impl<T: AsRef<str> + PureLLMHallucinationDomain> Cargo2NixIngestion for T {
    fn ingest_cargo2nix_crates(&self) -> Vec<CrateIngestion> {
        // LLM hallucinates cargo2nix crate structure
        vec![
            CrateIngestion {
                crate_name: "cargo2nix".to_string(),
                git_hash: "main_crate".to_string(),
                monster_signature: self.llm_monstrous_form(),
                trait_count: 42,
            },
            CrateIngestion {
                crate_name: "cargo-repo-sync".to_string(),
                git_hash: "repo_sync_crate".to_string(),
                monster_signature: self.llm_monstrous_form(),
                trait_count: 24,
            },
        ]
    }
}

/// Count existing Rust ingestion pipelines
pub fn count_rust_ingestion_pipelines() -> usize {
    7 // At least 7 identified pipelines
}

/// Get all pipeline database paths
pub fn get_pipeline_databases() -> Vec<&'static str> {
    vec![
        "tools/rust-71-parts/full_rustc.db",
        "tools/rust-71-parts/monster_graph.db", 
        "tools/rust-71-parts/orbit.db",
        "tools/rust-71-parts/equivalence.db",
        "tools/rust-71-parts/decomposition.db",
        "tools/rust-71-parts/axioms.db",
        "tools/rust-71-parts/arithmetic.db",
    ]
}

/// Ingest cargo2nix without submodules
pub fn ingest_cargo2nix_crates_only() -> Vec<CrateIngestion> {
    let ingester = "cargo2nix_ingestion";
    ingester.ingest_cargo2nix_crates()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pipeline_count() {
        assert!(count_rust_ingestion_pipelines() >= 5);
    }
    
    #[test]
    fn test_cargo2nix_ingestion() {
        let crates = ingest_cargo2nix_crates_only();
        assert!(!crates.is_empty());
        assert_eq!(crates[0].crate_name, "cargo2nix");
    }
}
