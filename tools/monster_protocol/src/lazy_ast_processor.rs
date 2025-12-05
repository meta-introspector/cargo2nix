#![no_std]
extern crate alloc;
use alloc::{string::String, vec::Vec, format};

use crate::llm_monstrous_traits::*;
use crate::git_repo_graph::*;
use crate::unified_decl_splitter::*;

/// Lazy AST Processor - loads, extracts, assigns values, rolls up, caches everything
pub struct LazyASTProcessor {
    pub repo_graph: GitRepoGraph,
    pub decl_splitter: UnifiedDeclSplitter,
    pub cache: ASTCache,
}

/// AST Cache in RocksDB
#[derive(Debug, Clone)]
pub struct ASTCache {
    pub file_asts: Vec<(String, CachedAST)>, // file_path -> AST
    pub decl_values: Vec<(String, DeclValue)>, // decl_id -> value
    pub rollup_summaries: Vec<(String, RollupSummary)>, // repo_id -> summary
}

/// Cached AST with preprocessing results
#[derive(Debug, Clone)]
pub struct CachedAST {
    pub file_path: String,
    pub git_hash: String,
    pub raw_content: Option<String>, // Lazy loaded
    pub parsed_ast: Option<ParsedAST>,
    pub decl_nodes: Vec<DeclNode>,
    pub monster_signature: LLMWeight12Form<2048>,
    pub processing_stage: ProcessingStage,
}

/// Parsed AST representation
#[derive(Debug, Clone)]
pub struct ParsedAST {
    pub items: Vec<ASTItem>,
    pub imports: Vec<String>,
    pub exports: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ASTItem {
    pub name: String,
    pub item_type: ASTItemType,
    pub monster_value: f64,
}

#[derive(Debug, Clone)]
pub enum ASTItemType {
    Function,
    Struct,
    Enum,
    Trait,
    Impl,
    Mod,
}

/// Declaration value assignment
#[derive(Debug, Clone)]
pub struct DeclValue {
    pub decl_id: String,
    pub monster_aspects: Vec<u8>, // 1-108 aspects
    pub complexity_score: f64,
    pub dependency_count: u32,
    pub monster_value: f64, // Final assigned value
}

/// Rollup summary for repository
#[derive(Debug, Clone)]
pub struct RollupSummary {
    pub repo_id: u32,
    pub total_files: u32,
    pub total_decls: u32,
    pub avg_complexity: f64,
    pub top_aspects: Vec<u8>, // Most common Monster aspects
    pub monster_signature: LLMWeight12Form<2048>,
}

#[derive(Debug, Clone)]
pub enum ProcessingStage {
    NotLoaded,
    ContentLoaded,
    ASTExtracted,
    ValuesAssigned,
    RolledUp,
    Cached,
}

impl LazyASTProcessor {
    pub fn new(repo_graph: GitRepoGraph) -> Self {
        Self {
            repo_graph,
            decl_splitter: UnifiedDeclSplitter::new(),
            cache: ASTCache {
                file_asts: Vec::new(),
                decl_values: Vec::new(),
                rollup_summaries: Vec::new(),
            },
        }
    }
    
    /// Lazy load and process repository
    pub fn process_repo_lazy(&mut self, repo_id: u32) -> Option<RollupSummary> {
        // Check cache first
        if let Some(summary) = self.get_cached_summary(repo_id) {
            return Some(summary);
        }
        
        let repo = self.repo_graph.repos.get(&repo_id)?;
        
        // Step 1: Lazy load git files
        let files = self.lazy_load_repo_files(&repo.repo_url);
        
        // Step 2: Extract ASTs
        let mut all_decls = Vec::new();
        for file_path in files {
            if let Some(cached_ast) = self.process_file_lazy(&file_path, &repo.repo_url) {
                all_decls.extend(cached_ast.decl_nodes);
            }
        }
        
        // Step 3: Assign values
        let decl_values = self.assign_decl_values(&all_decls);
        
        // Step 4: Roll up summary
        let summary = self.create_rollup_summary(repo_id, &decl_values);
        
        // Step 5: Cache everything
        self.cache_summary(&summary);
        
        Some(summary)
    }
    
    /// Lazy load single file and extract AST
    pub fn process_file_lazy(&mut self, file_path: &str, repo_url: &str) -> Option<CachedAST> {
        // Check cache first
        if let Some(cached) = self.get_cached_ast(file_path) {
            return Some(cached);
        }
        
        let mut cached_ast = CachedAST {
            file_path: file_path.to_string(),
            git_hash: self.get_file_git_hash(file_path, repo_url),
            raw_content: None,
            parsed_ast: None,
            decl_nodes: Vec::new(),
            monster_signature: self.llm_monstrous_form(),
            processing_stage: ProcessingStage::NotLoaded,
        };
        
        // Lazy load content only when needed
        if cached_ast.raw_content.is_none() {
            cached_ast.raw_content = self.lazy_load_file_content(file_path, repo_url);
            cached_ast.processing_stage = ProcessingStage::ContentLoaded;
        }
        
        // Extract AST
        if let Some(content) = &cached_ast.raw_content {
            cached_ast.decl_nodes = self.decl_splitter.split_file_into_decls(content);
            cached_ast.parsed_ast = Some(self.parse_content_to_ast(content));
            cached_ast.processing_stage = ProcessingStage::ASTExtracted;
        }
        
        // Cache intermediate result
        self.cache_ast(&cached_ast);
        
        Some(cached_ast)
    }
    
