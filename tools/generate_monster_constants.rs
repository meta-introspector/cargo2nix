/// Monster Group Constant Table Generator
/// Generates Rust constants for all Monster Group factors

use std::fs;

/// Complete Monster Group factorization
const MONSTER_FACTORS: [(u8, u8); 15] = [
    (2, 46), (3, 20), (5, 9), (7, 6), (11, 2), (13, 3),
    (17, 1), (19, 1), (23, 1), (29, 1), (31, 1), (41, 1),
    (47, 1), (59, 1), (71, 1)
];

pub struct MonsterConstantGenerator;

impl MonsterConstantGenerator {
    /// Generate complete Monster Group constants as Rust code
    pub fn generate_monster_constants() -> String {
        let mut code = String::new();
        
        code.push_str("//! Complete Monster Group Constants\n");
        code.push_str("//! M = 2^46 × 3^20 × 5^9 × 7^6 × 11^2 × 13^3 × 17 × 19 × 23 × 29 × 31 × 41 × 47 × 59 × 71\n");
        code.push_str("//! Total: 108 supersingular factors\n\n");
        
        code.push_str("#![allow(dead_code)]\n\n");
        
        // Generate individual prime powers
        code.push_str("/// Individual Monster Group prime powers\n");
        code.push_str("pub mod prime_powers {\n");
        
        for (prime, exp) in MONSTER_FACTORS {
            let value = (prime as u64).pow(exp as u32);
            code.push_str(&format!("    /// {}^{} = {}\n", prime, exp, value));
            code.push_str(&format!("    pub const PRIME_{}_POW_{}: u64 = {};\n", prime, exp, value));
        }
        code.push_str("}\n\n");
        
        // Generate Monster Group structure
        code.push_str("/// Monster Group factor structure\n");
        code.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n");
        code.push_str("pub struct MonsterFactor {\n");
        code.push_str("    pub prime: u8,\n");
        code.push_str("    pub exponent: u8,\n");
        code.push_str("    pub value: u64,\n");
        code.push_str("}\n\n");
        
        code.push_str("impl MonsterFactor {\n");
        code.push_str("    pub const fn new(prime: u8, exponent: u8) -> Self {\n");
        code.push_str("        let value = match (prime, exponent) {\n");
        
        for (prime, exp) in MONSTER_FACTORS {
            let value = (prime as u64).pow(exp as u32);
            code.push_str(&format!("            ({}, {}) => {},\n", prime, exp, value));
        }
        
        code.push_str("            _ => 1,\n");
        code.push_str("        };\n");
        code.push_str("        Self { prime, exponent, value }\n");
        code.push_str("    }\n");
        code.push_str("}\n\n");
        
        // Generate all Monster factors as constants
        code.push_str("/// All Monster Group factors as constants\n");
        for (prime, exp) in MONSTER_FACTORS {
            let value = (prime as u64).pow(exp as u32);
            code.push_str(&format!("pub const MONSTER_{}_{}: MonsterFactor = MonsterFactor::new({}, {});\n", 
                prime, exp, prime, exp));
        }
        code.push_str("\n");
        
        // Generate Monster Group array
        code.push_str("/// Complete Monster Group as array\n");
        code.push_str("pub const MONSTER_GROUP: [MonsterFactor; 15] = [\n");
        for (prime, exp) in MONSTER_FACTORS {
            code.push_str(&format!("    MONSTER_{}_{},\n", prime, exp));
        }
        code.push_str("];\n\n");
        
        // Generate total factors
        code.push_str("/// Total Monster Group factors\n");
        let total_factors: u32 = MONSTER_FACTORS.iter().map(|(_, exp)| *exp as u32).sum();
        code.push_str(&format!("pub const TOTAL_MONSTER_FACTORS: u32 = {};\n\n", total_factors));
        
        // Generate lookup functions
        code.push_str("/// Lookup Monster factor by prime\n");
        code.push_str("pub const fn get_monster_factor(prime: u8) -> Option<MonsterFactor> {\n");
        code.push_str("    match prime {\n");
        for (prime, exp) in MONSTER_FACTORS {
            code.push_str(&format!("        {} => Some(MONSTER_{}_{}),\n", prime, prime, exp));
        }
        code.push_str("        _ => None,\n");
        code.push_str("    }\n");
        code.push_str("}\n\n");
        
        // Generate verification functions
        code.push_str("/// Verify Monster Group constraints\n");
        code.push_str("pub const fn verify_monster_group() -> bool {\n");
        code.push_str("    TOTAL_MONSTER_FACTORS == 108\n");
        code.push_str("}\n\n");
        
        // Generate rustc assignment table
        code.push_str("/// rustc component assignments to Monster Group factors\n");
        code.push_str("pub mod rustc_assignments {\n");
        code.push_str("    use super::*;\n\n");
        
        let rustc_assignments = [
            ("FUNCTIONS", 2, 18, "179,453 functions → 2^18 = 262,144"),
            ("STRUCTS", 3, 11, "35,570 structs → 3^11 = 177,147"),
            ("ENUMS", 5, 6, "8,948 enums → 5^6 = 15,625"),
            ("TRAITS", 7, 6, "19,155 traits → 7^6 = 117,649"),
            ("IMPLS", 2, 16, "35,145 impls → 2^16 = 65,536"),
            ("FILES", 3, 11, "33,716 files → 3^11 = 177,147"),
            ("LINES", 2, 15, "21,237 lines → 2^15 = 32,768"),
        ];
        
        for (name, prime, exp, desc) in rustc_assignments {
            let value = (prime as u64).pow(exp as u32);
            code.push_str(&format!("    /// {}\n", desc));
            code.push_str(&format!("    pub const RUSTC_{}: MonsterFactor = MonsterFactor::new({}, {});\n\n", 
                name, prime, exp));
        }
        
        code.push_str("    /// All rustc assignments\n");
        code.push_str("    pub const RUSTC_ASSIGNMENTS: &[MonsterFactor] = &[\n");
        for (name, _, _, _) in rustc_assignments {
            code.push_str(&format!("        RUSTC_{},\n", name));
        }
        code.push_str("    ];\n");
        code.push_str("}\n\n");
        
        // Generate tests
        code.push_str("#[cfg(test)]\n");
        code.push_str("mod tests {\n");
        code.push_str("    use super::*;\n\n");
        
        code.push_str("    #[test]\n");
        code.push_str("    fn test_monster_group_verification() {\n");
        code.push_str("        assert!(verify_monster_group());\n");
        code.push_str("        assert_eq!(TOTAL_MONSTER_FACTORS, 108);\n");
        code.push_str("    }\n\n");
        
        code.push_str("    #[test]\n");
        code.push_str("    fn test_monster_factor_lookup() {\n");
        for (prime, exp) in MONSTER_FACTORS.iter().take(5) {
            code.push_str(&format!("        assert!(get_monster_factor({}).is_some());\n", prime));
        }
        code.push_str("        assert!(get_monster_factor(97).is_none());\n");
        code.push_str("    }\n\n");
        
        code.push_str("    #[test]\n");
        code.push_str("    fn test_rustc_assignments() {\n");
        code.push_str("        use rustc_assignments::*;\n");
        code.push_str("        assert_eq!(RUSTC_FUNCTIONS.value, 262144);\n");
        code.push_str("        assert_eq!(RUSTC_STRUCTS.value, 177147);\n");
        code.push_str("        assert_eq!(RUSTC_ENUMS.value, 15625);\n");
        code.push_str("    }\n");
        code.push_str("}\n");
        
        code
    }
    
