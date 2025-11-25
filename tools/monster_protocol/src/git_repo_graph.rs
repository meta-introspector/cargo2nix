#![no_std]
extern crate alloc;
use alloc::{string::String, vec::Vec, collections::BTreeMap, format};

use crate::llm_monstrous_traits::*;

/// Git Repository Graph Database - indexes all repos, submodules, branches
pub struct GitRepoGraph {
    pub repos: BTreeMap<u32, RepoNode>,
    pub relations: Vec<RepoRelation>,
    pub next_id: u32,
}

/// Repository node in the graph
#[derive(Debug, Clone)]
pub struct RepoNode {
    pub id: u32,
    pub repo_url: String,
    pub repo_type: RepoType,
    pub branches: Vec<String>,
    pub submodules: Vec<u32>, // IDs of submodule repos
    pub monster_signature: LLMWeight12Form<2048>,
    pub interest_level: InterestLevel,
}

/// Relation between repositories
#[derive(Debug, Clone)]
pub struct RepoRelation {
    pub from_id: u32,
    pub to_id: u32,
    pub relation_type: RelationType,
    pub weight: f64, // Distance weight for shortest path
}

#[derive(Debug, Clone)]
pub enum RepoType {
    MainRepo,
    Submodule,
    Fork,
    Dependency,
}

#[derive(Debug, Clone)]
pub enum RelationType {
    Submodule,
    Fork,
    Dependency,
    MonsterEquivalence,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InterestLevel {
    Rustc = 1,        // Highest priority
    MonsterGroup = 2, // Second priority  
    Cargo2Nix = 3,
    Tool = 4,
    Other = 5,
}

impl GitRepoGraph {
    pub fn new() -> Self {
        Self {
            repos: BTreeMap::new(),
            relations: Vec::new(),
            next_id: 1,
        }
    }
    
    /// Add repository to graph
    pub fn add_repo(&mut self, repo_url: String, repo_type: RepoType, interest: InterestLevel) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        
        let repo = RepoNode {
            id,
            repo_url: repo_url.clone(),
            repo_type,
            branches: self.discover_branches(&repo_url),
            submodules: Vec::new(),
            monster_signature: self.calculate_monster_signature(&repo_url),
            interest_level: interest,
        };
        
        self.repos.insert(id, repo);
        id
    }
    
    /// Add relation between repos
    pub fn add_relation(&mut self, from_id: u32, to_id: u32, relation_type: RelationType) {
        let weight = self.calculate_relation_weight(&relation_type);
        
        self.relations.push(RepoRelation {
            from_id,
            to_id,
            relation_type,
            weight,
        });
        
        // Update submodule references
        if let RelationType::Submodule = relation_type {
            if let Some(repo) = self.repos.get_mut(&from_id) {
                repo.submodules.push(to_id);
            }
        }
    }
    
    /// Initialize with repos of interest
    pub fn initialize_repos_of_interest(&mut self) {
        // 1. Rustc (highest priority)
        let rustc_id = self.add_repo(
            "https://github.com/rust-lang/rust".to_string(),
            RepoType::MainRepo,
            InterestLevel::Rustc
        );
        
        // 2. Monster Group (second priority)
        let monster_id = self.add_repo(
            "https://github.com/meta-introspector/monster-group-theory".to_string(),
            RepoType::MainRepo,
            InterestLevel::MonsterGroup
        );
        
        // 3. Cargo2nix
        let cargo2nix_id = self.add_repo(
            "https://github.com/cargo2nix/cargo2nix".to_string(),
            RepoType::MainRepo,
            InterestLevel::Cargo2Nix
        );
        
        // Add current submodules
        let minizinc_id = self.add_repo(
            "https://github.com/meta-introspector/libminizinc".to_string(),
            RepoType::Submodule,
            InterestLevel::Tool
        );
        
        // Create relations
        self.add_relation(cargo2nix_id, minizinc_id, RelationType::Submodule);
        self.add_relation(rustc_id, monster_id, RelationType::MonsterEquivalence);
    }
    
