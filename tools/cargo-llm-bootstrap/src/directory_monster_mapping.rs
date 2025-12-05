/// Directory Structure Monster Group Mapping
/// dir(rustc) = Monster Group through unitary balance
use std::collections::HashMap;
use std::path::Path;
use serde::{Deserialize, Serialize};

/// Monster Group prime factors for directory assignment
const MONSTER_FACTORS: [(u64, u32); 15] = [
    (2, 46), (3, 20), (5, 9), (7, 6), (11, 2), (13, 3),
    (17, 1), (19, 1), (23, 1), (29, 1), (31, 1), (41, 1),
    (47, 1), (59, 1), (71, 1)
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryFactor {
    pub path: String,
    pub assigned_prime: u64,
    pub assigned_exponent: u32,
    pub contribution: u64, // prime^exponent
    pub depth: usize,
    pub subdirs: Vec<String>,
}

#[derive(Debug)]
pub struct DirectoryMonsterMapper {
    pub directory_factors: Vec<DirectoryFactor>,
    pub factor_pool: HashMap<u64, u32>, // Available factors
    pub total_contribution: u64,
    pub target_order: u64, // Monster Group order
}

impl DirectoryMonsterMapper {
    pub fn new() -> Self {
        let mut factor_pool = HashMap::new();
        for (prime, exp) in MONSTER_FACTORS {
            factor_pool.insert(prime, exp);
        }
        
        // Monster Group order
        let target_order = MONSTER_FACTORS.iter()
            .map(|(p, e)| p.pow(*e))
            .product();
        
        Self {
            directory_factors: Vec::new(),
            factor_pool,
            total_contribution: 1,
            target_order,
        }
    }
    
    /// Scan rustc directory structure and assign Monster factors
    pub fn map_rustc_directories(&mut self, rust_src_path: &str) -> Result<(), String> {
        println!("🗂️  Mapping rustc directory structure to Monster Group factors");
        println!("Target: dir(rustc) = Monster Group order = {}", self.target_order);
        
        let rust_path = Path::new(rust_src_path);
        if !rust_path.exists() {
            return Err(format!("Rust source path does not exist: {}", rust_src_path));
        }
        
        // Get all directories in rustc
        let directories = self.collect_rustc_directories(rust_path)?;
        println!("Found {} directories in rustc", directories.len());
        
        // Assign factors to achieve unitary balance
        self.assign_factors_with_balance(&directories)?;
        
        // Verify unitary balance
        self.verify_unitary_balance()?;
        
        Ok(())
    }
    
    fn collect_rustc_directories(&self, rust_path: &Path) -> Result<Vec<(String, usize)>, String> {
        let mut directories = Vec::new();
        
        // Key rustc directories with their relative importance (depth weight)
        let key_dirs = [
            ("compiler", 1),
            ("library", 1), 
            ("src/tools", 2),
            ("tests", 2),
            ("compiler/rustc_driver", 2),
            ("compiler/rustc_middle", 2),
            ("compiler/rustc_codegen_llvm", 2),
            ("compiler/rustc_borrowck", 2),
            ("compiler/rustc_resolve", 2),
            ("compiler/rustc_trait_selection", 2),
            ("compiler/rustc_ast", 3),
            ("compiler/rustc_hir", 3),
            ("compiler/rustc_mir_build", 3),
            ("compiler/rustc_metadata", 3),
            ("library/std", 3),
        ];
        
        for (dir_path, depth) in key_dirs {
            let full_path = rust_path.join(dir_path);
            if full_path.exists() {
                directories.push((dir_path.to_string(), depth));
            }
        }
        
        Ok(directories)
    }
    
    fn assign_factors_with_balance(&mut self, directories: &[(String, usize)]) -> Result<(), String> {
        println!("🎯 Assigning Monster factors to achieve unitary balance");
        
        // Sort directories by importance (depth)
        let mut sorted_dirs = directories.clone();
        sorted_dirs.sort_by_key(|(_, depth)| *depth);
        
        // Assign largest factors to most important directories
        let mut factor_iter = MONSTER_FACTORS.iter().rev(); // Start with largest
        
        for (dir_path, depth) in sorted_dirs {
            if let Some(&(prime, exponent)) = factor_iter.next() {
                let contribution = prime.pow(exponent);
                
                let dir_factor = DirectoryFactor {
                    path: dir_path.clone(),
                    assigned_prime: prime,
                    assigned_exponent: exponent,
                    contribution,
                    depth,
                    subdirs: Vec::new(), // TODO: collect subdirs
                };
                
                println!("  {} → {}^{} = {}", dir_path, prime, exponent, contribution);
                self.directory_factors.push(dir_factor);
                self.total_contribution *= contribution;
            } else {
                // Assign remaining directories to factor 1 (identity)
                let dir_factor = DirectoryFactor {
                    path: dir_path.clone(),
                    assigned_prime: 1,
                    assigned_exponent: 1,
                    contribution: 1,
                    depth,
                    subdirs: Vec::new(),
                };
                self.directory_factors.push(dir_factor);
            }
        }
        
        Ok(())
    }
    
    fn verify_unitary_balance(&self) -> Result<(), String> {
        println!("⚖️  Verifying unitary balance: dir(rustc) = Monster Group");
        
        let calculated_order: u64 = self.directory_factors.iter()
            .map(|df| df.contribution)
            .product();
        
        println!("Calculated directory product: {}", calculated_order);
        println!("Monster Group target order:   {}", self.target_order);
        
        if calculated_order == self.target_order {
            println!("✅ UNITARY BALANCE ACHIEVED: dir(rustc) ≡ M");
            Ok(())
        } else {
            // Calculate adjustment needed
            let ratio = self.target_order as f64 / calculated_order as f64;
            println!("❌ Balance not achieved. Adjustment ratio: {:.2}", ratio);
            
            // Try to redistribute factors
            self.suggest_rebalancing(ratio)?;
            Err("Unitary balance not achieved".to_string())
        }
    }
    
    fn suggest_rebalancing(&self, ratio: f64) -> Result<(), String> {
        println!("🔄 Suggesting factor rebalancing:");
        
        if ratio > 1.0 {
            println!("  Need to increase total by factor of {:.2}", ratio);
            println!("  Suggestion: Assign higher exponents to core directories");
        } else {
            println!("  Need to decrease total by factor of {:.2}", 1.0/ratio);
            println!("  Suggestion: Use fractional assignments or identity factors");
        }
        
        // Show current assignments
        println!("Current assignments:");
        for df in &self.directory_factors {
            println!("  {} → {}^{} = {}", df.path, df.assigned_prime, df.assigned_exponent, df.contribution);
        }
        
        Ok(())
    }
    
    /// Generate directory-to-factor mapping for SAT solver
    pub fn generate_sat_constraints(&self) -> Vec<(String, u64, u32)> {
        self.directory_factors.iter()
            .map(|df| (df.path.clone(), df.assigned_prime, df.assigned_exponent))
            .collect()
    }
    
    /// Export directory mapping as JSON
    pub fn export_mapping(&self, output_path: &str) -> Result<(), String> {
        let json = serde_json::to_string_pretty(&self.directory_factors)
            .map_err(|e| format!("JSON serialization failed: {}", e))?;
        
        std::fs::write(output_path, json)
            .map_err(|e| format!("Failed to write mapping: {}", e))?;
        
        println!("📄 Directory mapping exported to: {}", output_path);
        Ok(())
    }
    
    /// Verify directory structure integrity
    pub fn verify_directory_integrity(&self, rust_src_path: &str) -> Result<bool, String> {
        println!("🔍 Verifying directory structure integrity");
        
        let rust_path = Path::new(rust_src_path);
        let mut all_exist = true;
        
        for df in &self.directory_factors {
            let dir_path = rust_path.join(&df.path);
            if !dir_path.exists() {
                println!("❌ Missing directory: {}", df.path);
                all_exist = false;
            } else {
                println!("✅ Found: {} → {}^{}", df.path, df.assigned_prime, df.assigned_exponent);
            }
        }
        
        Ok(all_exist)
    }
}