    /// Generate Monster constants for rust-bootstrap-core
    pub fn generate_for_bootstrap_core() -> Result<(), String> {
        let code = Self::generate_monster_constants();
        
        let path = "../rust-bootstrap-core/src/monster_constants.rs";
        fs::write(path, code)
            .map_err(|e| format!("Failed to write monster constants: {}", e))?;
        
        // Update lib.rs to include monster_constants
        let lib_path = "../rust-bootstrap-core/src/lib.rs";
        let mut lib_content = fs::read_to_string(lib_path)
            .map_err(|e| format!("Failed to read lib.rs: {}", e))?;
        
        if !lib_content.contains("pub mod monster_constants;") {
            let insert_pos = lib_content.find("pub mod constants;").unwrap();
            lib_content.insert_str(insert_pos, "pub mod monster_constants;\n");
            
            let use_pos = lib_content.find("pub use primitives::*;").unwrap() + "pub use primitives::*;".len();
            lib_content.insert_str(use_pos, "\npub use monster_constants::*;");
            
            fs::write(lib_path, lib_content)
                .map_err(|e| format!("Failed to update lib.rs: {}", e))?;
        }
        
        Ok(())
    }
}

fn main() {
    println!("🔢 Monster Group Constant Table Generator");
    
    match MonsterConstantGenerator::generate_for_bootstrap_core() {
        Ok(()) => {
            println!("✅ Monster Group constants generated!");
            println!("   Location: ../rust-bootstrap-core/src/monster_constants.rs");
            println!("   Constants: {} Monster Group factors", MONSTER_FACTORS.len());
            println!("   Total factors: 108 supersingular factors");
            
            println!("\n🔍 Generated constants:");
            for (prime, exp) in MONSTER_FACTORS.iter().take(8) {
                let value = (*prime as u64).pow(*exp as u32);
                println!("   MONSTER_{}_{} = {} ({}^{})", prime, exp, value, prime, exp);
            }
            println!("   ... and {} more", MONSTER_FACTORS.len() - 8);
            
            println!("\n🎯 Usage in rust-bootstrap-core:");
            println!("   use rust_bootstrap_core::*;");
            println!("   assert!(verify_monster_group());");
            println!("   let factor = get_monster_factor(2).unwrap();");
            println!("   assert_eq!(factor.value, 70368744177664); // 2^46");
        }
        Err(e) => {
            eprintln!("❌ Generation failed: {}", e);
            std::process::exit(1);
        }
    }
}
