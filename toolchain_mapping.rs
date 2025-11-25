// Toolchain Category Mapping: rustc Components → Prime Factor Sets
// High-level architectural decomposition via Monster Group partitioning

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct ToolchainCategory {
    pub name: String,
    pub prime_factors: HashSet<u64>,
    pub multiplicities: HashMap<u64, u32>,
    pub total_constraints: u32,
}

pub struct ToolchainMappingProtocol {
    categories: HashMap<String, ToolchainCategory>,
    prime_partition: HashMap<u64, String>,
}

impl ToolchainMappingProtocol {
    pub fn new() -> Self {
        let mut categories = HashMap::new();
        let mut prime_partition = HashMap::new();
        
        // Frontend: Lexical & Syntactic Analysis
        let frontend_primes = vec![(2, 46), (3, 20)];
        let mut frontend_factors = HashSet::new();
        let mut frontend_mults = HashMap::new();
        let mut frontend_total = 0;
        
        for (prime, mult) in frontend_primes {
            frontend_factors.insert(prime);
            frontend_mults.insert(prime, mult);
            frontend_total += mult;
            prime_partition.insert(prime, "Frontend".to_string());
        }
        
        categories.insert("Frontend".to_string(), ToolchainCategory {
            name: "Frontend".to_string(),
            prime_factors: frontend_factors,
            multiplicities: frontend_mults,
            total_constraints: frontend_total,
        });
        
        // Middleend: Type System & Analysis
        let middleend_primes = vec![(5, 9), (7, 6), (11, 2), (13, 3)];
        let mut middleend_factors = HashSet::new();
        let mut middleend_mults = HashMap::new();
        let mut middleend_total = 0;
        
        for (prime, mult) in middleend_primes {
            middleend_factors.insert(prime);
            middleend_mults.insert(prime, mult);
            middleend_total += mult;
            prime_partition.insert(prime, "Middleend".to_string());
        }
        
        categories.insert("Middleend".to_string(), ToolchainCategory {
            name: "Middleend".to_string(),
            prime_factors: middleend_factors,
            multiplicities: middleend_mults,
            total_constraints: middleend_total,
        });
        
        // Backend: Code Generation & Optimization
        let backend_primes = vec![(17, 1), (19, 1), (23, 1), (29, 1), (31, 1)];
        let mut backend_factors = HashSet::new();
        let mut backend_mults = HashMap::new();
        let mut backend_total = 0;
        
        for (prime, mult) in backend_primes {
            backend_factors.insert(prime);
            backend_mults.insert(prime, mult);
            backend_total += mult;
            prime_partition.insert(prime, "Backend".to_string());
        }
        
        categories.insert("Backend".to_string(), ToolchainCategory {
            name: "Backend".to_string(),
            prime_factors: backend_factors,
            multiplicities: backend_mults,
            total_constraints: backend_total,
        });
        
        // Runtime: Memory Management & Execution
        let runtime_primes = vec![(41, 1), (47, 1), (59, 1), (71, 1)];
        let mut runtime_factors = HashSet::new();
        let mut runtime_mults = HashMap::new();
        let mut runtime_total = 0;
        
        for (prime, mult) in runtime_primes {
            runtime_factors.insert(prime);
            runtime_mults.insert(prime, mult);
            runtime_total += mult;
            prime_partition.insert(prime, "Runtime".to_string());
        }
        
        categories.insert("Runtime".to_string(), ToolchainCategory {
            name: "Runtime".to_string(),
            prime_factors: runtime_factors,
            multiplicities: runtime_mults,
            total_constraints: runtime_total,
        });
        
        Self { categories, prime_partition }
    }
    
    pub fn get_category_for_prime(&self, prime: u64) -> Option<&String> {
        self.prime_partition.get(&prime)
    }
    
    pub fn get_category(&self, name: &str) -> Option<&ToolchainCategory> {
        self.categories.get(name)
    }
    
    pub fn validate_partition(&self) -> bool {
        let total_constraints: u32 = self.categories.values()
            .map(|cat| cat.total_constraints)
            .sum();
        
        let unique_primes: HashSet<u64> = self.categories.values()
            .flat_map(|cat| &cat.prime_factors)
            .cloned()
            .collect();
        
        total_constraints == 108 && unique_primes.len() == 15
    }
    
    pub fn generate_mapping_report(&self) -> String {
        let mut report = String::new();
        report.push_str("🏗️  TOOLCHAIN CATEGORY MAPPING\n");
        report.push_str("📐 rustc Components → Distinct Prime Factor Sets\n\n");
        
        let categories = ["Frontend", "Middleend", "Backend", "Runtime"];
        
        for category_name in &categories {
            if let Some(category) = self.categories.get(*category_name) {
                report.push_str(&format!("🔧 {}:\n", category.name));
                report.push_str(&format!("   Prime factors: {:?}\n", 
                    category.prime_factors.iter().collect::<Vec<_>>()));
                report.push_str(&format!("   Total constraints: {}\n", category.total_constraints));
                
                for (&prime, &mult) in &category.multiplicities {
                    report.push_str(&format!("   {}^{} ", prime, mult));
                }
                report.push_str("\n\n");
            }
        }
        
        report.push_str(&format!("✅ Partition validation: {}\n", self.validate_partition()));
        report.push_str(&format!("📊 Total categories: {}\n", self.categories.len()));
        report.push_str(&format!("🔢 Total prime factors: {}\n", self.prime_partition.len()));
        
        report
    }
    
    pub fn compute_category_interactions(&self) -> HashMap<(String, String), f64> {
        let mut interactions = HashMap::new();
        let category_names: Vec<String> = self.categories.keys().cloned().collect();
        
        for i in 0..category_names.len() {
            for j in i+1..category_names.len() {
                let cat1 = &category_names[i];
                let cat2 = &category_names[j];
                
                if let (Some(c1), Some(c2)) = (self.categories.get(cat1), self.categories.get(cat2)) {
                    let interaction_strength = self.calculate_interaction(c1, c2);
                    interactions.insert((cat1.clone(), cat2.clone()), interaction_strength);
                }
            }
        }
        
        interactions
    }
    
    fn calculate_interaction(&self, cat1: &ToolchainCategory, cat2: &ToolchainCategory) -> f64 {
        let min_prime1 = cat1.prime_factors.iter().min().unwrap_or(&1);
        let min_prime2 = cat2.prime_factors.iter().min().unwrap_or(&1);
        
        (*min_prime1 as f64).ln() * (*min_prime2 as f64).ln() / 
        (cat1.total_constraints as f64 + cat2.total_constraints as f64)
    }
}

fn main() {
    let protocol = ToolchainMappingProtocol::new();
    println!("{}", protocol.generate_mapping_report());
    
    // Category interactions
    let interactions = protocol.compute_category_interactions();
    println!("🔗 CATEGORY INTERACTIONS:");
    for ((cat1, cat2), strength) in interactions {
        println!("   {} ↔ {}: {:.3}", cat1, cat2, strength);
    }
    
    // Prime lookup examples
    println!("\n🔍 PRIME CATEGORY LOOKUP:");
    for prime in [2, 3, 5, 17, 41] {
        if let Some(category) = protocol.get_category_for_prime(prime) {
            println!("   Prime {} → {}", prime, category);
        }
    }
}
