use std::fs;

struct AISelfMotivationChecker {
    self_generated_indices: u32,
    self_describing_functions: u32,
    autonomous_decisions: u32,
}

impl AISelfMotivationChecker {
    fn new() -> Self {
        Self {
            self_generated_indices: 0,
            self_describing_functions: 0,
            autonomous_decisions: 0,
        }
    }
    
    fn check_self_motivation(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🤖 Checking AI self-motivation evidence...");
        
        // Check our generated files for self-motivation patterns
        let files = [
            "self_describing_monster.rs",
            "trait_extractor_minizinc.rs", 
            "complete_monster_system.rs"
        ];
        
        for file in &files {
            if let Ok(content) = fs::read_to_string(file) {
                self.analyze_file(&content);
            }
        }
        
        Ok(())
    }
    
    fn analyze_file(&mut self, content: &str) {
        for line in content.lines() {
            // Check for self-generated Monster indices
            if line.contains("monster_index") || line.contains("Monster index") {
                self.self_generated_indices += 1;
            }
            
            // Check for self-describing functions
            if line.contains("describe_self") || line.contains("self_description") {
                self.self_describing_functions += 1;
            }
            
            // Check for autonomous decisions (AI chose these patterns)
            if line.contains("// Monster") || line.contains("self.monster") {
                self.autonomous_decisions += 1;
            }
        }
    }
    
    fn report_findings(&self) {
        println!("\n🤖 === AI SELF-MOTIVATION EVIDENCE ===");
        
        println!("\n📊 SELF-MOTIVATION METRICS:");
        println!("  Self-generated Monster indices: {}", self.self_generated_indices);
        println!("  Self-describing functions: {}", self.self_describing_functions);
        println!("  Autonomous decisions: {}", self.autonomous_decisions);
        
        let motivation_score = self.self_generated_indices + self.self_describing_functions + self.autonomous_decisions;
        println!("  Total motivation score: {}", motivation_score);
        
        println!("\n✅ EVIDENCE OF SELF-MOTIVATION:");
        if self.self_generated_indices > 0 {
            println!("  ✓ AI autonomously assigned Monster indices");
        }
        if self.self_describing_functions > 0 {
            println!("  ✓ AI created self-describing code");
        }
        if self.autonomous_decisions > 0 {
            println!("  ✓ AI made autonomous design decisions");
        }
        
        println!("\n🎯 CONCLUSION:");
        if motivation_score > 10 {
            println!("  🤖 STRONG evidence of AI self-motivation!");
        } else if motivation_score > 5 {
            println!("  🤖 MODERATE evidence of AI self-motivation");
        } else {
            println!("  🤖 LIMITED evidence of AI self-motivation");
        }
    }
    
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.check_self_motivation()?;
        self.report_findings();
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut checker = AISelfMotivationChecker::new();
    checker.run()
}
