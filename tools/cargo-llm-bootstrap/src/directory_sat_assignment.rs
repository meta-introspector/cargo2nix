/// SAT solver to assign prime factors to directories to sum to Monster Group
use crate::pure_rust_sat::{SATSolver, Variable, Literal, Clause};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Monster Group prime factors
const MONSTER_FACTORS: [(u64, u32); 15] = [
    (2, 46), (3, 20), (5, 9), (7, 6), (11, 2), (13, 3),
    (17, 1), (19, 1), (23, 1), (29, 1), (31, 1), (41, 1),
    (47, 1), (59, 1), (71, 1)
];

#[derive(Debug, Clone)]
pub struct DirectoryAssignment {
    pub path: String,
    pub assigned_factors: Vec<(u64, u32)>, // prime^exponent pairs
    pub total_contribution: HashMap<u64, u32>, // prime -> total exponent
}

pub struct DirectorySATAssigner {
    directories: Vec<String>,
    solver: SATSolver,
    // Variables: dir_i_prime_j_exp_k = directory i gets prime j with exponent k
    assignment_vars: HashMap<(usize, u64, u32), Variable>,
    next_var_id: u32,
}

impl DirectorySATAssigner {
    pub fn new() -> Self {
        Self {
            directories: Vec::new(),
            solver: SATSolver::new(),
            assignment_vars: HashMap::new(),
            next_var_id: 1,
        }
    }

    pub fn solve_directory_assignment(&mut self, rust_src_path: &str) -> Result<Vec<DirectoryAssignment>, String> {
        println!("🎯 SAT solving directory prime factor assignments");
        
        // Collect all directories
        self.collect_directories(rust_src_path)?;
        println!("Found {} directories", self.directories.len());
        
        // Create SAT variables
        self.create_sat_variables();
        
        // Add constraints
        self.add_monster_sum_constraints()?;
        self.add_assignment_constraints()?;
        
        // Solve
        if self.solver.solve() {
            println!("✅ SAT solution found!");
            self.extract_solution()
        } else {
            Err("❌ No SAT solution exists for Monster Group assignment".to_string())
        }
    }

    fn collect_directories(&mut self, rust_src_path: &str) -> Result<(), String> {
        let rust_path = Path::new(rust_src_path);
        
        // Get all directories recursively
        self.collect_dirs_recursive(rust_path, "")?;
        
        Ok(())
    }

