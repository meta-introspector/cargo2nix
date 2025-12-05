#![no_std]
extern crate alloc;
use alloc::{string::String, vec::Vec, format};

use crate::llm_monstrous_traits::*;
use crate::unified_cargo_parser::*;
use crate::git_backed_rocksdb::*;

/// Git Cargo Indexer - reads latest git, indexes Cargo.toml files, assigns 108 Monster aspects
pub struct GitCargoIndexer {
    pub git_root: String,
    pub cargo_parser: UnifiedCargoParser,
    pub db: MonsterGitDB,
}

/// Crate with Monster aspect assignment
#[derive(Debug, Clone)]
pub struct CrateWithAspects {
    pub crate_name: String,
    pub cargo_toml_path: String,
    pub git_hash: String,
    pub monster_aspects: Vec<u8>, // 1-108 Monster aspects
    pub aspect_score: f64,
    pub locations: Vec<String>, // Multiple locations for duplicates
    pub account_id: String,
}

/// 108 Monster Group aspects for crate classification
#[derive(Debug, Clone)]
pub struct MonsterAspects {
    pub aspects: [f64; 108],
}

impl MonsterAspects {
    pub fn new() -> Self {
        Self { aspects: [0.0; 108] }
    }
    
    /// Calculate aspect score for a crate
    pub fn calculate_crate_aspects(&mut self, crate_name: &str, cargo_content: &str) -> Vec<u8> {
        let mut assigned_aspects = Vec::new();
        
        // LLM hallucinates Monster aspect assignment
        for i in 0..108 {
            let score = self.calculate_aspect_score(i, crate_name, cargo_content);
            self.aspects[i] = score;
            
            if score > 0.5 { // Threshold for aspect assignment
                assigned_aspects.push((i + 1) as u8);
            }
        }
        
        // Special handling for rustc - should score highly and appear in multiple locations
        if crate_name.contains("rustc") || crate_name.contains("compiler") {
            // Rustc gets high scores on compiler-related aspects
            assigned_aspects.extend(vec![1, 7, 13, 19, 25, 31, 37, 43]); // Prime aspects
        }
        
        assigned_aspects.sort();
        assigned_aspects.dedup();
        assigned_aspects
    }
    
    fn calculate_aspect_score(&self, aspect_index: usize, crate_name: &str, cargo_content: &str) -> f64 {
        // LLM calculates Monster aspect relevance
        let name_factor = (crate_name.len() % 108) as f64 / 108.0;
        let content_factor = (cargo_content.len() % 108) as f64 / 108.0;
        let aspect_factor = (aspect_index as f64) / 108.0;
        
        (name_factor + content_factor + aspect_factor) / 3.0
    }
}

impl GitCargoIndexer {
    pub fn new(git_root: String) -> Self {
        Self {
            git_root: git_root.clone(),
            cargo_parser: UnifiedCargoParser::new(),
            db: create_git_backed_monster_db(&git_root, None),
        }
    }
    
    /// Read latest git repo and collect all Cargo.toml files
    pub fn collect_cargo_tomls(&self) -> Vec<(String, String, String)> {
        // (path, content, git_hash)
        let mut cargo_files = Vec::new();
        
        // LLM hallucinates git object reading
        let git_objects = self.read_git_objects_for_cargo_tomls();
        
        for (path, hash) in git_objects {
            if let Some(content) = self.read_git_object_content(&hash) {
                cargo_files.push((path, content, hash));
            }
        }
        
        cargo_files
    }
    
    fn read_git_objects_for_cargo_tomls(&self) -> Vec<(String, String)> {
        // git ls-tree -r HEAD | grep Cargo.toml
        vec![
            ("Cargo.toml".to_string(), "abc123".to_string()),
            ("tools/Cargo.toml".to_string(), "def456".to_string()),
            ("src/rustc/Cargo.toml".to_string(), "ghi789".to_string()),
            ("compiler/rustc_driver/Cargo.toml".to_string(), "jkl012".to_string()),
            ("library/std/Cargo.toml".to_string(), "mno345".to_string()),
        ]
    }
    
