use std::process::Command;
use std::fs;

struct RemoveDuplicateSubmodules {
    to_remove: Vec<String>,
    removed_count: u32,
}

impl RemoveDuplicateSubmodules {
    fn new() -> Self {
        Self {
            to_remove: vec![
                "blake3".to_string(),
                "annotate-snippets".to_string(), 
                "boml".to_string(),
                "cranelift-codegen".to_string(),
                "cranelift-frontend".to_string(),
                "cranelift-jit".to_string(),
                "cranelift-module".to_string(),
                "rust".to_string(), // Any rust.git duplicates
            ],
            removed_count: 0,
        }
    }
    
    fn remove_submodules(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🗑️ Removing duplicate/unnecessary submodules...");
        
        for submodule in &self.to_remove {
            if self.remove_single_submodule(submodule)? {
                self.removed_count += 1;
                println!("  ✓ Removed {}", submodule);
            } else {
                println!("  - {} not found or already removed", submodule);
            }
        }
        
        Ok(())
    }
    
    fn remove_single_submodule(&self, name: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let submodule_path = format!("../submodules/{}", name);
        
        // Check if submodule exists
        if !std::path::Path::new(&submodule_path).exists() {
            return Ok(false);
        }
        
        // Remove from git submodules
        let deinit_output = Command::new("git")
            .args(&["submodule", "deinit", "-f", &submodule_path])
            .current_dir("..")
            .output()?;
        
        // Remove from .git/modules
        let modules_path = format!("../.git/modules/submodules/{}", name);
        if std::path::Path::new(&modules_path).exists() {
            fs::remove_dir_all(&modules_path)?;
        }
        
        // Remove directory
        if std::path::Path::new(&submodule_path).exists() {
            fs::remove_dir_all(&submodule_path)?;
        }
        
        // Remove from .gitmodules
        self.update_gitmodules(name)?;
        
        Ok(deinit_output.status.success())
    }
    
    fn update_gitmodules(&self, removed_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let gitmodules_path = "../.gitmodules";
        
        if let Ok(content) = fs::read_to_string(gitmodules_path) {
            let mut new_content = String::new();
            let mut skip_section = false;
            
            for line in content.lines() {
                if line.starts_with("[submodule") && line.contains(removed_name) {
                    skip_section = true;
                    continue;
                }
                
                if line.starts_with("[submodule") && skip_section {
                    skip_section = false;
                }
                
                if !skip_section {
                    new_content.push_str(line);
                    new_content.push('\n');
                }
            }
            
            fs::write(gitmodules_path, new_content)?;
        }
        
        Ok(())
    }
    
    fn verify_removal(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔍 Verifying submodule removal...");
        
        let output = Command::new("git")
            .args(&["submodule", "status"])
            .current_dir("..")
            .output()?;
        
        let status = String::from_utf8_lossy(&output.stdout);
        
        for removed in &self.to_remove {
            if status.contains(removed) {
                println!("  ⚠️ {} still appears in git submodule status", removed);
            } else {
                println!("  ✓ {} successfully removed", removed);
            }
        }
        
        Ok(())
    }
    
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🎯 Removing Duplicate Submodules");
        
        self.remove_submodules()?;
        self.verify_removal()?;
        
        println!("\n📊 REMOVAL SUMMARY:");
        println!("  Targeted for removal: {}", self.to_remove.len());
        println!("  Successfully removed: {}", self.removed_count);
        
        println!("\n✅ CLEANUP COMPLETE:");
        println!("  • Removed duplicate rust.git references");
        println!("  • Removed external crates we'll add properly");
        println!("  • Ready for smart Monster Protocol build");
        
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut remover = RemoveDuplicateSubmodules::new();
    remover.run()
}
