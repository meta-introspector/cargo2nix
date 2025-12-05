use std::fs;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct SameNameGroup {
    name: String,
    modules: Vec<ModuleInfo>,
}

#[derive(Debug, Clone)]
struct ModuleInfo {
    submodule_path: String,
    url: String,
    git_object: String,
}

struct SameNameMapper {
    name_groups: HashMap<String, SameNameGroup>,
}

impl SameNameMapper {
    fn new() -> Self {
        Self {
            name_groups: HashMap::new(),
        }
    }
    
    fn parse_gitmodules(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📋 Parsing .gitmodules for same-name relationships...");
        
        let gitmodules = fs::read_to_string("../.gitmodules")?;
        let mut current_path = String::new();
        let mut current_url = String::new();
        
        for line in gitmodules.lines() {
            let line = line.trim();
            
            if line.starts_with("[submodule") {
                // Process previous entry
                if !current_path.is_empty() && !current_url.is_empty() {
                    self.add_module(&current_path, &current_url);
                }
                current_path.clear();
                current_url.clear();
            } else if line.starts_with("path = ") {
                current_path = line.replace("path = ", "");
            } else if line.starts_with("url = ") {
                current_url = line.replace("url = ", "");
            }
        }
        
        // Don't forget the last entry
        if !current_path.is_empty() && !current_url.is_empty() {
            self.add_module(&current_path, &current_url);
        }
        
        println!("  ✓ Processed {} git modules", self.count_total_modules());
        Ok(())
    }
    
    fn add_module(&mut self, path: &str, url: &str) {
        let name = self.extract_repo_name(url);
        let git_object = self.get_git_object(path);
        
        let module_info = ModuleInfo {
            submodule_path: path.to_string(),
            url: url.to_string(),
            git_object,
        };
        
        self.name_groups
            .entry(name.clone())
            .or_insert_with(|| SameNameGroup {
                name: name.clone(),
                modules: Vec::new(),
            })
            .modules
            .push(module_info);
    }
    
    fn extract_repo_name(&self, url: &str) -> String {
        if let Some(repo_name) = url.split('/').last() {
            repo_name.replace(".git", "")
        } else {
            "unknown".to_string()
        }
    }
    
    fn get_git_object(&self, submodule_path: &str) -> String {
        use std::process::Command;
        
        let output = Command::new("git")
            .args(&["submodule", "status", submodule_path])
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
    
    fn count_total_modules(&self) -> usize {
        self.name_groups.values().map(|g| g.modules.len()).sum()
    }
    
    fn find_same_name_groups(&self) -> Vec<&SameNameGroup> {
        self.name_groups.values()
            .filter(|group| group.modules.len() > 1)
            .collect()
    }
    
    fn generate_same_name_report(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📊 Generating same-name relationship report...");
        
        let same_name_groups = self.find_same_name_groups();
        
        let mut report = String::new();
        report.push_str("# Same Name Relationship Mapping Report\n\n");
        
        report.push_str("## Same Name Relationship Flow\n");
        report.push_str("git module - same name as - git module\n\n");
        
        report.push_str("## Same Name Groups\n\n");
        
        // Sort by number of modules with same name
        let mut sorted_groups = same_name_groups;
        sorted_groups.sort_by(|a, b| b.modules.len().cmp(&a.modules.len()));
        
        for group in &sorted_groups {
            report.push_str(&format!("### {} ({} modules)\n", group.name, group.modules.len()));
            
            // Show all modules with this name
            for (i, module) in group.modules.iter().enumerate() {
                report.push_str(&format!("#### Module {} - {}\n", i + 1, module.submodule_path.replace("submodules/", "")));
                report.push_str(&format!("- **Path**: `{}`\n", module.submodule_path));
                report.push_str(&format!("- **URL**: `{}`\n", module.url));
                report.push_str(&format!("- **Git Object**: `{}`\n", module.git_object));
                report.push_str("\n");
            }
            
            // Show same-name relationships
            report.push_str("**Same Name Relationships**:\n");
            for i in 0..group.modules.len() {
                for j in (i + 1)..group.modules.len() {
                    let module_a = &group.modules[i];
                    let module_b = &group.modules[j];
                    
                    report.push_str(&format!("- `{}` - **same name as** - `{}`\n", 
                        module_a.submodule_path.replace("submodules/", ""),
                        module_b.submodule_path.replace("submodules/", "")));
                }
            }
            
            report.push_str("\n");
        }
        
        // Unique names (no collisions)
        let unique_names: Vec<_> = self.name_groups.values()
            .filter(|group| group.modules.len() == 1)
            .collect();
        
        report.push_str("## Statistics\n");
        report.push_str(&format!("- Total git modules: {}\n", self.count_total_modules()));
        report.push_str(&format!("- Unique names: {}\n", unique_names.len()));
        report.push_str(&format!("- Name collisions: {}\n", sorted_groups.len()));
        report.push_str(&format!("- Total name groups: {}\n", self.name_groups.len()));
        
        if sorted_groups.len() > 0 {
            let total_collision_modules: usize = sorted_groups.iter()
                .map(|g| g.modules.len())
                .sum();
            
            report.push_str(&format!("- Modules with name collisions: {}\n", total_collision_modules));
            
            let avg_collision_size = total_collision_modules as f64 / sorted_groups.len() as f64;
            report.push_str(&format!("- Average collision group size: {:.1}\n", avg_collision_size));
        }
        
        // Top collision groups
        report.push_str("\n## Top Name Collision Groups\n");
        for (i, group) in sorted_groups.iter().enumerate() {
            if i < 10 {
                report.push_str(&format!("{}. **{}** - {} modules\n", i + 1, group.name, group.modules.len()));
            }
        }
        
        report.push_str("\n## RocksDB Same Name Schema\n");
        report.push_str("```\n");
        report.push_str("Key: repo_name\n");
        report.push_str("Value: {\n");
        report.push_str("  name: string,\n");
        report.push_str("  modules: [{\n");
        report.push_str("    submodule_path: string,\n");
        report.push_str("    url: string,\n");
        report.push_str("    git_object: string\n");
        report.push_str("  }],\n");
        report.push_str("  relationship: \"same_name_as\"\n");
        report.push_str("}\n");
        report.push_str("```\n");
        
        fs::write("SAME_NAME_MAPPING.md", &report)?;
        
        println!("  ✓ Report written to SAME_NAME_MAPPING.md");
        Ok(())
    }
    
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🎯 Same Name Mapper - git module - same name as - git module");
        
        self.parse_gitmodules()?;
        self.generate_same_name_report()?;
        
        let same_name_groups = self.find_same_name_groups();
        let total_collision_modules: usize = same_name_groups.iter()
            .map(|g| g.modules.len())
            .sum();
        
        println!("\n🎯 === SAME NAME MAPPING COMPLETE ===");
        println!("  Total git modules: {}", self.count_total_modules());
        println!("  Name collision groups: {}", same_name_groups.len());
        println!("  Modules with collisions: {}", total_collision_modules);
        
        if same_name_groups.len() > 0 {
            let avg_size = total_collision_modules as f64 / same_name_groups.len() as f64;
            println!("  Average collision size: {:.1}", avg_size);
        }
        
        println!("\n🔗 SAME NAME RELATIONSHIP: git module - same name as - git module");
        
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut mapper = SameNameMapper::new();
    mapper.run()
}
