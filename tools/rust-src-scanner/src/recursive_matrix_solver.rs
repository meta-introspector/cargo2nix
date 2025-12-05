/// Recursive Matrix Solver for Monster Group Mapping
/// Adjusts matrix F weights to find optimal mapping in recursive loop
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

const MONSTER_TARGET: [(u64, u32); 15] = [
    (2, 46), (3, 20), (5, 9), (7, 6), (11, 2), (13, 3),
    (17, 1), (19, 1), (23, 1), (29, 1), (31, 1), (41, 1),
    (47, 1), (59, 1), (71, 1)
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixF {
    pub weights: Vec<Vec<f64>>, // N x 15 matrix (declarations x Monster primes)
    pub declarations: Vec<String>,
    pub current_mapping: HashMap<u64, u32>, // prime -> current exponent
    pub target_mapping: HashMap<u64, u32>,  // prime -> target exponent
    pub convergence_error: f64,
}

#[derive(Debug, Clone)]
pub struct RecursiveMatrixSolver {
    pub matrix_f: MatrixF,
    pub learning_rate: f64,
    pub max_iterations: usize,
    pub convergence_threshold: f64,
}

impl RecursiveMatrixSolver {
    pub fn new(declarations: Vec<String>) -> Self {
        let n = declarations.len();
        let weights = vec![vec![0.1; 15]; n]; // Initialize with small weights
        
        let mut target_mapping = HashMap::new();
        for (prime, exp) in MONSTER_TARGET {
            target_mapping.insert(prime, exp);
        }
        
        Self {
            matrix_f: MatrixF {
                weights,
                declarations,
                current_mapping: HashMap::new(),
                target_mapping,
                convergence_error: f64::INFINITY,
            },
            learning_rate: 0.01,
            max_iterations: 1000,
            convergence_threshold: 0.001,
        }
    }
    
    /// Main recursive solver loop
    pub fn solve_recursive_mapping(&mut self) -> Result<bool, String> {
        println!("🔄 Starting recursive matrix solver for Monster Group mapping");
        println!("Target: Find F weights such that F * declarations = Monster Group");
        
        for iteration in 0..self.max_iterations {
            // Forward pass: compute current mapping
            self.compute_current_mapping();
            
            // Calculate error
            let error = self.calculate_mapping_error();
            self.matrix_f.convergence_error = error;
            
            if iteration % 100 == 0 {
                println!("Iteration {}: Error = {:.6}", iteration, error);
            }
            
            // Check convergence
            if error < self.convergence_threshold {
                println!("✅ Converged after {} iterations", iteration);
                return Ok(true);
            }
            
            // Backward pass: adjust weights
            self.adjust_weights_recursive();
        }
        
        println!("❌ Failed to converge after {} iterations", self.max_iterations);
        Ok(false)
    }
    
    fn compute_current_mapping(&mut self) {
        self.matrix_f.current_mapping.clear();
        
        // Matrix multiplication: F * declarations -> prime assignments
        for (prime_idx, &(prime, _)) in MONSTER_TARGET.iter().enumerate() {
            let mut total_contribution = 0.0;
            
            for decl_idx in 0..self.matrix_f.declarations.len() {
                total_contribution += self.matrix_f.weights[decl_idx][prime_idx];
            }
            
            // Convert contribution to exponent (simplified)
            let exponent = (total_contribution.log2().max(0.0) as u32).min(50);
            self.matrix_f.current_mapping.insert(prime, exponent);
        }
    }
    
    fn calculate_mapping_error(&self) -> f64 {
        let mut total_error = 0.0;
        
        for &(prime, target_exp) in &MONSTER_TARGET {
            let current_exp = self.matrix_f.current_mapping.get(&prime).copied().unwrap_or(0);
            let error = (target_exp as f64 - current_exp as f64).powi(2);
            total_error += error;
        }
        
        total_error.sqrt()
    }
    
    fn adjust_weights_recursive(&mut self) {
        // Gradient descent with recursive feedback
        for (prime_idx, &(prime, target_exp)) in MONSTER_TARGET.iter().enumerate() {
            let current_exp = self.matrix_f.current_mapping.get(&prime).copied().unwrap_or(0);
            let error = target_exp as f64 - current_exp as f64;
            
            // Adjust weights for this prime
            for decl_idx in 0..self.matrix_f.declarations.len() {
                let gradient = self.compute_gradient(decl_idx, prime_idx, error);
                
                // Recursive adjustment: current weight influences next adjustment
                let current_weight = self.matrix_f.weights[decl_idx][prime_idx];
                let recursive_factor = 1.0 + (current_weight * 0.1); // Recursive feedback
                
                let adjustment = self.learning_rate * gradient * recursive_factor;
                self.matrix_f.weights[decl_idx][prime_idx] += adjustment;
                
                // Keep weights positive and bounded
                self.matrix_f.weights[decl_idx][prime_idx] = 
                    self.matrix_f.weights[decl_idx][prime_idx].max(0.0).min(10.0);
            }
        }
    }
    
    fn compute_gradient(&self, decl_idx: usize, prime_idx: usize, error: f64) -> f64 {
        // Simplified gradient computation
        // In practice, this would be the partial derivative of the error function
        let declaration_influence = self.get_declaration_influence(decl_idx, prime_idx);
        error * declaration_influence
    }
    
    fn get_declaration_influence(&self, decl_idx: usize, prime_idx: usize) -> f64 {
        // Calculate how much this declaration should influence this prime
        let decl_name = &self.matrix_f.declarations[decl_idx];
        let (prime, _) = MONSTER_TARGET[prime_idx];
        
        // Semantic influence based on declaration type and prime
        match (self.infer_decl_type(decl_name), prime) {
            (_, 71) if decl_name.contains("main") => 1.0,
            (_, 59) if decl_name.contains("fn") => 0.8,
            (_, 47) if decl_name.contains("struct") => 0.7,
            (_, 41) if decl_name.contains("enum") => 0.6,
            (_, 31) if decl_name.contains("trait") => 0.5,
            (_, 2) => 0.3, // Default influence on prime 2
            _ => 0.1,
        }
    }
    
    fn infer_decl_type(&self, decl_name: &str) -> &str {
        if decl_name.contains("fn") { "function" }
        else if decl_name.contains("struct") { "struct" }
        else if decl_name.contains("enum") { "enum" }
        else if decl_name.contains("trait") { "trait" }
        else { "unknown" }
    }
    
    /// Validate final mapping
    pub fn validate_mapping(&self) -> (bool, Vec<String>) {
        let mut issues = Vec::new();
        let mut valid = true;
        
        println!("\n🔍 Validating final Monster Group mapping:");
        
        for &(prime, target_exp) in &MONSTER_TARGET {
            let current_exp = self.matrix_f.current_mapping.get(&prime).copied().unwrap_or(0);
            let diff = (target_exp as i32 - current_exp as i32).abs();
            
            let status = if diff == 0 { "✅" } else if diff <= 2 { "⚠️" } else { "❌" };
            println!("  Prime {}: {} → {} (diff: {}) {}", 
                    prime, current_exp, target_exp, diff, status);
            
            if diff > 2 {
                issues.push(format!("Prime {} off by {}", prime, diff));
                valid = false;
            }
        }
        
        (valid, issues)
    }
    
    /// Export optimized matrix
    pub fn export_matrix(&self, output_path: &str) -> Result<(), String> {
        let json = serde_json::to_string_pretty(&self.matrix_f)
            .map_err(|e| format!("Serialization failed: {}", e))?;
        
        std::fs::write(output_path, json)
            .map_err(|e| format!("Write failed: {}", e))?;
        
        println!("📄 Optimized matrix F exported to: {}", output_path);
        Ok(())
    }
    
    pub fn print_solution_summary(&self) {
        println!("\n📊 Recursive Matrix Solver Summary:");
        println!("Declarations: {}", self.matrix_f.declarations.len());
        println!("Monster primes: {}", MONSTER_TARGET.len());
        println!("Final error: {:.6}", self.matrix_f.convergence_error);
        println!("Learning rate: {}", self.learning_rate);
        
        // Show weight distribution
        println!("\nWeight distribution by prime:");
        for (prime_idx, &(prime, _)) in MONSTER_TARGET.iter().enumerate() {
            let total_weight: f64 = self.matrix_f.weights.iter()
                .map(|row| row[prime_idx])
                .sum();
            let avg_weight = total_weight / self.matrix_f.declarations.len() as f64;
            println!("  Prime {}: avg weight = {:.3}", prime, avg_weight);
        }
    }
}

/// Solver factory for different optimization strategies
pub struct MatrixSolverFactory;

impl MatrixSolverFactory {
    pub fn create_gradient_descent_solver(declarations: Vec<String>) -> RecursiveMatrixSolver {
        let mut solver = RecursiveMatrixSolver::new(declarations);
        solver.learning_rate = 0.01;
        solver.max_iterations = 1000;
        solver
    }
    
    pub fn create_adaptive_solver(declarations: Vec<String>) -> RecursiveMatrixSolver {
        let mut solver = RecursiveMatrixSolver::new(declarations);
        solver.learning_rate = 0.05; // Higher learning rate
        solver.max_iterations = 2000;
        solver.convergence_threshold = 0.0001; // Tighter convergence
        solver
    }
    
    pub fn create_fast_solver(declarations: Vec<String>) -> RecursiveMatrixSolver {
        let mut solver = RecursiveMatrixSolver::new(declarations);
        solver.learning_rate = 0.1; // Very high learning rate
        solver.max_iterations = 500;
        solver.convergence_threshold = 0.01; // Looser convergence
        solver
    }
}