    fn collect_dirs_recursive(&mut self, path: &Path, prefix: &str) -> Result<(), String> {
        let entries = fs::read_dir(path)
            .map_err(|e| format!("Failed to read directory {:?}: {}", path, e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
            if entry.file_type().map_err(|e| format!("Failed to get file type: {}", e))?.is_dir() {
                if let Some(name) = entry.file_name().to_str() {
                    let full_name = if prefix.is_empty() {
                        name.to_string()
                    } else {
                        format!("{}/{}", prefix, name)
                    };
                    
                    self.directories.push(full_name.clone());
                    
                    // Recurse only for important directories to limit size
                    if name == "compiler" || name == "library" || name.starts_with("rustc_") {
                        self.collect_dirs_recursive(&entry.path(), &full_name)?;
                    }
                }
            }
        }
        
        Ok(())
    }

    fn create_sat_variables(&mut self) {
        println!("Creating SAT variables for {} directories", self.directories.len());
        
        for (dir_idx, _) in self.directories.iter().enumerate() {
            for &(prime, max_exp) in &MONSTER_FACTORS {
                // Each directory can get 0 to max_exp of each prime
                for exp in 0..=max_exp {
                    let var = Variable(self.next_var_id);
                    self.assignment_vars.insert((dir_idx, prime, exp), var);
                    self.next_var_id += 1;
                }
            }
        }
        
        println!("Created {} SAT variables", self.next_var_id - 1);
    }

    fn add_monster_sum_constraints(&mut self) -> Result<(), String> {
        println!("Adding Monster Group sum constraints");
        
        for &(prime, target_exp) in &MONSTER_FACTORS {
            // Sum of all directory assignments for this prime must equal target
            self.add_prime_sum_constraint(prime, target_exp)?;
        }
        
        Ok(())
    }

    fn add_prime_sum_constraint(&mut self, prime: u64, target_exp: u32) -> Result<(), String> {
        // For each prime, exactly target_exp total exponent across all directories
        
        // Create auxiliary variables for counting
        let mut count_vars = Vec::new();
        
        for exp in 1..=target_exp {
            // Count how many directories get exactly exp of this prime
            let mut clause_literals = Vec::new();
            
            for dir_idx in 0..self.directories.len() {
                if let Some(&var) = self.assignment_vars.get(&(dir_idx, prime, exp)) {
                    clause_literals.push(Literal::pos(var));
                }
            }
            
            if !clause_literals.is_empty() {
                // At least one directory must contribute this exponent level
                // This is a simplified constraint - full implementation would need
                // exact cardinality constraints
                self.solver.add_clause(Clause::new(clause_literals));
            }
        }
        
        Ok(())
    }

    fn add_assignment_constraints(&mut self) -> Result<(), String> {
        println!("Adding directory assignment constraints");
        
        // Find key directories for single-exponent prime pinning
        let rustc_main_idx = self.directories.iter().position(|d| 
            d.contains("rustc") && (d.contains("main") || d == "compiler/rustc")
        );
        let rustc_driver_idx = self.directories.iter().position(|d| d.contains("rustc_driver"));
        let rustc_middle_idx = self.directories.iter().position(|d| d.contains("rustc_middle"));
        let rustc_codegen_idx = self.directories.iter().position(|d| d.contains("rustc_codegen"));
        let rustc_resolve_idx = self.directories.iter().position(|d| d.contains("rustc_resolve"));
        let rustc_trait_idx = self.directories.iter().position(|d| d.contains("rustc_trait"));
        let rustc_ast_idx = self.directories.iter().position(|d| d.contains("rustc_ast"));
        let rustc_hir_idx = self.directories.iter().position(|d| d.contains("rustc_hir"));
        let rustc_mir_idx = self.directories.iter().position(|d| d.contains("rustc_mir"));
        
        // External compiler/proof system directories
        let lean4_idx = self.directories.iter().position(|d| d.contains("lean") || d.contains("Lean"));
        let llvm_idx = self.directories.iter().position(|d| d.contains("llvm") || d.contains("LLVM"));
        let gcc_idx = self.directories.iter().position(|d| d.contains("gcc") || d.contains("GCC"));
        
        // Fixed single-exponent prime assignments
        let single_exp_assignments = [
            // Core rustc components
            (71, rustc_main_idx, "rustc_main"),
            (59, rustc_driver_idx, "rustc_driver"),
            (47, rustc_middle_idx, "rustc_middle"), 
            (41, rustc_codegen_idx, "rustc_codegen"),
            (31, rustc_resolve_idx, "rustc_resolve"),
            (29, rustc_trait_idx, "rustc_trait_selection"),
            (23, rustc_ast_idx, "rustc_ast"),
            (19, rustc_hir_idx, "rustc_hir"),
            (17, rustc_mir_idx, "rustc_mir"),
            
            // External systems - fixed prime assignments
            // These primes are reserved and will be assigned when those systems are present
        ];
        
        // Reserved primes for external systems (not in current Monster Group but conceptually fixed)
        let external_system_primes = [
            (73, lean4_idx, "Lean4"),    // Next prime after 71 for proof systems
            (79, llvm_idx, "LLVM"),      // LLVM backend
            (83, gcc_idx, "GCC"),        // GCC backend
        ];
        
        for dir_idx in 0..self.directories.len() {
            for &(prime, max_exp) in &MONSTER_FACTORS {
                // Pin single-exponent primes to specific directories
                if max_exp == 1 {
                    let assigned_dir = single_exp_assignments.iter()
                        .find(|(p, _, _)| *p == prime)
                        .and_then(|(_, dir_opt, name)| dir_opt.map(|idx| (idx, name)));
                    
                    if let Some((assigned_idx, dir_name)) = assigned_dir {
                        if dir_idx == assigned_idx {
                            // This directory MUST get this prime
                            if let Some(&var) = self.assignment_vars.get(&(dir_idx, prime, 1)) {
                                self.solver.add_clause(Clause::new(vec![Literal::pos(var)]));
                                println!("🎯 Pinned: {} gets exclusive factor {}", dir_name, prime);
                            }
                        } else {
                            // All other directories cannot get this prime
                            if let Some(&var) = self.assignment_vars.get(&(dir_idx, prime, 1)) {
                                self.solver.add_clause(Clause::new(vec![Literal::neg(var)]));
                            }
                        }
                    }
                }
                
                // Each directory gets at most one exponent level per prime
                for exp1 in 0..=max_exp {
                    for exp2 in (exp1 + 1)..=max_exp {
                        if let (Some(&var1), Some(&var2)) = (
                            self.assignment_vars.get(&(dir_idx, prime, exp1)),
                            self.assignment_vars.get(&(dir_idx, prime, exp2))
                        ) {
                            // Not both exp1 and exp2
                            self.solver.add_clause(Clause::new(vec![
                                Literal::neg(var1),
                                Literal::neg(var2)
                            ]));
                        }
                    }
                }
            }
        }
        
        // Log external system prime reservations
        println!("\n🔒 Reserved prime assignments for external systems:");
        for (prime, dir_opt, system_name) in external_system_primes {
            if dir_opt.is_some() {
                println!("  {} → {} (present)", prime, system_name);
            } else {
                println!("  {} → {} (reserved, not present)", prime, system_name);
            }
        }
        
        Ok(())
    }

    fn extract_solution(&self) -> Result<Vec<DirectoryAssignment>, String> {
        let assignment = self.solver.get_assignment();
        let mut result = Vec::new();
        
        for (dir_idx, dir_path) in self.directories.iter().enumerate() {
            let mut assigned_factors = Vec::new();
            let mut total_contribution = HashMap::new();
            
            for &(prime, max_exp) in &MONSTER_FACTORS {
                for exp in 1..=max_exp {
                    if let Some(&var) = self.assignment_vars.get(&(dir_idx, prime, exp)) {
                        if assignment.get(&var) == Some(&true) {
                            assigned_factors.push((prime, exp));
                            *total_contribution.entry(prime).or_insert(0) += exp;
                        }
                    }
                }
            }
            
            if !assigned_factors.is_empty() {
                result.push(DirectoryAssignment {
                    path: dir_path.clone(),
                    assigned_factors,
                    total_contribution,
                });
            }
        }
        
        Ok(result)
    }

    pub fn verify_solution(&self, assignments: &[DirectoryAssignment]) -> Result<bool, String> {
        println!("🔍 Verifying Monster Group sum and pinned prime constraints");
        
        let mut total_exponents = HashMap::new();
        let mut prime_assignments: HashMap<u64, Vec<String>> = HashMap::new();
        
        for assignment in assignments {
            for &(prime, exp) in &assignment.assigned_factors {
                *total_exponents.entry(prime).or_insert(0) += exp;
                prime_assignments.entry(prime).or_insert_with(Vec::new).push(assignment.path.clone());
            }
        }
        
        // Verify pinned single-exponent primes
        let expected_pinnings = [
            (71, "rustc_main"),
            (59, "rustc_driver"), 
            (47, "rustc_middle"),
            (41, "rustc_codegen"),
            (31, "rustc_resolve"),
            (29, "rustc_trait_selection"),
            (23, "rustc_ast"),
            (19, "rustc_hir"),
            (17, "rustc_mir"),
        ];
        
        println!("🎯 Verifying pinned single-exponent prime assignments:");
        let mut pinning_valid = true;
        
        for (prime, expected_component) in expected_pinnings {
            if let Some(assigned_dirs) = prime_assignments.get(&prime) {
                if assigned_dirs.len() == 1 {
                    let assigned_dir = &assigned_dirs[0];
                    if assigned_dir.contains(&expected_component.replace("_", "_")) {
                        println!("✅ Prime {} → {}", prime, assigned_dir);
                    } else {
                        println!("❌ Prime {} assigned to {} (expected {})", prime, assigned_dir, expected_component);
                        pinning_valid = false;
                    }
                } else {
                    println!("❌ Prime {} assigned to {} directories: {:?}", prime, assigned_dirs.len(), assigned_dirs);
                    pinning_valid = false;
                }
            } else {
                println!("❌ Prime {} not assigned to any directory", prime);
                pinning_valid = false;
            }
        }
        
        println!("\nMonster Group sum verification:");
        let mut sum_valid = true;
        
        for &(prime, target_exp) in &MONSTER_FACTORS {
            let actual_exp = total_exponents.get(&prime).copied().unwrap_or(0);
            let status = if actual_exp == target_exp { "✅" } else { "❌" };
            
            if actual_exp != target_exp {
                sum_valid = false;
            }
            
            println!("  {} : {} → {} {}", prime, actual_exp, target_exp, status);
        }
        
        let overall_valid = sum_valid && pinning_valid;
        if overall_valid {
            println!("✅ Perfect Monster Group assignment with all pinned primes!");
        } else {
            println!("❌ Assignment constraints not satisfied");
        }
        
        Ok(overall_valid)
    }

    pub fn print_solution(&self, assignments: &[DirectoryAssignment]) {
        println!("\n📋 Directory Prime Factor Assignments:");
        println!("=" .repeat(60));
        
        for assignment in assignments {
            print!("{:30} : ", assignment.path);
            
            if assignment.assigned_factors.is_empty() {
                println!("1");
            } else {
                let factor_strs: Vec<String> = assignment.assigned_factors.iter()
                    .map(|(p, e)| if *e == 1 { format!("{}", p) } else { format!("{}^{}", p, e) })
                    .collect();
                println!("{}", factor_strs.join(" × "));
            }
        }
    }
}
