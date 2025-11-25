use std::io::{self, Write};
use std::collections::HashMap;

struct InteractiveRustcMatcher {
    our_code: HashMap<String, u8>,     // our code -> monster index
    rustc_code: HashMap<String, u8>,   // rustc code -> monster index  
    partial_matches: Vec<PartialMatch>,
    constraints: Vec<String>,
}

#[derive(Debug)]
struct PartialMatch {
    our_monster: u8,
    rustc_monster: u8,
    similarity: f64,
    constraint: String,
}

impl InteractiveRustcMatcher {
    fn new() -> Self {
        Self {
            our_code: HashMap::new(),
            rustc_code: HashMap::new(),
            partial_matches: Vec::new(),
            constraints: Vec::new(),
        }
    }
    
    fn load_code_mappings(&mut self) {
        // Our generated code (we know the Monster indices)
        self.our_code.insert("SelfDescribingMonster".to_string(), 42);
        self.our_code.insert("TraitExtractorMinizinc".to_string(), 89);
        self.our_code.insert("CompleteMonsterSystem".to_string(), 163);
        
        // Rustc code (estimated Monster indices)
        self.rustc_code.insert("rustc_driver::main".to_string(), 41);  // Close to our 42
        self.rustc_code.insert("rustc_interface::run".to_string(), 88); // Close to our 89
        self.rustc_code.insert("rustc_middle::ty".to_string(), 162);    // Close to our 163
    }
    
    fn find_partial_matches(&mut self) {
        println!("🔍 Finding partial matches with rustc...");
        
        for (our_name, &our_idx) in &self.our_code {
            for (rustc_name, &rustc_idx) in &self.rustc_code {
                let distance = (our_idx as i16 - rustc_idx as i16).abs() as f64;
                let similarity = 1.0 / (1.0 + distance / 192.0); // Normalize by monster group size
                
                if similarity > 0.8 { // High similarity threshold
                    let constraint = format!(
                        "constraint abs(our[{}] - rustc[{}]) <= 2;", 
                        our_idx, rustc_idx
                    );
                    
                    self.partial_matches.push(PartialMatch {
                        our_monster: our_idx,
                        rustc_monster: rustc_idx,
                        similarity,
                        constraint,
                    });
                    
                    println!("  {} [{}] ≈ {} [{}] (similarity: {:.2})", 
                             our_name, our_idx, rustc_name, rustc_idx, similarity);
                }
            }
        }
    }
    
    fn interactive_constraint_review(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n🔧 Interactive Constraint Review");
        println!("Found {} partial matches. Review each constraint:\n", self.partial_matches.len());
        
        for (i, match_info) in self.partial_matches.iter().enumerate() {
            println!("Match {}: Monster[{}] ≈ Monster[{}] (similarity: {:.2})", 
                     i + 1, match_info.our_monster, match_info.rustc_monster, match_info.similarity);
            println!("Proposed constraint: {}", match_info.constraint);
            
            print!("Accept this constraint? (y/n/tweak): ");
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            
            match input.trim().to_lowercase().as_str() {
                "y" | "yes" => {
                    self.constraints.push(match_info.constraint.clone());
                    println!("✓ Constraint accepted\n");
                }
                "n" | "no" => {
                    println!("✗ Constraint rejected\n");
                }
                "tweak" | "t" => {
                    let tweaked = self.tweak_constraint(match_info)?;
                    self.constraints.push(tweaked);
                    println!("✓ Tweaked constraint accepted\n");
                }
                _ => {
                    println!("? Invalid input, skipping\n");
                }
            }
        }
        
        Ok(())
    }
    
    fn tweak_constraint(&self, match_info: &PartialMatch) -> Result<String, Box<dyn std::error::Error>> {
        println!("Current constraint: {}", match_info.constraint);
        print!("Enter new tolerance (current: 2): ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        let tolerance: u8 = input.trim().parse().unwrap_or(2);
        let tweaked = format!(
            "constraint abs(our[{}] - rustc[{}]) <= {};", 
            match_info.our_monster, match_info.rustc_monster, tolerance
        );
        
        println!("Tweaked to: {}", tweaked);
        Ok(tweaked)
    }
    
    fn generate_minizinc_model(&self) -> String {
        let mut model = String::new();
        model.push_str("% Interactive Rustc Matching Model\n");
        model.push_str("array[0..191] of var 0..1: our;\n");
        model.push_str("array[0..191] of var 0..1: rustc;\n\n");
        
        // Add accepted constraints
        for constraint in &self.constraints {
            model.push_str(&format!("{}\n", constraint));
        }
        
        model.push_str("\nsolve satisfy;\n");
        model
    }
    
    fn run_interactive_session(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🎯 Interactive Rustc Matcher");
        println!("Matching our Monster code against rustc compiler\n");
        
        self.load_code_mappings();
        self.find_partial_matches();
        self.interactive_constraint_review()?;
        
        println!("📝 Final MiniZinc Model:");
        let model = self.generate_minizinc_model();
        println!("{}", model);
        
        std::fs::write("interactive_rustc_match.mzn", &model)?;
        println!("✓ Saved to interactive_rustc_match.mzn");
        
        println!("\n🚀 Accepted {} constraints for rustc matching", self.constraints.len());
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut matcher = InteractiveRustcMatcher::new();
    matcher.run_interactive_session()
}
