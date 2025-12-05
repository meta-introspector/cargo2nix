/// rustc Crate Monster Group Assignment: Verify rustc ≡ M in terms of exponents
use crate::semantic_constraints::GödelNumber;
use crate::monster_compiler::{SupersingularReason, RustcComponentCategory};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Monster Group order: |M| = 2^46 × 3^20 × 5^9 × 7^6 × 11^2 × 13^3 × 17 × 19 × 23 × 29 × 31 × 41 × 47 × 59 × 71
const MONSTER_PRIME_EXPONENTS: [(u64, u32); 15] = [
    (2, 46), (3, 20), (5, 9), (7, 6), (11, 2), (13, 3), (17, 1), (19, 1),
    (23, 1), (29, 1), (31, 1), (41, 1), (47, 1), (59, 1), (71, 1)
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustcCrateAssignment {
    pub crate_name: String,
    pub crate_path: String,
    pub assigned_prime: u64,
    pub assigned_exponent: u32,
    pub monster_reason_id: u32,
    pub semantic_category: RustcComponentCategory,
    pub godel_signature: GödelNumber,
}

#[derive(Debug)]
pub struct MonsterGroupVerifier {
    pub crate_assignments: Vec<RustcCrateAssignment>,
    pub prime_usage: HashMap<u64, u32>, // prime -> total exponent used
    pub monster_target: HashMap<u64, u32>, // Monster Group target exponents
}

impl MonsterGroupVerifier {
    pub fn new() -> Self {
        let mut monster_target = HashMap::new();
        for &(prime, exponent) in &MONSTER_PRIME_EXPONENTS {
            monster_target.insert(prime, exponent);
        }

        Self {
            crate_assignments: Vec::new(),
            prime_usage: HashMap::new(),
            monster_target,
        }
    }

    /// Read rustc crates and assign Monster Group prime factorization
    pub fn assign_rustc_crates(&mut self, rust_src_path: &str) -> Result<(), String> {
        println!("🔍 Reading rustc crates from: {}", rust_src_path);
        
        let crates = self.discover_rustc_crates(rust_src_path)?;
        println!("📦 Found {} rustc crates", crates.len());

        // Assign each crate to Monster Group prime factors
        for (idx, crate_info) in crates.iter().enumerate() {
            let assignment = self.assign_crate_to_monster_prime(crate_info, idx)?;
            self.crate_assignments.push(assignment);
        }

        println!("✓ Assigned {} crates to Monster Group prime factors", self.crate_assignments.len());
        Ok(())
    }

    /// Verify rustc ≡ Monster Group in terms of exponents
    pub fn verify_monster_equivalence(&self) -> Result<bool, String> {
        println!("🧮 Verifying rustc ≡ Monster Group equivalence...");

        // Calculate total exponent usage
        let mut total_usage = HashMap::new();
        for assignment in &self.crate_assignments {
            *total_usage.entry(assignment.assigned_prime).or_insert(0) += assignment.assigned_exponent;
        }

        // Compare with Monster Group target
        let mut is_equivalent = true;
        let mut discrepancies = Vec::new();

        for (&prime, &target_exp) in &self.monster_target {
            let actual_exp = total_usage.get(&prime).unwrap_or(&0);
            if *actual_exp != target_exp {
                is_equivalent = false;
                discrepancies.push(format!("Prime {}: target={}, actual={}", prime, target_exp, actual_exp));
            }
        }

        if is_equivalent {
            println!("✅ VERIFIED: rustc ≡ Monster Group (M)");
            println!("   All prime exponents match exactly!");
        } else {
            println!("❌ DISCREPANCY: rustc ≢ Monster Group");
            for disc in &discrepancies {
                println!("   {}", disc);
            }
        }

        Ok(is_equivalent)
    }

    /// Discover rustc crates from source path
    fn discover_rustc_crates(&self, rust_src_path: &str) -> Result<Vec<RustcCrateInfo>, String> {
        let mut crates = Vec::new();
        
        // Core rustc crates (simplified discovery)
        let core_crates = vec![
            "rustc_driver", "rustc_interface", "rustc_session", "rustc_ast", "rustc_parse",
            "rustc_hir", "rustc_middle", "rustc_mir_build", "rustc_mir_transform", "rustc_codegen_llvm",
            "rustc_metadata", "rustc_resolve", "rustc_typeck", "rustc_trait_selection", "rustc_infer",
            "rustc_borrowck", "rustc_const_eval", "rustc_monomorphize", "rustc_llvm", "rustc_target",
            "rustc_serialize", "rustc_span", "rustc_errors", "rustc_data_structures", "rustc_index",
            "rustc_arena", "rustc_macros", "rustc_lexer", "rustc_feature", "rustc_attr",
            "rustc_builtin_macros", "rustc_expand", "rustc_save_analysis", "rustc_plugin_impl",
            "rustc_privacy", "rustc_passes", "rustc_incremental", "rustc_query_system", "rustc_ty_utils",
            "rustc_traits", "rustc_symbol_mangling", "rustc_lint", "rustc_lint_defs", "rustc_plugin",
            "rustc_fs_util", "rustc_back", "rustc_platform_intrinsics", "rustc_trans_utils"
        ];

        for crate_name in core_crates {
            let crate_path = format!("{}/compiler/{}", rust_src_path, crate_name);
            if Path::new(&crate_path).exists() {
                crates.push(RustcCrateInfo {
                    name: crate_name.to_string(),
                    path: crate_path,
                    category: self.categorize_crate(crate_name),
                });
            }
        }

        // Add library crates
        let lib_crates = vec!["std", "core", "alloc", "proc_macro", "test", "rustc_std_workspace_core"];
        for crate_name in lib_crates {
            let crate_path = format!("{}/library/{}", rust_src_path, crate_name);
            if Path::new(&crate_path).exists() {
                crates.push(RustcCrateInfo {
                    name: crate_name.to_string(),
                    path: crate_path,
                    category: RustcComponentCategory::CoreArchitecture,
                });
            }
        }

        Ok(crates)
    }

    /// Assign crate to Monster Group prime factor
    fn assign_crate_to_monster_prime(&mut self, crate_info: &RustcCrateInfo, idx: usize) -> Result<RustcCrateAssignment, String> {
        // Distribute crates across Monster Group primes based on category and hash
        let (prime, max_exponent) = self.select_prime_for_crate(crate_info, idx);
        
        // Calculate exponent based on crate importance/size (simplified)
        let exponent = self.calculate_crate_exponent(crate_info, max_exponent);
        
        // Update usage tracking
        *self.prime_usage.entry(prime).or_insert(0) += exponent;

        // Generate Gödel signature
        let godel_signature = self.create_godel_signature(prime, exponent);

        Ok(RustcCrateAssignment {
            crate_name: crate_info.name.clone(),
            crate_path: crate_info.path.clone(),
            assigned_prime: prime,
            assigned_exponent: exponent,
            monster_reason_id: self.get_reason_id(prime),
            semantic_category: crate_info.category.clone(),
            godel_signature,
        })
    }

    /// Select appropriate prime for crate based on category
    fn select_prime_for_crate(&self, crate_info: &RustcCrateInfo, idx: usize) -> (u64, u32) {
        match crate_info.category {
            RustcComponentCategory::CoreArchitecture => {
                // Use 2^46 for core architecture
                if crate_info.name.contains("driver") || crate_info.name.contains("interface") {
                    (2, 46)
                } else {
                    (5, 9) // 5^9 for other core components
                }
            },
            RustcComponentCategory::ASTComposition => {
                // Use 3^20 for AST-related crates
                (3, 20)
            },
            RustcComponentCategory::ModularForms => {
                // Distribute across modular form primes
                let primes = [11, 17, 19, 41, 47, 59];
                let prime_idx = idx % primes.len();
                let prime = primes[prime_idx];
                let exp = if prime == 11 { 2 } else { 1 };
                (prime, exp)
            },
            RustcComponentCategory::Verification => {
                // Use verification primes
                let primes = [23, 29, 31, 71];
                let prime_idx = idx % primes.len();
                (primes[prime_idx], 1)
            },
        }
    }

    /// Calculate exponent for crate (simplified heuristic)
    fn calculate_crate_exponent(&self, crate_info: &RustcCrateInfo, max_exponent: u32) -> u32 {
        // Simplified: assign based on crate importance
        if crate_info.name.contains("driver") || crate_info.name == "std" {
            max_exponent / 4 // Major components get 1/4 of max
        } else if crate_info.name.contains("middle") || crate_info.name.contains("hir") {
            max_exponent / 8 // Important components get 1/8
        } else {
            1 // Most crates get exponent 1
        }
    }

    /// Create Gödel signature for crate
    fn create_godel_signature(&self, prime: u64, exponent: u32) -> GödelNumber {
        let mut exponents = [0u8; 15];
        
        // Find prime index and set exponent
        for (i, &(p, _)) in MONSTER_PRIME_EXPONENTS.iter().enumerate() {
            if p == prime {
                exponents[i] = exponent.min(255) as u8; // Clamp to u8 range
                break;
            }
        }
        
        GödelNumber { exponents }
    }

    /// Get Monster reason ID for prime
    fn get_reason_id(&self, prime: u64) -> u32 {
        // Map prime to reason ID (simplified)
        match prime {
            2 => 1,   // Core architecture
            3 => 47,  // AST composition starts at reason 47
            5 => 2,   // Core architecture
            7 => 3,   // Core architecture
            _ => 67 + (prime % 10) as u32, // Modular forms start at 67
        }
    }

    /// Categorize crate by name
    fn categorize_crate(&self, crate_name: &str) -> RustcComponentCategory {
        if crate_name.contains("ast") || crate_name.contains("hir") || crate_name.contains("parse") {
            RustcComponentCategory::ASTComposition
        } else if crate_name.contains("mir") || crate_name.contains("codegen") || crate_name.contains("llvm") {
            RustcComponentCategory::ModularForms
        } else if crate_name.contains("borrowck") || crate_name.contains("trait") || crate_name.contains("infer") {
            RustcComponentCategory::Verification
        } else {
            RustcComponentCategory::CoreArchitecture
        }
    }

    /// Print assignment summary
    pub fn print_assignment_summary(&self) {
        println!("\n📊 Monster Group Assignment Summary:");
        println!("═══════════════════════════════════════");
        
        for &(prime, target_exp) in &MONSTER_PRIME_EXPONENTS {
            let used_exp = self.prime_usage.get(&prime).unwrap_or(&0);
            let status = if *used_exp == target_exp { "✅" } else { "❌" };
            println!("{} Prime {}: {}/{} (target/actual)", status, prime, target_exp, used_exp);
        }

        println!("\n🔍 Crate Assignments by Category:");
        for category in [
            RustcComponentCategory::CoreArchitecture,
            RustcComponentCategory::ASTComposition, 
            RustcComponentCategory::ModularForms,
            RustcComponentCategory::Verification
        ] {
            let count = self.crate_assignments.iter()
                .filter(|a| std::mem::discriminant(&a.semantic_category) == std::mem::discriminant(&category))
                .count();
            println!("  {:?}: {} crates", category, count);
        }
    }
}

#[derive(Debug, Clone)]
struct RustcCrateInfo {
    name: String,
    path: String,
    category: RustcComponentCategory,
}
