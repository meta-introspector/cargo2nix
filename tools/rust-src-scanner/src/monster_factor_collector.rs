/// AST-based Monster Group factor collection
/// Maps Rust syntax elements to Monster Group prime factors
use syn::{visit::Visit, Item, Expr, Type, Pat, Stmt};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Monster Group primes for factor assignment
const MONSTER_PRIMES: [u64; 15] = [2,3,5,7,11,13,17,19,23,29,31,41,47,59,71];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileFactors {
    pub file_path: String,
    pub term_counts: HashMap<String, usize>,
    pub assigned_factors: Vec<(u64, u32)>, // prime^exponent
    pub total_contribution: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterFactorMapping {
    pub term_to_prime: HashMap<String, u64>,
    pub file_factors: Vec<FileFactors>,
    pub total_factors_used: HashMap<u64, u32>, // prime -> total exponent
    pub target_monster: HashMap<u64, u32>, // Monster Group target
}

pub struct ASTFactorCollector {
    pub term_counts: HashMap<String, usize>,
}

impl ASTFactorCollector {
    pub fn new() -> Self {
        Self {
            term_counts: HashMap::new(),
        }
    }

    pub fn collect_file_factors(&mut self, file_path: &str, syntax_tree: &syn::File) -> FileFactors {
        self.term_counts.clear();
        self.visit_file(syntax_tree);
        
        let assigned_factors = self.calculate_factors();
        let total_contribution = assigned_factors.iter()
            .map(|(p, e)| p.pow(*e))
            .product();

        FileFactors {
            file_path: file_path.to_string(),
            term_counts: self.term_counts.clone(),
            assigned_factors,
            total_contribution,
        }
    }

    fn calculate_factors(&self) -> Vec<(u64, u32)> {
        let mut factors = Vec::new();
        let mapping = self.create_term_mapping();
        
        for (term, &count) in &self.term_counts {
            if let Some(&prime) = mapping.get(term) {
                let exponent = self.count_to_exponent(count);
                factors.push((prime, exponent));
            }
        }
        
        factors
    }

    fn create_term_mapping(&self) -> HashMap<String, u64> {
        let mut mapping = HashMap::new();
        
        // Core language constructs → Monster Group primes
        mapping.insert("fn".to_string(), 71);           // Functions → highest prime
        mapping.insert("struct".to_string(), 59);       // Structs → second highest
        mapping.insert("enum".to_string(), 47);         // Enums
        mapping.insert("trait".to_string(), 41);        // Traits
        mapping.insert("impl".to_string(), 31);         // Implementations
        mapping.insert("mod".to_string(), 29);          // Modules
        mapping.insert("use".to_string(), 23);          // Imports
        mapping.insert("let".to_string(), 19);          // Bindings
        mapping.insert("match".to_string(), 17);        // Pattern matching
        mapping.insert("if".to_string(), 13);           // Conditionals
        mapping.insert("for".to_string(), 11);          // Loops
        mapping.insert("while".to_string(), 7);         // While loops
        mapping.insert("type".to_string(), 5);          // Type aliases
        mapping.insert("const".to_string(), 3);         // Constants
        mapping.insert("static".to_string(), 2);        // Statics
        
        mapping
    }

    fn count_to_exponent(&self, count: usize) -> u32 {
        match count {
            0 => 0,
            1..=2 => 1,
            3..=8 => 2,
            9..=32 => 3,
            33..=128 => 4,
            129..=512 => 5,
            _ => 6,
        }
    }
}

impl<'ast> Visit<'ast> for ASTFactorCollector {
    fn visit_item(&mut self, item: &'ast Item) {
        match item {
            Item::Fn(_) => *self.term_counts.entry("fn".to_string()).or_insert(0) += 1,
            Item::Struct(_) => *self.term_counts.entry("struct".to_string()).or_insert(0) += 1,
            Item::Enum(_) => *self.term_counts.entry("enum".to_string()).or_insert(0) += 1,
            Item::Trait(_) => *self.term_counts.entry("trait".to_string()).or_insert(0) += 1,
            Item::Impl(_) => *self.term_counts.entry("impl".to_string()).or_insert(0) += 1,
            Item::Mod(_) => *self.term_counts.entry("mod".to_string()).or_insert(0) += 1,
            Item::Use(_) => *self.term_counts.entry("use".to_string()).or_insert(0) += 1,
            Item::Type(_) => *self.term_counts.entry("type".to_string()).or_insert(0) += 1,
            Item::Const(_) => *self.term_counts.entry("const".to_string()).or_insert(0) += 1,
            Item::Static(_) => *self.term_counts.entry("static".to_string()).or_insert(0) += 1,
            _ => {}
        }
        syn::visit::visit_item(self, item);
    }

    fn visit_expr(&mut self, expr: &'ast Expr) {
        match expr {
            Expr::Match(_) => *self.term_counts.entry("match".to_string()).or_insert(0) += 1,
            Expr::If(_) => *self.term_counts.entry("if".to_string()).or_insert(0) += 1,
            Expr::ForLoop(_) => *self.term_counts.entry("for".to_string()).or_insert(0) += 1,
            Expr::While(_) => *self.term_counts.entry("while".to_string()).or_insert(0) += 1,
            Expr::Let(_) => *self.term_counts.entry("let".to_string()).or_insert(0) += 1,
            _ => {}
        }
        syn::visit::visit_expr(self, expr);
    }
}

pub struct MonsterFactorSolver {
    pub mapping: MonsterFactorMapping,
}

impl MonsterFactorSolver {
    pub fn new() -> Self {
        let mut target_monster = HashMap::new();
        let monster_factors = [(2,46), (3,20), (5,9), (7,6), (11,2), (13,3), 
                              (17,1), (19,1), (23,1), (29,1), (31,1), (41,1), 
                              (47,1), (59,1), (71,1)];
        
        for (prime, exp) in monster_factors {
            target_monster.insert(prime, exp);
        }

        Self {
            mapping: MonsterFactorMapping {
                term_to_prime: HashMap::new(),
                file_factors: Vec::new(),
                total_factors_used: HashMap::new(),
                target_monster,
            }
        }
    }

    pub fn add_file_factors(&mut self, file_factors: FileFactors) {
        // Accumulate total factors used
        for (prime, exp) in &file_factors.assigned_factors {
            *self.mapping.total_factors_used.entry(*prime).or_insert(0) += exp;
        }
        
        self.mapping.file_factors.push(file_factors);
    }

    pub fn check_monster_constraint(&self) -> (bool, Vec<String>) {
        let mut issues = Vec::new();
        let mut valid = true;

        for (&prime, &target_exp) in &self.mapping.target_monster {
            let used_exp = self.mapping.total_factors_used.get(&prime).copied().unwrap_or(0);
            
            if used_exp > target_exp {
                issues.push(format!("Prime {} exceeds limit: {} > {}", prime, used_exp, target_exp));
                valid = false;
            } else if used_exp < target_exp {
                issues.push(format!("Prime {} under-utilized: {} < {}", prime, used_exp, target_exp));
            }
        }

        (valid, issues)
    }

    pub fn generate_sat_problem(&self) -> String {
        // Generate SAT clauses for exact Monster Group matching
        let mut clauses = Vec::new();
        
        for (&prime, &target_exp) in &self.mapping.target_monster {
            let used_exp = self.mapping.total_factors_used.get(&prime).copied().unwrap_or(0);
            clauses.push(format!("prime_{}_target_{}_used_{}", prime, target_exp, used_exp));
        }
        
        clauses.join("\n")
    }

    pub fn print_summary(&self) {
        println!("🔢 Monster Factor Collection Summary");
        println!("Files analyzed: {}", self.mapping.file_factors.len());
        
        println!("\nFactor usage vs Monster Group targets:");
        for (&prime, &target_exp) in &self.mapping.target_monster {
            let used_exp = self.mapping.total_factors_used.get(&prime).copied().unwrap_or(0);
            let status = if used_exp == target_exp { "✅" } 
                        else if used_exp < target_exp { "⬇️" } 
                        else { "⬆️" };
            println!("  {} : {} / {} {}", prime, used_exp, target_exp, status);
        }

        let (valid, issues) = self.check_monster_constraint();
        if valid {
            println!("✅ Monster Group constraint satisfied!");
        } else {
            println!("❌ Constraint violations:");
            for issue in issues {
                println!("  {}", issue);
            }
        }
    }
}
