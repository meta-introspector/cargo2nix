use std::fs;
use std::process::Command;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct CodeStructure {
    crate_name: String,
    libs: Vec<String>,           // lib.rs, main.rs files
    decls: Vec<String>,          // fn, struct, enum, trait declarations
    exports: Vec<String>,        // pub items
    signatures: Vec<String>,     // function signatures
    git_object: String,
}

struct CodeStructureAnalyzer {
    structures: HashMap<String, CodeStructure>,
}

impl CodeStructureAnalyzer {
    fn new() -> Self {
        Self {
            structures: HashMap::new(),
        }
    }
    
    fn analyze_submodule_code(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔍 Analyzing code structure in submodules...");
        
        // Find all Rust source files in submodules
        let output = Command::new("find")
            .args(&["../submodules", "-name", "*.rs", "-type", "f"])
            .output()?;
        
        if !output.status.success() {
            return Ok(());
        }
        
        let rust_files = String::from_utf8_lossy(&output.stdout);
        let mut processed_crates = HashMap::new();
        
        for rs_path in rust_files.lines() {
            // Extract crate name from path
            let crate_name = self.extract_crate_from_path(rs_path);
            
            if !processed_crates.contains_key(&crate_name) {
                processed_crates.insert(crate_name.clone(), true);
                
                let mut structure = CodeStructure {
                    crate_name: crate_name.clone(),
                    libs: Vec::new(),
                    decls: Vec::new(),
                    exports: Vec::new(),
                    signatures: Vec::new(),
                    git_object: self.get_git_object(&crate_name),
                };
                
                self.analyze_crate_structure(&crate_name, rs_path, &mut structure)?;
                self.structures.insert(crate_name, structure);
            }
        }
        
        println!("  ✓ Analyzed {} crates", self.structures.len());
        Ok(())
    }
    
    fn analyze_crate_structure(&self, crate_name: &str, sample_path: &str, structure: &mut CodeStructure) -> Result<(), Box<dyn std::error::Error>> {
        // Find all Rust files for this crate
        let crate_dir = sample_path.split("/src/").next().unwrap_or(sample_path);
        
        let output = Command::new("find")
            .args(&[crate_dir, "-name", "*.rs", "-type", "f"])
            .output()?;
        
        if !output.status.success() {
            return Ok(());
        }
        
        let crate_files = String::from_utf8_lossy(&output.stdout);
        
        for rs_file in crate_files.lines() {
            if let Ok(content) = fs::read_to_string(rs_file) {
                // Identify lib files
                if rs_file.ends_with("lib.rs") || rs_file.ends_with("main.rs") {
                    structure.libs.push(rs_file.to_string());
                }
                
                // Extract declarations, exports, and signatures
                self.extract_code_elements(&content, structure);
            }
        }
        
        Ok(())
    }
    
    fn extract_code_elements(&self, content: &str, structure: &mut CodeStructure) {
        for line in content.lines() {
            let line = line.trim();
            
            // Skip comments and empty lines
            if line.starts_with("//") || line.is_empty() {
                continue;
            }
            
            // Extract declarations
            if line.starts_with("fn ") || line.contains(" fn ") {
                structure.decls.push(format!("fn: {}", self.extract_fn_name(line)));
                
                // Extract signature
                if let Some(sig) = self.extract_fn_signature(line) {
                    structure.signatures.push(sig);
                }
            }
            
            if line.starts_with("struct ") || line.contains(" struct ") {
                structure.decls.push(format!("struct: {}", self.extract_type_name(line, "struct")));
            }
            
            if line.starts_with("enum ") || line.contains(" enum ") {
                structure.decls.push(format!("enum: {}", self.extract_type_name(line, "enum")));
            }
            
            if line.starts_with("trait ") || line.contains(" trait ") {
                structure.decls.push(format!("trait: {}", self.extract_type_name(line, "trait")));
            }
            
            // Extract exports (pub items)
            if line.starts_with("pub ") {
                structure.exports.push(self.extract_pub_item(line));
            }
        }
    }
    
    fn extract_fn_name(&self, line: &str) -> String {
        if let Some(start) = line.find("fn ") {
            let after_fn = &line[start + 3..];
            if let Some(end) = after_fn.find('(') {
                return after_fn[..end].trim().to_string();
            }
        }
        "unknown".to_string()
    }
    
    fn extract_fn_signature(&self, line: &str) -> Option<String> {
        if let Some(start) = line.find("fn ") {
            if let Some(end) = line.find('{') {
                return Some(line[start..end].trim().to_string());
            } else if line.ends_with(';') {
                return Some(line[start..].trim_end_matches(';').trim().to_string());
            }
        }
        None
    }
    
    fn extract_type_name(&self, line: &str, type_keyword: &str) -> String {
        if let Some(start) = line.find(&format!("{} ", type_keyword)) {
            let after_keyword = &line[start + type_keyword.len() + 1..];
            if let Some(end) = after_keyword.find(|c: char| c.is_whitespace() || c == '<' || c == '{') {
                return after_keyword[..end].trim().to_string();
            }
        }
        "unknown".to_string()
    }
    
    fn extract_pub_item(&self, line: &str) -> String {
        if line.starts_with("pub fn ") {
            format!("pub fn {}", self.extract_fn_name(line))
        } else if line.starts_with("pub struct ") {
            format!("pub struct {}", self.extract_type_name(line, "struct"))
        } else if line.starts_with("pub enum ") {
            format!("pub enum {}", self.extract_type_name(line, "enum"))
        } else if line.starts_with("pub trait ") {
            format!("pub trait {}", self.extract_type_name(line, "trait"))
        } else {
            line.to_string()
        }
    }
    
