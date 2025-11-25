use std::io::{self, Write};
use std::collections::HashMap;

struct InteractiveRustcConstraintTweaker {
    our_generated: HashMap<String, u8>,    // our_code -> monster_index
    rustc_compiler: HashMap<String, u8>,   // rustc_code -> monster_index
    constraints: Vec<String>,
    partial_matches: Vec<(String, String, f64)>, // (our_code, rustc_code, similarity)
}

impl InteractiveRustcConstraintTweaker {
    fn new() -> Self {
        Self {
            our_generated: HashMap::new(),
            rustc_compiler: HashMap::new(),
            constraints: Vec::new(),
            partial_matches: Vec::new(),
        }
    }
    
    fn load_code_mappings(&mut self) {
        // Our generated code (we know exact Monster indices)
        self.our_generated.insert("SelfDescribingCode".to_string(), 42);
        self.our_generated.insert("ZKProgramProver".to_string(), 156);
        self.our_generated.insert("MonsterTransformation".to_string(), 89);
        
        // Rustc compiler (estimated Monster indices - partial matches)
        self.rustc_compiler.insert("rustc_driver::main".to_string(), 41);    // Close to our 42
        self.rustc_compiler.insert("rustc_interface::run".to_string(), 155);  // Close to our 156
        self.rustc_compiler.insert("rustc_middle::ty".to_string(), 88);       // Close to our 89
    }
    
    fn find_partial_matches(&mut self) {
        println!("🔍 Finding partial matches with rustc compiler...");
        
        for (our_name, &our_idx) in &self.our_generated {
            for (rustc_name, &rustc_idx) in &self.rustc_compiler {
                let distance = (our_idx as i16 - rustc_idx as i16).abs() as f64;
                let similarity = 1.0 - (distance / 192.0); // Normalize by Monster group size
                
                if similarity > 0.95 { // High similarity threshold
                    self.partial_matches.push((our_name.clone(), rustc_name.clone(), similarity));
                    println!("  {} [{}] ≈ {} [{}] ({:.1}% similar)", 
                             our_name, our_idx, rustc_name, rustc_idx, similarity * 100.0);
                }
            }
        }
    }
    
    fn interactive_constraint_review(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n🔧 Interactive constraint review:");
        
        for (our_code, rustc_code, similarity) in &self.partial_matches {
            let our_idx = self.our_generated[our_code];
            let rustc_idx = self.rustc_compiler[rustc_code];
            
            println!("\nMatch: {} ≈ {} ({:.1}% similar)", our_code, rustc_code, similarity * 100.0);
            let proposed = format!("constraint abs(our[{}] - rustc[{}]) <= 2;", our_idx, rustc_idx);
            println!("Proposed: {}", proposed);
            
            print!("Accept? (y/n/tweak): ");
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            
            match input.trim() {
                "y" => {
                    self.constraints.push(proposed);
                    println!("✓ Accepted");
                }
                "n" => {
                    println!("✗ Rejected");
                }
                "tweak" => {
                    print!("New tolerance (current: 2): ");
                    io::stdout().flush()?;
                    let mut tolerance_input = String::new();
                    io::stdin().read_line(&mut tolerance_input)?;
                    let tolerance: u8 = tolerance_input.trim().parse().unwrap_or(2);
                    
                    let tweaked = format!("constraint abs(our[{}] - rustc[{}]) <= {};", 
                                         our_idx, rustc_idx, tolerance);
                    self.constraints.push(tweaked.clone());
                    println!("✓ Tweaked: {}", tweaked);
                }
                _ => println!("? Skipped"),
            }
        }
        
        Ok(())
    }
    
    fn generate_final_model(&self) -> String {
        let mut model = String::new();
        model.push_str("% Interactive Rustc Partial Matching\n");
        model.push_str("array[0..191] of var 0..1: our;\n");
        model.push_str("array[0..191] of var 0..1: rustc;\n\n");
        
        for constraint in &self.constraints {
            model.push_str(&format!("{}\n", constraint));
        }
        
        model.push_str("\nsolve satisfy;\n");
        model
    }
    
    fn run_interactive_session(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🎯 Interactive Rustc Constraint Tweaker");
        
        self.load_code_mappings();
        self.find_partial_matches();
        self.interactive_constraint_review()?;
        
        let model = self.generate_final_model();
        std::fs::write("interactive_rustc_constraints.mzn", &model)?;
        
        println!("\n📝 Final model with {} constraints:", self.constraints.len());
        println!("{}", model);
        
        println!("✓ Saved to interactive_rustc_constraints.mzn");
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tweaker = InteractiveRustcConstraintTweaker::new();
    tweaker.run_interactive_session()
}
