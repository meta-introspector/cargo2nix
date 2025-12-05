/// Perfect Hashing Function for rustc Terms to Monster Group Factors
/// Maps all rustc terms directly to Monster Group prime factors
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

const MONSTER_PRIMES: [u64; 15] = [2,3,5,7,11,13,17,19,23,29,31,41,47,59,71];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerfectMonsterHash {
    pub term_to_factor: HashMap<String, (u64, u32)>, // term -> (prime, exponent)
    pub factor_usage: HashMap<u64, u32>, // prime -> total exponent used
    pub hash_parameters: HashParameters,
    pub collision_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashParameters {
    pub salt: u64,
    pub modulus: u64,
    pub multiplier: u64,
    pub offset: u64,
}

pub struct PerfectHashBuilder {
    pub terms: Vec<String>,
    pub optimized_matrix: HashMap<String, Vec<f64>>, // From recursive solver
    pub hash_function: PerfectMonsterHash,
}

impl PerfectHashBuilder {
    pub fn new(terms: Vec<String>) -> Self {
        Self {
            terms,
            optimized_matrix: HashMap::new(),
            hash_function: PerfectMonsterHash {
                term_to_factor: HashMap::new(),
                factor_usage: HashMap::new(),
                hash_parameters: HashParameters {
                    salt: 0x1337BEEF,
                    modulus: 108, // Total Monster factors
                    multiplier: 31,
                    offset: 17,
                },
                collision_count: 0,
            },
        }
    }

    /// Build perfect hash from optimized matrix F
    pub fn build_from_matrix(&mut self, matrix_f: &crate::recursive_matrix_solver::MatrixF) -> Result<(), String> {
        println!("🔨 Building perfect hash function from optimized matrix F");
        
        // Extract term weights from matrix
        for (decl_idx, decl_name) in matrix_f.declarations.iter().enumerate() {
            let weights = matrix_f.weights[decl_idx].clone();
            self.optimized_matrix.insert(decl_name.clone(), weights);
        }
        
        // Generate perfect hash for all terms
        self.generate_perfect_mapping()?;
        
        // Optimize hash parameters to minimize collisions
        self.optimize_hash_parameters()?;
        
        println!("✅ Perfect hash function generated");
        println!("  Terms mapped: {}", self.hash_function.term_to_factor.len());
        println!("  Collisions: {}", self.hash_function.collision_count);
        
        Ok(())
    }

    fn generate_perfect_mapping(&mut self) -> Result<(), String> {
        for term in &self.terms {
            let (prime, exponent) = self.compute_optimal_factor(term)?;
            
            // Check for collisions and resolve
            if let Some(existing) = self.hash_function.term_to_factor.get(term) {
                if existing != &(prime, exponent) {
                    self.hash_function.collision_count += 1;
                    // Resolve collision by adjusting hash parameters
                    self.resolve_collision(term, prime, exponent)?;
                }
            } else {
                self.hash_function.term_to_factor.insert(term.clone(), (prime, exponent));
                *self.hash_function.factor_usage.entry(prime).or_insert(0) += exponent;
            }
        }
        
        Ok(())
    }

    fn compute_optimal_factor(&self, term: &str) -> Result<(u64, u32), String> {
        // Use optimized matrix weights if available
        if let Some(weights) = self.optimized_matrix.get(term) {
            return self.weights_to_factor(weights);
        }
        
        // Fallback to hash-based assignment
        self.hash_to_factor(term)
    }

    fn weights_to_factor(&self, weights: &[f64]) -> Result<(u64, u32), String> {
        // Find prime with highest weight
        let mut max_weight = 0.0;
        let mut best_prime_idx = 0;
        
        for (idx, &weight) in weights.iter().enumerate() {
            if weight > max_weight {
                max_weight = weight;
                best_prime_idx = idx;
            }
        }
        
        let prime = MONSTER_PRIMES[best_prime_idx];
        let exponent = (max_weight.log2().max(0.0) as u32 + 1).min(10);
        
        Ok((prime, exponent))
    }

    fn hash_to_factor(&self, term: &str) -> Result<(u64, u32), String> {
        let hash = self.compute_term_hash(term);
        let prime_idx = (hash % MONSTER_PRIMES.len() as u64) as usize;
        let prime = MONSTER_PRIMES[prime_idx];
        
        // Compute exponent based on term characteristics
        let exponent = self.compute_exponent_for_term(term, hash);
        
        Ok((prime, exponent))
    }

    fn compute_term_hash(&self, term: &str) -> u64 {
        let params = &self.hash_function.hash_parameters;
        
        // Perfect hash function: h(x) = ((a*x + b) mod p) mod m
        let mut hasher = Sha256::new();
        hasher.update(term.as_bytes());
        hasher.update(params.salt.to_le_bytes());
        
        let hash_bytes = hasher.finalize();
        let hash_u64 = u64::from_le_bytes([
            hash_bytes[0], hash_bytes[1], hash_bytes[2], hash_bytes[3],
            hash_bytes[4], hash_bytes[5], hash_bytes[6], hash_bytes[7],
        ]);
        
        ((params.multiplier * hash_u64 + params.offset) % params.modulus) % 108
    }

    fn compute_exponent_for_term(&self, term: &str, hash: u64) -> u32 {
        // Semantic exponent assignment
        let base_exp = match term {
            t if t.contains("main") => 3,
            t if t.contains("fn") => 2,
            t if t.contains("struct") => 2,
            t if t.contains("enum") => 2,
            t if t.contains("trait") => 1,
            t if t.contains("impl") => 1,
            _ => 1,
        };
        
        // Add hash-based variation
        let hash_exp = (hash % 3) as u32;
        (base_exp + hash_exp).min(5)
    }

    fn resolve_collision(&mut self, term: &str, prime: u64, exponent: u32) -> Result<(), String> {
        // Adjust hash parameters to resolve collision
        self.hash_function.hash_parameters.salt = self.hash_function.hash_parameters.salt.wrapping_mul(31).wrapping_add(17);
        
        // Recompute hash with new parameters
        let new_hash = self.compute_term_hash(term);
        let new_prime_idx = (new_hash % MONSTER_PRIMES.len() as u64) as usize;
        let new_prime = MONSTER_PRIMES[new_prime_idx];
        
        self.hash_function.term_to_factor.insert(term.to_string(), (new_prime, exponent));
        *self.hash_function.factor_usage.entry(new_prime).or_insert(0) += exponent;
        
        Ok(())
    }

    fn optimize_hash_parameters(&mut self) -> Result<(), String> {
        let mut best_collision_count = self.hash_function.collision_count;
        let mut best_params = self.hash_function.hash_parameters.clone();
        
        // Try different parameter combinations
        for multiplier in [31, 37, 41, 43, 47] {
            for offset in [17, 19, 23, 29] {
                let mut test_params = HashParameters {
                    salt: 0x1337BEEF,
                    modulus: 108,
                    multiplier,
                    offset,
                };
                
                let collision_count = self.test_parameters(&test_params)?;
                if collision_count < best_collision_count {
                    best_collision_count = collision_count;
                    best_params = test_params;
                }
            }
        }
        
        self.hash_function.hash_parameters = best_params;
        self.hash_function.collision_count = best_collision_count;
        
        Ok(())
    }

    fn test_parameters(&self, params: &HashParameters) -> Result<usize, String> {
        let mut test_mapping = HashMap::new();
        let mut collisions = 0;
        
        for term in &self.terms {
            let hash = self.compute_term_hash_with_params(term, params);
            let prime_idx = (hash % MONSTER_PRIMES.len() as u64) as usize;
            let prime = MONSTER_PRIMES[prime_idx];
            
            if test_mapping.contains_key(&hash) {
                collisions += 1;
            } else {
                test_mapping.insert(hash, (term.clone(), prime));
            }
        }
        
        Ok(collisions)
    }

    fn compute_term_hash_with_params(&self, term: &str, params: &HashParameters) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(term.as_bytes());
        hasher.update(params.salt.to_le_bytes());
        
        let hash_bytes = hasher.finalize();
        let hash_u64 = u64::from_le_bytes([
            hash_bytes[0], hash_bytes[1], hash_bytes[2], hash_bytes[3],
            hash_bytes[4], hash_bytes[5], hash_bytes[6], hash_bytes[7],
        ]);
        
        ((params.multiplier * hash_u64 + params.offset) % params.modulus) % 108
    }

    /// Validate perfect hash function
    pub fn validate_perfect_hash(&self) -> Result<bool, String> {
        println!("🔍 Validating perfect hash function");
        
        // Check Monster Group constraints
        let mut total_factors = 0;
        let mut constraint_violations = 0;
        
        for (&prime, &used_exp) in &self.hash_function.factor_usage {
            let max_exp = match prime {
                2 => 46, 3 => 20, 5 => 9, 7 => 6, 11 => 2, 13 => 3,
                _ => 1, // Single exponent primes
            };
            
            total_factors += used_exp;
            
            if used_exp > max_exp {
                constraint_violations += 1;
                println!("❌ Prime {} exceeds limit: {} > {}", prime, used_exp, max_exp);
            } else {
                println!("✅ Prime {}: {} / {} used", prime, used_exp, max_exp);
            }
        }
        
        println!("Total factors used: {} / 108", total_factors);
        println!("Constraint violations: {}", constraint_violations);
        
        let valid = constraint_violations == 0 && total_factors <= 108;
        
        if valid {
            println!("✅ Perfect hash function is valid!");
        } else {
            println!("❌ Hash function violates Monster Group constraints");
        }
        
        Ok(valid)
    }

    /// Generate lookup function for direct term->factor mapping
    pub fn generate_lookup_function(&self) -> String {
        let mut code = String::new();
        code.push_str("/// Generated perfect hash function for rustc terms\n");
        code.push_str("pub fn rustc_term_to_monster_factor(term: &str) -> Option<(u64, u32)> {\n");
        code.push_str("    match term {\n");
        
        for (term, (prime, exp)) in &self.hash_function.term_to_factor {
            code.push_str(&format!("        \"{}\" => Some(({}, {})),\n", term, prime, exp));
        }
        
        code.push_str("        _ => None,\n");
        code.push_str("    }\n");
        code.push_str("}\n");
        
        code
    }

    pub fn export_hash_function(&self, output_path: &str) -> Result<(), String> {
        let json = serde_json::to_string_pretty(&self.hash_function)
            .map_err(|e| format!("Serialization failed: {}", e))?;
        
        std::fs::write(output_path, json)
            .map_err(|e| format!("Write failed: {}", e))?;
        
        // Also export the lookup function
        let lookup_code = self.generate_lookup_function();
        let code_path = output_path.replace(".json", "_lookup.rs");
        std::fs::write(code_path, lookup_code)
            .map_err(|e| format!("Code write failed: {}", e))?;
        
        println!("📄 Perfect hash function exported to: {}", output_path);
        Ok(())
    }
}