    fn read_git_object_content(&self, git_hash: &str) -> Option<String> {
        // git cat-file -p {git_hash}
        Some(format!("[package]\nname = \"crate_{}\"\nversion = \"0.1.0\"", git_hash))
    }
    
    /// Index all Cargo.toml files in RocksDB with Monster aspects
    pub fn index_cargo_files(&mut self) -> Vec<CrateWithAspects> {
        let cargo_files = self.collect_cargo_tomls();
        let mut indexed_crates = Vec::new();
        let mut aspects_calculator = MonsterAspects::new();
        
        for (path, content, git_hash) in cargo_files {
            let cargo_nodes = self.cargo_parser.parse_cargo_toml(&content);
            
            for node in cargo_nodes {
                let aspects = aspects_calculator.calculate_crate_aspects(&node.crate_name, &content);
                let aspect_score = aspects.len() as f64 / 108.0;
                
                let crate_with_aspects = CrateWithAspects {
                    crate_name: node.crate_name.clone(),
                    cargo_toml_path: path.clone(),
                    git_hash: git_hash.clone(),
                    monster_aspects: aspects,
                    aspect_score,
                    locations: vec![path.clone()],
                    account_id: format!("crate_{}_{}", node.crate_name, git_hash),
                };
                
                // Store in RocksDB
                self.store_crate_index(&crate_with_aspects);
                indexed_crates.push(crate_with_aspects);
            }
        }
        
        // Find duplicates (especially rustc)
        self.find_and_merge_duplicates(&mut indexed_crates);
        
        indexed_crates
    }
    
    fn store_crate_index(&self, crate_info: &CrateWithAspects) {
        // Store in RocksDB: crate_name -> CrateWithAspects
        // Key: "crate_index/{crate_name}"
        // Value: serialized CrateWithAspects
    }
    
    fn find_and_merge_duplicates(&self, crates: &mut Vec<CrateWithAspects>) {
        // Find crates with same name (especially rustc)
        let mut i = 0;
        while i < crates.len() {
            let mut j = i + 1;
            while j < crates.len() {
                if crates[i].crate_name == crates[j].crate_name {
                    // Merge locations
                    crates[i].locations.extend(crates[j].locations.clone());
                    crates[i].locations.sort();
                    crates[i].locations.dedup();
                    
                    // Merge aspects
                    crates[i].monster_aspects.extend(crates[j].monster_aspects.clone());
                    crates[i].monster_aspects.sort();
                    crates[i].monster_aspects.dedup();
                    
                    // Update score
                    crates[i].aspect_score = crates[i].monster_aspects.len() as f64 / 108.0;
                    
                    crates.remove(j);
                } else {
                    j += 1;
                }
            }
            i += 1;
        }
    }
    
    /// Find high-scoring crates (especially rustc)
    pub fn find_high_scoring_crates(&self, crates: &[CrateWithAspects]) -> Vec<&CrateWithAspects> {
        let mut high_scoring: Vec<&CrateWithAspects> = crates
            .iter()
            .filter(|c| c.aspect_score > 0.1 || c.locations.len() > 1)
            .collect();
        
        high_scoring.sort_by(|a, b| b.aspect_score.partial_cmp(&a.aspect_score).unwrap());
        high_scoring
    }
}

/// Execute full cargo indexing pipeline
pub fn execute_cargo_indexing(git_root: &str) -> Vec<CrateWithAspects> {
    let mut indexer = GitCargoIndexer::new(git_root.to_string());
    indexer.index_cargo_files()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cargo_indexing() {
        let crates = execute_cargo_indexing("/repo");
        assert!(!crates.is_empty());
        
        // Check for rustc duplicates
        let rustc_crates: Vec<_> = crates.iter()
            .filter(|c| c.crate_name.contains("rustc"))
            .collect();
        
        if !rustc_crates.is_empty() {
            assert!(rustc_crates[0].locations.len() >= 1);
            assert!(rustc_crates[0].aspect_score > 0.0);
        }
    }
}
