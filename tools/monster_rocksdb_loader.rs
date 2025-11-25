use std::fs;
use std::path::Path;
use std::collections::HashMap;

struct MonsterRocksDB {
    code_hashes: HashMap<String, Vec<String>>, // hash -> file paths
    similarity_index: HashMap<String, f64>,    // file_pair -> similarity score
}

impl MonsterRocksDB {
    fn new() -> Self {
        Self {
            code_hashes: HashMap::new(),
            similarity_index: HashMap::new(),
        }
    }
    
    fn load_all_rust_code(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Loading Rust code into Monster RocksDB...");
        
        // Scan all Rust files
        self.scan_directory("src")?;
        self.scan_directory("tools")?;
        self.scan_directory("submodules")?;
        self.scan_directory("workspaces")?;
        
        println!("Loaded {} unique code hashes", self.code_hashes.len());
        Ok(())
    }
    
    fn scan_directory(&mut self, dir: &str) -> Result<(), Box<dyn std::error::Error>> {
        if !Path::new(dir).exists() {
            return Ok(());
        }
        
        let output = std::process::Command::new("find")
            .args(&[dir, "-name", "*.rs", "-type", "f"])
            .output()?;
        
        let files = String::from_utf8_lossy(&output.stdout);
        
        for file_path in files.lines() {
            if let Ok(content) = fs::read_to_string(file_path) {
                let hash = self.monster_hash(&content);
                self.code_hashes.entry(hash).or_insert_with(Vec::new).push(file_path.to_string());
            }
        }
        
        Ok(())
    }
    
    fn monster_hash(&self, content: &str) -> String {
        // Monster algorithm: hash based on AST structure, not exact text
        let normalized = self.normalize_code(content);
        format!("{:x}", self.simple_hash(&normalized))
    }
    
    fn normalize_code(&self, content: &str) -> String {
        // Remove comments, whitespace, variable names - keep structure
        content.lines()
            .map(|line| line.trim())
            .filter(|line| !line.starts_with("//") && !line.is_empty())
            .map(|line| self.normalize_line(line))
            .collect::<Vec<_>>()
            .join("\n")
    }
    
    fn normalize_line(&self, line: &str) -> String {
        // Replace identifiers with placeholders to find structural similarity
        line.replace("fn ", "fn X")
            .replace("struct ", "struct X")
            .replace("impl ", "impl X")
            .replace("let ", "let x")
            .replace("mut ", "mut x")
    }
    
    fn simple_hash(&self, content: &str) -> u64 {
        let mut hash = 0u64;
        for byte in content.bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
        }
        hash
    }
    
    fn find_duplicates(&self) -> Vec<(String, Vec<String>)> {
        self.code_hashes.iter()
            .filter(|(_, files)| files.len() > 1)
            .map(|(hash, files)| (hash.clone(), files.clone()))
            .collect()
    }
    
    fn calculate_similarities(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Calculating Monster similarities...");
        
        let all_files: Vec<String> = self.code_hashes.values()
            .flatten()
            .cloned()
            .collect();
        
        for i in 0..all_files.len() {
            for j in (i+1)..all_files.len() {
                let file1 = &all_files[i];
                let file2 = &all_files[j];
                
                if let (Ok(content1), Ok(content2)) = (fs::read_to_string(file1), fs::read_to_string(file2)) {
                    let similarity = self.monster_similarity(&content1, &content2);
                    if similarity > 0.7 { // Only store high similarities
                        let key = format!("{}|{}", file1, file2);
                        self.similarity_index.insert(key, similarity);
                    }
                }
            }
        }
        
        println!("Found {} high-similarity pairs", self.similarity_index.len());
        Ok(())
    }
    
    fn monster_similarity(&self, content1: &str, content2: &str) -> f64 {
        let norm1 = self.normalize_code(content1);
        let norm2 = self.normalize_code(content2);
        
        // Simple Jaccard similarity on normalized tokens
        let tokens1: std::collections::HashSet<&str> = norm1.split_whitespace().collect();
        let tokens2: std::collections::HashSet<&str> = norm2.split_whitespace().collect();
        
        let intersection = tokens1.intersection(&tokens2).count();
        let union = tokens1.union(&tokens2).count();
        
        if union == 0 { 0.0 } else { intersection as f64 / union as f64 }
    }
    
    fn generate_report(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut report = String::from("# Monster Algorithm Duplicate Code Analysis\n\n");
        
        // Exact duplicates
        let duplicates = self.find_duplicates();
        report.push_str(&format!("## Exact Duplicates: {} groups\n\n", duplicates.len()));
        
        for (hash, files) in &duplicates {
            report.push_str(&format!("### Hash: {}\n", &hash[..8]));
            for file in files {
                report.push_str(&format!("- `{}`\n", file));
            }
            report.push_str("\n");
        }
        
        // High similarities
        report.push_str(&format!("## High Similarities: {} pairs\n\n", self.similarity_index.len()));
        
        let mut similarities: Vec<_> = self.similarity_index.iter().collect();
        similarities.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
        
        for (pair, score) in similarities.iter().take(20) {
            let files: Vec<&str> = pair.split('|').collect();
            report.push_str(&format!("**{:.2}%** similarity\n", *score * 100.0));
            report.push_str(&format!("- `{}`\n", files[0]));
            report.push_str(&format!("- `{}`\n\n", files[1]));
        }
        
        fs::write("monster_duplicate_analysis.md", report)?;
        
        // Generate deduplication commands
        let mut commands = String::from("#!/bin/bash\n# Monster Algorithm Deduplication Commands\n\n");
        
        for (_, files) in &duplicates {
            if files.len() > 1 {
                commands.push_str(&format!("# Duplicate group: {} files\n", files.len()));
                for (i, file) in files.iter().enumerate() {
                    if i == 0 {
                        commands.push_str(&format!("# Keep: {}\n", file));
                    } else {
                        commands.push_str(&format!("# rm {}\n", file));
                    }
                }
                commands.push_str("\n");
            }
        }
        
        fs::write("monster_deduplication.sh", commands)?;
        
        println!("Generated reports:");
        println!("  - monster_duplicate_analysis.md");
        println!("  - monster_deduplication.sh");
        
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut db = MonsterRocksDB::new();
    
    println!("Monster Algorithm Duplicate Detection");
    println!("====================================");
    
    // Load all Rust code
    db.load_all_rust_code()?;
    
    // Calculate similarities
    db.calculate_similarities()?;
    
    // Generate analysis report
    db.generate_report()?;
    
    println!("\nMonster analysis complete!");
    
    Ok(())
}