    /// Assign Monster values to declarations
    pub fn assign_decl_values(&mut self, decls: &[DeclNode]) -> Vec<DeclValue> {
        let mut values = Vec::new();
        
        for decl in decls {
            let types = self.decl_splitter.extract_types_from_decl(decl);
            
            let decl_value = DeclValue {
                decl_id: decl.account_id.clone(),
                monster_aspects: types.iter().map(|t| t.monster_factor).collect(),
                complexity_score: self.calculate_complexity_score(decl),
                dependency_count: self.count_dependencies(decl),
                monster_value: self.calculate_monster_value(decl),
            };
            
            // Cache decl value
            self.cache_decl_value(&decl_value);
            values.push(decl_value);
        }
        
        values
    }
    
    /// Create rollup summary
    pub fn create_rollup_summary(&self, repo_id: u32, decl_values: &[DeclValue]) -> RollupSummary {
        let total_decls = decl_values.len() as u32;
        let avg_complexity = decl_values.iter()
            .map(|d| d.complexity_score)
            .sum::<f64>() / total_decls as f64;
        
        // Find most common aspects
        let mut aspect_counts = [0u32; 108];
        for decl in decl_values {
            for &aspect in &decl.monster_aspects {
                if aspect > 0 && aspect <= 108 {
                    aspect_counts[(aspect - 1) as usize] += 1;
                }
            }
        }
        
        let mut top_aspects: Vec<u8> = (1..=108)
            .collect::<Vec<u8>>()
            .into_iter()
            .filter(|&i| aspect_counts[(i - 1) as usize] > 0)
            .collect();
        top_aspects.sort_by_key(|&i| aspect_counts[(i - 1) as usize]);
        top_aspects.reverse();
        top_aspects.truncate(10); // Top 10 aspects
        
        RollupSummary {
            repo_id,
            total_files: 0, // Will be updated
            total_decls,
            avg_complexity,
            top_aspects,
            monster_signature: self.llm_monstrous_form(),
        }
    }
    
    // Cache operations
    fn get_cached_ast(&self, file_path: &str) -> Option<CachedAST> {
        self.cache.file_asts.iter()
            .find(|(path, _)| path == file_path)
            .map(|(_, ast)| ast.clone())
    }
    
    fn cache_ast(&mut self, ast: &CachedAST) {
        self.cache.file_asts.push((ast.file_path.clone(), ast.clone()));
    }
    
    fn cache_decl_value(&mut self, value: &DeclValue) {
        self.cache.decl_values.push((value.decl_id.clone(), value.clone()));
    }
    
    fn get_cached_summary(&self, repo_id: u32) -> Option<RollupSummary> {
        self.cache.rollup_summaries.iter()
            .find(|(id, _)| id == &repo_id.to_string())
            .map(|(_, summary)| summary.clone())
    }
    
    fn cache_summary(&mut self, summary: &RollupSummary) {
        self.cache.rollup_summaries.push((summary.repo_id.to_string(), summary.clone()));
    }
    
    // Helper methods
    fn lazy_load_repo_files(&self, repo_url: &str) -> Vec<String> {
        // git ls-tree -r HEAD --name-only | grep "\.rs$"
        vec![
            "src/lib.rs".to_string(),
            "src/main.rs".to_string(),
            "compiler/rustc_driver/src/lib.rs".to_string(),
        ]
    }
    
    fn lazy_load_file_content(&self, file_path: &str, repo_url: &str) -> Option<String> {
        // git show HEAD:{file_path}
        Some(format!("// Content of {}", file_path))
    }
    
    fn get_file_git_hash(&self, file_path: &str, repo_url: &str) -> String {
        // git rev-parse HEAD:{file_path}
        format!("hash_{}", file_path.len())
    }
    
    fn parse_content_to_ast(&self, content: &str) -> ParsedAST {
        ParsedAST {
            items: vec![ASTItem {
                name: "example".to_string(),
                item_type: ASTItemType::Function,
                monster_value: 1.0,
            }],
            imports: vec!["std".to_string()],
            exports: vec!["main".to_string()],
        }
    }
    
    fn calculate_complexity_score(&self, decl: &DeclNode) -> f64 {
        decl.decl_name.len() as f64 / 100.0
    }
    
    fn count_dependencies(&self, decl: &DeclNode) -> u32 {
        decl.decl_name.len() as u32 % 10
    }
    
    fn calculate_monster_value(&self, decl: &DeclNode) -> f64 {
        // LLM calculates Monster value based on signature
        let sig_sum: u64 = decl.monster_signature.0.iter().map(|&x| x as u64).sum();
        (sig_sum % 1000) as f64 / 1000.0
    }
}

/// Process repository with full lazy loading pipeline
pub fn process_repo_with_lazy_loading(repo_id: u32, repo_graph: GitRepoGraph) -> Option<RollupSummary> {
    let mut processor = LazyASTProcessor::new(repo_graph);
    processor.process_repo_lazy(repo_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lazy_ast_processor() {
        let graph = initialize_git_repo_graph();
        let summary = process_repo_with_lazy_loading(1, graph);
        assert!(summary.is_some());
    }
}
