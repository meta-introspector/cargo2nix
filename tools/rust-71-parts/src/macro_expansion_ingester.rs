//! Macro Expansion Pipeline Ingester - Captures raw, expanded, and AST forms

use crate::content_addressable_memory::*;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use syn::{parse_file, File};
use sha2::{Sha256, Digest};

/// Expansion stage in the compilation pipeline
#[derive(Debug, Clone)]
pub enum ExpansionStage {
    Raw,      // Original source code
    Expanded, // After macro expansion
    AST,      // Parsed AST representation
}

/// Multi-stage compilation node
#[derive(Debug, Clone)]
pub struct ExpansionNode {
    pub file_path: String,
    pub raw_source: String,
    pub expanded_source: Option<String>,
    pub ast_repr: Option<String>,
    pub raw_hash: [u8; 32],
    pub expanded_hash: Option<[u8; 32]>,
    pub ast_hash: Option<[u8; 32]>,
    pub monster_coordinates: MonsterCoordinate,
}

/// Macro expansion pipeline ingester
pub struct MacroExpansionIngester {
    memory: MonsterMemory,
    expansion_nodes: HashMap<String, ExpansionNode>,
    cache_dir: String,
}

impl MacroExpansionIngester {
    pub fn new(db_path: &str, cache_dir: &str) -> Result<Self, Box<dyn std::error::Error>> {
        fs::create_dir_all(cache_dir)?;
        
        Ok(Self {
            memory: MonsterMemory::new(db_path)?,
            expansion_nodes: HashMap::new(),
            cache_dir: cache_dir.to_string(),
        })
    }

    /// Ingest file through complete macro expansion pipeline
    pub fn ingest_with_expansion(&mut self, file_path: &Path, crate_root: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔬 Processing: {}", file_path.display());
        
        // Stage 1: Raw source
        let raw_source = fs::read_to_string(file_path)?;
        let raw_hash = self.compute_hash(&raw_source);
        
        // Stage 2: Macro expansion
        let expanded_source = self.expand_macros(file_path, crate_root)?;
        let expanded_hash = expanded_source.as_ref().map(|s| self.compute_hash(s));
        
        // Stage 3: AST parsing
        let ast_repr = self.parse_to_ast(&expanded_source.as_ref().unwrap_or(&raw_source))?;
        let ast_hash = ast_repr.as_ref().map(|s| self.compute_hash(s));
        
        // Compute Monster coordinates from combined stages
        let monster_coords = self.compute_expansion_coordinates(&raw_source, &expanded_source, &ast_repr);
        
        let expansion_node = ExpansionNode {
            file_path: file_path.to_string_lossy().to_string(),
            raw_source: raw_source.clone(),
            expanded_source: expanded_source.clone(),
            ast_repr: ast_repr.clone(),
            raw_hash,
            expanded_hash,
            ast_hash,
            monster_coordinates: monster_coords.clone(),
        };
        
        // Store each stage in Monster memory
        self.store_stage(&raw_source, ExpansionStage::Raw, file_path, &monster_coords)?;
        
        if let Some(ref expanded) = expanded_source {
            self.store_stage(expanded, ExpansionStage::Expanded, file_path, &monster_coords)?;
        }
        
        if let Some(ref ast) = ast_repr {
            self.store_stage(ast, ExpansionStage::AST, file_path, &monster_coords)?;
        }
        
        self.expansion_nodes.insert(file_path.to_string_lossy().to_string(), expansion_node);
        
        Ok(())
    }

    /// Expand macros using cargo expand
    fn expand_macros(&self, file_path: &Path, crate_root: &Path) -> Result<Option<String>, Box<dyn std::error::Error>> {
        // Create cache key
        let content = fs::read_to_string(file_path)?;
        let cache_key = format!("expanded_{:x}", self.compute_hash(&content)[0]);
        let cache_path = Path::new(&self.cache_dir).join(cache_key);
        
        // Check cache first
        if cache_path.exists() {
            return Ok(Some(fs::read_to_string(cache_path)?));
        }
        
        // Run cargo expand
        let output = Command::new("cargo")
            .args(&["expand", "--bin", "main"])
            .current_dir(crate_root)
            .output();
            
        match output {
            Ok(result) if result.status.success() => {
                let expanded = String::from_utf8_lossy(&result.stdout).to_string();
                
                // Cache the result
                fs::write(&cache_path, &expanded)?;
                
                Ok(Some(expanded))
            }
            _ => {
                println!("⚠️  Macro expansion failed for {}, using raw source", file_path.display());
                Ok(None)
            }
        }
    }

