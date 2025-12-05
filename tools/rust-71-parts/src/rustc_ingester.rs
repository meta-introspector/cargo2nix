//! Rustc Ingester - Break rustc into level0 blocks with dependency arrows

use crate::content_addressable_memory::*;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use syn::{parse_file, Item, UseTree, ItemUse};

/// Level0 block - atomic unit of rustc code
#[derive(Debug, Clone)]
pub struct Level0Block {
    pub hash: [u8; 32],
    pub content: String,
    pub file_path: String,
    pub line_range: (u32, u32),
    pub block_type: BlockType,
    pub dependencies: Vec<[u8; 32]>, // arrows to other blocks
}

#[derive(Debug, Clone)]
pub enum BlockType {
    Use,      // use statements
    Trait,    // trait definitions
    Impl,     // impl blocks
    Struct,   // struct definitions
    Enum,     // enum definitions
    Function, // function definitions
    Const,    // constants
    Module,   // mod declarations
}

/// Rustc ingestion engine
pub struct RustcIngester {
    memory: MonsterMemory,
    blocks: HashMap<[u8; 32], Level0Block>,
    dependency_graph: HashMap<[u8; 32], Vec<[u8; 32]>>,
}

impl RustcIngester {
    pub fn new(db_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            memory: MonsterMemory::new(db_path)?,
            blocks: HashMap::new(),
            dependency_graph: HashMap::new(),
        })
    }

    /// Ingest entire rustc directory
    pub fn ingest_rustc(&mut self, rustc_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔬 Ingesting rustc from: {}", rustc_path);
        
        self.walk_directory(rustc_path)?;
        self.build_dependency_graph()?;
        
        println!("✓ Ingested {} blocks", self.blocks.len());
        println!("✓ Built dependency graph with {} edges", 
            self.dependency_graph.values().map(|v| v.len()).sum::<usize>());
        
        Ok(())
    }

    fn walk_directory(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                self.walk_directory(path.to_str().unwrap())?;
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                self.process_rust_file(&path)?;
            }
        }
        Ok(())
    }

    fn process_rust_file(&mut self, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;
        let syntax_tree = parse_file(&content)?;
        
        for (i, item) in syntax_tree.items.iter().enumerate() {
            let block = self.extract_level0_block(item, file_path, i, &content)?;
            let hash = block.hash;
            
            // Store in Monster memory
            let location = SourceLocation {
                file: file_path.to_string_lossy().to_string(),
                line: block.line_range.0,
                column: 0,
            };
            self.memory.store(&block.content, location)?;
            
            self.blocks.insert(hash, block);
        }
        
        Ok(())
    }

    fn extract_level0_block(&self, item: &Item, file_path: &Path, index: usize, content: &str) -> Result<Level0Block, Box<dyn std::error::Error>> {
        let (block_type, item_content) = match item {
            Item::Use(use_item) => (BlockType::Use, quote::quote!(#use_item).to_string()),
            Item::Trait(trait_item) => (BlockType::Trait, quote::quote!(#trait_item).to_string()),
            Item::Impl(impl_item) => (BlockType::Impl, quote::quote!(#impl_item).to_string()),
            Item::Struct(struct_item) => (BlockType::Struct, quote::quote!(#struct_item).to_string()),
            Item::Enum(enum_item) => (BlockType::Enum, quote::quote!(#enum_item).to_string()),
            Item::Fn(fn_item) => (BlockType::Function, quote::quote!(#fn_item).to_string()),
            Item::Const(const_item) => (BlockType::Const, quote::quote!(#const_item).to_string()),
            Item::Mod(mod_item) => (BlockType::Module, quote::quote!(#mod_item).to_string()),
            _ => (BlockType::Function, format!("// Other item: {}", index)),
        };

        let hash = self.compute_block_hash(&item_content);
        let line_range = (index as u32 * 10, (index + 1) as u32 * 10); // Approximate

        Ok(Level0Block {
            hash,
            content: item_content,
            file_path: file_path.to_string_lossy().to_string(),
            line_range,
            block_type,
            dependencies: Vec::new(), // Will be filled later
        })
    }

    fn build_dependency_graph(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let block_hashes: Vec<[u8; 32]> = self.blocks.keys().cloned().collect();
        
        for hash in &block_hashes {
            if let Some(block) = self.blocks.get(hash) {
                let deps = self.find_dependencies(block)?;
                self.dependency_graph.insert(*hash, deps.clone());
                
                // Update block with dependencies
                if let Some(mut_block) = self.blocks.get_mut(hash) {
                    mut_block.dependencies = deps;
                }
            }
        }
        
        Ok(())
    }

    fn find_dependencies(&self, block: &Level0Block) -> Result<Vec<[u8; 32]>, Box<dyn std::error::Error>> {
        let mut deps = Vec::new();
        
        // Simple dependency detection - look for identifiers in other blocks
        for (other_hash, other_block) in &self.blocks {
            if *other_hash == block.hash { continue; }
            
            if self.blocks_depend(block, other_block) {
                deps.push(*other_hash);
            }
        }
        
        Ok(deps)
    }

    fn blocks_depend(&self, block: &Level0Block, other: &Level0Block) -> bool {
        // Simple heuristic: if block mentions something defined in other
        match (&block.block_type, &other.block_type) {
            (BlockType::Impl, BlockType::Trait) => block.content.contains(&self.extract_name(&other.content)),
            (BlockType::Function, BlockType::Struct) => block.content.contains(&self.extract_name(&other.content)),
            (BlockType::Use, _) => false, // Use statements don't depend on local items
            _ => false,
        }
    }

    fn extract_name(&self, content: &str) -> String {
        // Extract identifier name from content
        content.split_whitespace()
            .nth(1)
            .unwrap_or("")
            .trim_matches(|c: char| !c.is_alphanumeric() && c != '_')
            .to_string()
    }

    fn compute_block_hash(&self, content: &str) -> [u8; 32] {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        hasher.finalize().into()
    }

    /// Export dependency graph as DOT format
    pub fn export_dot(&self) -> String {
        let mut dot = String::from("digraph rustc_blocks {\n");
        
        for (hash, block) in &self.blocks {
            let label = format!("{}:{}", 
                self.block_type_name(&block.block_type),
                &hex::encode(&hash[..4])
            );
            dot.push_str(&format!("  \"{}\" [label=\"{}\"];\n", hex::encode(hash), label));
        }
        
        for (from_hash, deps) in &self.dependency_graph {
            for to_hash in deps {
                dot.push_str(&format!("  \"{}\" -> \"{}\";\n", 
                    hex::encode(from_hash), hex::encode(to_hash)));
            }
        }
        
        dot.push_str("}\n");
        dot
    }

    fn block_type_name(&self, block_type: &BlockType) -> &str {
        match block_type {
            BlockType::Use => "use",
            BlockType::Trait => "trait",
            BlockType::Impl => "impl",
            BlockType::Struct => "struct",
            BlockType::Enum => "enum",
            BlockType::Function => "fn",
            BlockType::Const => "const",
            BlockType::Module => "mod",
        }
    }

    pub fn get_stats(&self) -> (usize, usize) {
        (self.blocks.len(), self.dependency_graph.values().map(|v| v.len()).sum())
    }
}
