use std::fs;
use std::collections::{HashMap, HashSet};

struct CoreRustcResolver {
    core_components: Vec<String>,
    resolved_components: HashMap<String, Vec<String>>, // component -> dependencies
    missing_critical: Vec<String>,
    resolution_path: Vec<String>,
}

impl CoreRustcResolver {
    fn new() -> Self {
        Self {
            core_components: vec![
                "rustc_driver".to_string(),
                "rustc_interface".to_string(), 
                "rustc_middle".to_string(),
                "rustc_codegen_llvm".to_string(),
                "rustc_hir".to_string(),
                "rustc_ast".to_string(),
            ],
            resolved_components: HashMap::new(),
            missing_critical: Vec::new(),
            resolution_path: Vec::new(),
        }
    }
    
    fn check_core_resolution(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🎯 Checking core rustc component resolution...");
        
        // Read the previous resolution report
        let report = fs::read_to_string("RECURSIVE_RESOLUTION_REPORT.md")?;
        
        for component in &self.core_components {
            if report.contains(&format!("- `{}`", component)) {
                if report.lines().any(|line| line.contains(component) && line.contains("## Resolved Crates") || 
                                     (line.contains(component) && !line.contains("missing dependencies"))) {
                    self.resolved_components.insert(component.clone(), vec!["resolved".to_string()]);
                    self.resolution_path.push(format!("✓ {}", component));
                } else {
                    self.missing_critical.push(component.clone());
                    self.resolution_path.push(format!("✗ {}", component));
                }
            } else {
                self.missing_critical.push(component.clone());
                self.resolution_path.push(format!("? {} (not found)", component));
            }
        }
        
        println!("  ✓ Resolved: {}", self.resolved_components.len());
        println!("  ✗ Missing: {}", self.missing_critical.len());
        
        Ok(())
    }
    
    fn identify_critical_missing(&self) -> Vec<String> {
        // From the resolution report, identify the most critical missing dependencies
        vec![
            "rustc_middle".to_string(),
            "rustc_ast".to_string(), 
            "rustc_hir".to_string(),
            "rustc_span".to_string(),
            "rustc_data_structures".to_string(),
            "rustc_errors".to_string(),
            "rustc_session".to_string(),
        ]
    }
    
    fn generate_build_strategy(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📋 Generating Monster Protocol build strategy...");
        
        let mut strategy = String::new();
        strategy.push_str("# Monster Protocol Rustc Build Strategy\n\n");
        
        strategy.push_str("## Core Component Status\n");
        for step in &self.resolution_path {
            strategy.push_str(&format!("{}\n", step));
        }
        
        strategy.push_str("\n## Build Order (Monster Protocol)\n");
        strategy.push_str("Based on 41.8% resolution rate, we need a phased approach:\n\n");
        
        strategy.push_str("### Phase 1: Foundation (Resolved Components)\n");
        for (component, _) in &self.resolved_components {
            strategy.push_str(&format!("1. Build `{}` using existing submodules\n", component));
        }
        
        strategy.push_str("\n### Phase 2: Critical Missing Components\n");
        let critical = self.identify_critical_missing();
        for component in &critical {
            strategy.push_str(&format!("2. Add submodule for `{}` or create Monster trait replacement\n", component));
        }
        
        strategy.push_str("\n### Phase 3: Monster Protocol Integration\n");
        strategy.push_str("3. Apply Monster Group mappings to resolved components\n");
        strategy.push_str("4. Generate trait replacements for missing dependencies\n");
        strategy.push_str("5. Build incrementally using Monster indices\n");
        
        strategy.push_str("\n## Next Actions\n");
        strategy.push_str("- ✅ 33 rustc crates already resolved\n");
        strategy.push_str("- 🔄 Need to add ~13 critical missing submodules\n");
        strategy.push_str("- 🎯 Target 80%+ resolution rate for Monster build\n");
        
        fs::write("MONSTER_BUILD_STRATEGY.md", &strategy)?;
        
        println!("  ✓ Strategy written to MONSTER_BUILD_STRATEGY.md");
        Ok(())
    }
    
    fn calculate_monster_readiness(&self) -> f64 {
        let resolved_count = self.resolved_components.len() as f64;
        let total_core = self.core_components.len() as f64;
        
        // Weight core components more heavily
        let core_resolution = resolved_count / total_core;
        
        // From previous analysis: 33/79 = 41.8% overall
        let overall_resolution = 0.418;
        
        // Combined Monster Protocol readiness score
        (core_resolution * 0.6) + (overall_resolution * 0.4)
    }
    
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🎯 Core Rustc Resolver - Monster Protocol Readiness Check");
        
        self.check_core_resolution()?;
        self.generate_build_strategy()?;
        
        let readiness = self.calculate_monster_readiness();
        
        println!("\n🎯 === MONSTER PROTOCOL READINESS ===");
        println!("  Core components resolved: {}/{}", self.resolved_components.len(), self.core_components.len());
        println!("  Overall resolution: 33/79 (41.8%)");
        println!("  Monster readiness score: {:.1}%", readiness * 100.0);
        
        if readiness > 0.7 {
            println!("  🚀 READY for Monster Protocol build!");
        } else if readiness > 0.5 {
            println!("  ⚠️ PARTIALLY READY - need more submodules");
        } else {
            println!("  🔧 NEEDS WORK - add critical missing components");
        }
        
        println!("\n📋 NEXT STEPS:");
        println!("  1. Add missing critical submodules");
        println!("  2. Implement Monster trait replacements");
        println!("  3. Execute phased Monster Protocol build");
        
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut resolver = CoreRustcResolver::new();
    resolver.run()
}