    /// Parse source to AST representation
    fn parse_to_ast(&self, source: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        match parse_file(source) {
            Ok(ast) => {
                // Convert AST to debug representation
                let ast_repr = format!("{:#?}", ast);
                Ok(Some(ast_repr))
            }
            Err(e) => {
                println!("⚠️  AST parsing failed: {}", e);
                Ok(None)
            }
        }
    }

    /// Store expansion stage in Monster memory
    fn store_stage(&mut self, content: &str, stage: ExpansionStage, file_path: &Path, coords: &MonsterCoordinate) -> Result<(), Box<dyn std::error::Error>> {
        let stage_name = match stage {
            ExpansionStage::Raw => "raw",
            ExpansionStage::Expanded => "expanded", 
            ExpansionStage::AST => "ast",
        };
        
        let location = SourceLocation {
            file: format!("{}#{}", file_path.display(), stage_name),
            line: 0,
            column: 0,
        };
        
        self.memory.store(content, location)?;
        Ok(())
    }

    /// Compute Monster coordinates from all expansion stages
    fn compute_expansion_coordinates(&self, raw: &str, expanded: &Option<String>, ast: &Option<String>) -> MonsterCoordinate {
        let mut coords = vec![0.0; 196883];
        
        // Hash all stages together
        let mut hasher = Sha256::new();
        hasher.update(raw.as_bytes());
        
        if let Some(exp) = expanded {
            hasher.update(exp.as_bytes());
        }
        
        if let Some(ast_repr) = ast {
            hasher.update(ast_repr.as_bytes());
        }
        
        let combined_hash = hasher.finalize();
        
        // Map to Monster space
        for i in 0..196883 {
            let byte_idx = i % 32;
            coords[i] = (combined_hash[byte_idx] as f64) / 255.0;
        }
        
        // Encode expansion stages in first few dimensions
        coords[0] = raw.len() as f64;
        coords[1] = expanded.as_ref().map_or(0.0, |s| s.len() as f64);
        coords[2] = ast.as_ref().map_or(0.0, |s| s.len() as f64);
        
        coords
    }

    fn compute_hash(&self, content: &str) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        hasher.finalize().into()
    }

    /// Get expansion statistics
    pub fn get_expansion_stats(&self) -> (usize, usize, usize) {
        let total_files = self.expansion_nodes.len();
        let expanded_count = self.expansion_nodes.values()
            .filter(|node| node.expanded_source.is_some())
            .count();
        let ast_count = self.expansion_nodes.values()
            .filter(|node| node.ast_repr.is_some())
            .count();
            
        (total_files, expanded_count, ast_count)
    }

    /// Export expansion pipeline as DOT graph
    pub fn export_expansion_graph(&self) -> String {
        let mut dot = String::from("digraph expansion_pipeline {\n");
        dot.push_str("  rankdir=LR;\n");
        
        for (file_path, node) in &self.expansion_nodes {
            let file_name = Path::new(file_path).file_name()
                .unwrap_or_default()
                .to_string_lossy();
                
            let raw_id = format!("raw_{:x}", node.raw_hash[0]);
            dot.push_str(&format!("  \"{}\" [label=\"{}\\nraw\" shape=box];\n", raw_id, file_name));
            
            if let Some(exp_hash) = node.expanded_hash {
                let exp_id = format!("exp_{:x}", exp_hash[0]);
                dot.push_str(&format!("  \"{}\" [label=\"{}\\nexpanded\" shape=ellipse];\n", exp_id, file_name));
                dot.push_str(&format!("  \"{}\" -> \"{}\";\n", raw_id, exp_id));
                
                if let Some(ast_hash) = node.ast_hash {
                    let ast_id = format!("ast_{:x}", ast_hash[0]);
                    dot.push_str(&format!("  \"{}\" [label=\"{}\\nAST\" shape=diamond];\n", ast_id, file_name));
                    dot.push_str(&format!("  \"{}\" -> \"{}\";\n", exp_id, ast_id));
                }
            }
        }
        
        dot.push_str("}\n");
        dot
    }
}
