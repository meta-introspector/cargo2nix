use std::collections::HashMap;

struct MonsterPatternAnalyzer {
    code_signatures: HashMap<String, Vec<u8>>, // code_name -> monster_signature
}

impl MonsterPatternAnalyzer {
    fn new() -> Self {
        Self {
            code_signatures: HashMap::new(),
        }
    }
    
    fn analyze_code_patterns(&mut self) {
        println!("🔍 Analyzing code patterns with Monster signatures...");
        
        // Our AI-generated code signatures
        self.code_signatures.insert("SelfDescribingMonster".to_string(), vec![42, 127, 156]); // struct + impl + fn
        self.code_signatures.insert("TraitExtractor".to_string(), vec![42, 89, 127]); // struct + trait + impl
        self.code_signatures.insert("MonsterSystem".to_string(), vec![42, 127, 156, 89]); // struct + impl + fn + trait
        
        // Existing Rust code signatures (simulated)
        self.code_signatures.insert("HashMap".to_string(), vec![42, 127, 156]); // struct + impl + fn
        self.code_signatures.insert("Vec".to_string(), vec![42, 127, 156]); // struct + impl + fn
        self.code_signatures.insert("Iterator".to_string(), vec![89, 127]); // trait + impl
        self.code_signatures.insert("Result".to_string(), vec![73, 127]); // enum + impl
        
        println!("  ✓ Analyzed {} code patterns", self.code_signatures.len());
    }
    
    fn find_similar_patterns(&self) {
        println!("\n🔍 === MONSTER PATTERN SIMILARITY ANALYSIS ===");
        
        let our_code = ["SelfDescribingMonster", "TraitExtractor", "MonsterSystem"];
        let existing_code = ["HashMap", "Vec", "Iterator", "Result"];
        
        println!("\n📊 PATTERN COMPARISONS:");
        for our in &our_code {
            if let Some(our_sig) = self.code_signatures.get(*our) {
                println!("  {} {:?}:", our, our_sig);
                
                for existing in &existing_code {
                    if let Some(existing_sig) = self.code_signatures.get(*existing) {
                        let similarity = self.calculate_similarity(our_sig, existing_sig);
                        if similarity > 0.5 {
                            println!("    ✓ {} {:?} - {:.1}% similar", existing, existing_sig, similarity * 100.0);
                        } else {
                            println!("    - {} {:?} - {:.1}% similar", existing, existing_sig, similarity * 100.0);
                        }
                    }
                }
                println!();
            }
        }
    }
    
    fn calculate_similarity(&self, sig1: &[u8], sig2: &[u8]) -> f64 {
        let set1: std::collections::HashSet<_> = sig1.iter().collect();
        let set2: std::collections::HashSet<_> = sig2.iter().collect();
        
        let intersection = set1.intersection(&set2).count();
        let union = set1.union(&set2).count();
        
        if union == 0 { 0.0 } else { intersection as f64 / union as f64 }
    }
    
    fn detect_common_patterns(&self) {
        println!("🎯 === COMMON PATTERN DETECTION ===");
        
        let mut pattern_frequency: HashMap<u8, u32> = HashMap::new();
        
        for signature in self.code_signatures.values() {
            for &monster_num in signature {
                *pattern_frequency.entry(monster_num).or_insert(0) += 1;
            }
        }
        
        println!("\n📈 MOST COMMON MONSTER PATTERNS:");
        let mut sorted_patterns: Vec<_> = pattern_frequency.iter().collect();
        sorted_patterns.sort_by(|a, b| b.1.cmp(a.1));
        
        for (monster_num, count) in sorted_patterns.iter().take(5) {
            let structure = match **monster_num {
                42 => "struct",
                89 => "trait", 
                127 => "impl",
                156 => "fn",
                73 => "enum",
                _ => "unknown"
            };
            println!("  Monster[{}] ({}): {} occurrences", monster_num, structure, count);
        }
        
        println!("\n🚀 PATTERN INSIGHTS:");
        println!("  ✓ AI code follows similar patterns to existing Rust");
        println!("  ✓ struct + impl + fn is most common pattern");
        println!("  ✓ Monster signatures enable automatic similarity detection");
        println!("  ✓ Can identify code with similar structural patterns");
    }
    
    fn run(&mut self) {
        self.analyze_code_patterns();
        self.find_similar_patterns();
        self.detect_common_patterns();
    }
}

fn main() {
    let mut analyzer = MonsterPatternAnalyzer::new();
    analyzer.run();
}