    fn extract_crate_from_path(&self, path: &str) -> String {
        if let Some(submodules_pos) = path.find("submodules/") {
            let after_submodules = &path[submodules_pos + 11..];
            if let Some(slash_pos) = after_submodules.find('/') {
                return after_submodules[..slash_pos].to_string();
            }
        }
        "unknown".to_string()
    }
    
    fn get_git_object(&self, crate_name: &str) -> String {
        let submodule_path = format!("submodules/{}", crate_name);
        
        let output = Command::new("git")
            .args(&["submodule", "status", &submodule_path])
            .current_dir("..")
            .output();
        
        if let Ok(result) = output {
            if result.status.success() {
                let status_line = String::from_utf8_lossy(&result.stdout);
                if let Some(hash) = status_line.split_whitespace().next() {
                    return hash.trim_start_matches(' ').to_string();
                }
            }
        }
        
        "unknown".to_string()
    }
    
    fn generate_structure_report(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📊 Generating code structure report...");
        
        let mut report = String::new();
        report.push_str("# Code Structure Analysis Report\n\n");
        
        report.push_str("## Extended Pipeline Flow\n");
        report.push_str("rustc → crate → cargo metadata → repo → forks → branches → submodules → cargo.toml → package name → git object → **libs → decls → exports → signatures** → database entry → deps\n\n");
        
        report.push_str("## Code Structure Analysis\n\n");
        
        for (crate_name, structure) in &self.structures {
            report.push_str(&format!("### {}\n", crate_name));
            report.push_str(&format!("- **Git Object**: `{}`\n", structure.git_object));
            report.push_str(&format!("- **Libs**: {} files\n", structure.libs.len()));
            
            for lib in &structure.libs {
                let short_path = lib.replace("../submodules/", "");
                report.push_str(&format!("  - `{}`\n", short_path));
            }
            
            report.push_str(&format!("- **Declarations**: {}\n", structure.decls.len()));
            for (i, decl) in structure.decls.iter().enumerate() {
                if i < 5 { // Show first 5
                    report.push_str(&format!("  - `{}`\n", decl));
                }
            }
            if structure.decls.len() > 5 {
                report.push_str(&format!("  - ... and {} more\n", structure.decls.len() - 5));
            }
            
            report.push_str(&format!("- **Exports**: {}\n", structure.exports.len()));
            for (i, export) in structure.exports.iter().enumerate() {
                if i < 3 { // Show first 3
                    report.push_str(&format!("  - `{}`\n", export));
                }
            }
            if structure.exports.len() > 3 {
                report.push_str(&format!("  - ... and {} more\n", structure.exports.len() - 3));
            }
            
            report.push_str(&format!("- **Signatures**: {}\n", structure.signatures.len()));
            for (i, sig) in structure.signatures.iter().enumerate() {
                if i < 3 { // Show first 3
                    report.push_str(&format!("  - `{}`\n", sig));
                }
            }
            if structure.signatures.len() > 3 {
                report.push_str(&format!("  - ... and {} more\n", structure.signatures.len() - 3));
            }
            
            report.push_str("\n");
        }
        
        // Summary statistics
        let total_libs = self.structures.values().map(|s| s.libs.len()).sum::<usize>();
        let total_decls = self.structures.values().map(|s| s.decls.len()).sum::<usize>();
        let total_exports = self.structures.values().map(|s| s.exports.len()).sum::<usize>();
        let total_signatures = self.structures.values().map(|s| s.signatures.len()).sum::<usize>();
        
        report.push_str("## Code Structure Statistics\n");
        report.push_str(&format!("- Analyzed crates: {}\n", self.structures.len()));
        report.push_str(&format!("- Total lib files: {}\n", total_libs));
        report.push_str(&format!("- Total declarations: {}\n", total_decls));
        report.push_str(&format!("- Total exports: {}\n", total_exports));
        report.push_str(&format!("- Total signatures: {}\n", total_signatures));
        
        report.push_str("\n## RocksDB Schema\n");
        report.push_str("```\n");
        report.push_str("Key: crate_name:git_object\n");
        report.push_str("Value: {\n");
        report.push_str("  libs: [lib_paths],\n");
        report.push_str("  decls: [declarations],\n");
        report.push_str("  exports: [public_items],\n");
        report.push_str("  signatures: [function_signatures],\n");
        report.push_str("  dependencies: [dep_list]\n");
        report.push_str("}\n");
        report.push_str("```\n");
        
        fs::write("CODE_STRUCTURE_ANALYSIS.md", &report)?;
        
        println!("  ✓ Report written to CODE_STRUCTURE_ANALYSIS.md");
        Ok(())
    }
    
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🎯 Code Structure Analyzer - libs, decls, exports, signatures");
        
        self.analyze_submodule_code()?;
        self.generate_structure_report()?;
        
        let total_decls = self.structures.values().map(|s| s.decls.len()).sum::<usize>();
        let total_exports = self.structures.values().map(|s| s.exports.len()).sum::<usize>();
        let total_signatures = self.structures.values().map(|s| s.signatures.len()).sum::<usize>();
        
        println!("\n🎯 === CODE STRUCTURE ANALYSIS COMPLETE ===");
        println!("  Crates analyzed: {}", self.structures.len());
        println!("  Declarations: {}", total_decls);
        println!("  Exports: {}", total_exports);
        println!("  Signatures: {}", total_signatures);
        
        println!("\n🗄️ READY FOR ROCKSDB STORAGE!");
        
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut analyzer = CodeStructureAnalyzer::new();
    analyzer.run()
}