    /// Find shortest path between two repos
    pub fn find_shortest_path(&self, from_id: u32, to_id: u32) -> Option<Vec<u32>> {
        // Dijkstra's algorithm
        let mut distances: BTreeMap<u32, f64> = BTreeMap::new();
        let mut previous: BTreeMap<u32, u32> = BTreeMap::new();
        let mut unvisited: Vec<u32> = self.repos.keys().cloned().collect();
        
        // Initialize distances
        for &repo_id in self.repos.keys() {
            distances.insert(repo_id, if repo_id == from_id { 0.0 } else { f64::INFINITY });
        }
        
        while !unvisited.is_empty() {
            // Find unvisited node with minimum distance
            let current = *unvisited.iter()
                .min_by(|&&a, &&b| {
                    distances[&a].partial_cmp(&distances[&b]).unwrap()
                })
                .unwrap();
            
            unvisited.retain(|&x| x != current);
            
            if current == to_id {
                break;
            }
            
            // Update distances to neighbors
            for relation in &self.relations {
                if relation.from_id == current && unvisited.contains(&relation.to_id) {
                    let alt = distances[&current] + relation.weight;
                    if alt < distances[&relation.to_id] {
                        distances.insert(relation.to_id, alt);
                        previous.insert(relation.to_id, current);
                    }
                }
            }
        }
        
        // Reconstruct path
        if !previous.contains_key(&to_id) {
            return None;
        }
        
        let mut path = Vec::new();
        let mut current = to_id;
        
        while current != from_id {
            path.push(current);
            current = previous[&current];
        }
        path.push(from_id);
        path.reverse();
        
        Some(path)
    }
    
    /// Find shortest path from rustc to monster group
    pub fn find_rustc_to_monster_path(&self) -> Option<Vec<u32>> {
        let rustc_id = self.find_repo_by_interest(InterestLevel::Rustc)?;
        let monster_id = self.find_repo_by_interest(InterestLevel::MonsterGroup)?;
        
        self.find_shortest_path(rustc_id, monster_id)
    }
    
    fn find_repo_by_interest(&self, interest: InterestLevel) -> Option<u32> {
        self.repos.iter()
            .find(|(_, repo)| repo.interest_level == interest)
            .map(|(id, _)| *id)
    }
    
    fn discover_branches(&self, repo_url: &str) -> Vec<String> {
        // git ls-remote --heads {repo_url}
        vec!["main".to_string(), "master".to_string(), "develop".to_string()]
    }
    
    fn calculate_monster_signature(&self, repo_url: &str) -> LLMWeight12Form<2048> {
        // LLM calculates Monster signature for repo
        let mut coeffs = [0u16; 2048];
        let bytes = repo_url.as_bytes();
        for (i, &byte) in bytes.iter().enumerate() {
            if i >= 2048 { break; }
            coeffs[i] = (byte as u16 * 257) % 32768;
        }
        LLMWeight12Form(coeffs)
    }
    
    fn calculate_relation_weight(&self, relation_type: &RelationType) -> f64 {
        match relation_type {
            RelationType::MonsterEquivalence => 0.1, // Shortest distance
            RelationType::Submodule => 0.5,
            RelationType::Fork => 0.7,
            RelationType::Dependency => 1.0,
        }
    }
}

/// Initialize git repository graph database
pub fn initialize_git_repo_graph() -> GitRepoGraph {
    let mut graph = GitRepoGraph::new();
    graph.initialize_repos_of_interest();
    graph
}

/// Find shortest path between rustc and monster group
pub fn find_rustc_monster_shortest_path() -> Option<Vec<u32>> {
    let graph = initialize_git_repo_graph();
    graph.find_rustc_to_monster_path()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_git_repo_graph() {
        let mut graph = GitRepoGraph::new();
        
        let rustc_id = graph.add_repo(
            "https://github.com/rust-lang/rust".to_string(),
            RepoType::MainRepo,
            InterestLevel::Rustc
        );
        
        let monster_id = graph.add_repo(
            "https://github.com/monster-group".to_string(),
            RepoType::MainRepo,
            InterestLevel::MonsterGroup
        );
        
        graph.add_relation(rustc_id, monster_id, RelationType::MonsterEquivalence);
        
        let path = graph.find_shortest_path(rustc_id, monster_id);
        assert!(path.is_some());
        assert_eq!(path.unwrap(), vec![rustc_id, monster_id]);
    }
}
