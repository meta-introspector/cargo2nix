use std::fs;
use std::process::Command;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct CargoDependency {
    name: String,
    version: Option<String>,
    source: Option<String>, // git, crates.io, path, etc.
}

#[derive(Debug, Clone)]
struct CargoModule {
    name: String,
    cargo_toml_path: String,
    git_module: String,
    dependencies: Vec<CargoDependency>,
    used_by: Vec<String>, // modules that depend on this one
}

struct CargoDependencyMapper {
    modules: HashMap<String, CargoModule>,
}

impl CargoDependencyMapper {
    fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }
    
    fn scan_cargo_modules(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📦 Scanning cargo modules and their dependencies...");
        
        let output = Command::new("find")
            .args(&["../submodules", "-name", "Cargo.toml", "-type", "f"])
            .output()?;
        
        if !output.status.success() {
            return Ok(());
        }
        
        let cargo_files = String::from_utf8_lossy(&output.stdout);
        
        for cargo_toml_path in cargo_files.lines() {
            if let Ok(content) = fs::read_to_string(cargo_toml_path) {
                if let Some(module) = self.parse_cargo_module(cargo_toml_path, &content) {
                    self.modules.insert(module.name.clone(), module);
                }
            }
        }
        
        println!("  ✓ Found {} cargo modules", self.modules.len());
        Ok(())
    }
    
    fn parse_cargo_module(&self, cargo_toml_path: &str, content: &str) -> Option<CargoModule> {
        let mut name = None;
        let mut dependencies = Vec::new();
        let mut in_deps_section = false;
        
        for line in content.lines() {
            let line = line.trim();
            
            // Extract package name
            if line.starts_with("name = ") {
                name = line.split('"').nth(1).map(|s| s.to_string());
            }
            
            // Track dependency sections
            if line.starts_with("[dependencies") || line.starts_with("[dev-dependencies") || line.starts_with("[build-dependencies") {
                in_deps_section = true;
                continue;
            }
            
            if line.starts_with('[') && in_deps_section {
                in_deps_section = false;
            }
            
            // Parse dependencies
            if in_deps_section && line.contains('=') && !line.starts_with('#') {
                if let Some(dep) = self.parse_dependency_line(line) {
                    dependencies.push(dep);
                }
            }
        }
        
        if let Some(module_name) = name {
            let git_module = self.extract_git_module(cargo_toml_path);
            
            Some(CargoModule {
                name: module_name,
                cargo_toml_path: cargo_toml_path.to_string(),
                git_module,
                dependencies,
                used_by: Vec::new(),
            })
        } else {
            None
        }
    }
    
    fn parse_dependency_line(&self, line: &str) -> Option<CargoDependency> {
        if let Some(eq_pos) = line.find('=') {
            let dep_name = line[..eq_pos].trim().replace('"', "");
            let dep_value = line[eq_pos + 1..].trim();
            
            if dep_name.is_empty() {
                return None;
            }
            
            // Simple version string
            if dep_value.starts_with('"') && dep_value.ends_with('"') {
                let version = dep_value.trim_matches('"').to_string();
                return Some(CargoDependency {
                    name: dep_name,
                    version: Some(version),
                    source: Some("crates.io".to_string()),
                });
            }
            
            // Complex dependency (git, path, etc.)
            if dep_value.starts_with('{') {
                let mut version = None;
                let mut source = None;
                
                if dep_value.contains("git =") {
                    source = Some("git".to_string());
                }
                if dep_value.contains("path =") {
                    source = Some("path".to_string());
                }
                if let Some(ver_start) = dep_value.find("version = \"") {
                    let ver_start = ver_start + 11;
                    if let Some(ver_end) = dep_value[ver_start..].find('"') {
                        version = Some(dep_value[ver_start..ver_start + ver_end].to_string());
                    }
                }
                
                return Some(CargoDependency {
                    name: dep_name,
                    version,
                    source,
                });
            }
        }
        
        None
    }
    
    fn extract_git_module(&self, cargo_toml_path: &str) -> String {
        // Extract git module from path like "../submodules/juniper/Cargo.toml"
        if let Some(submodules_pos) = cargo_toml_path.find("submodules/") {
            let after_submodules = &cargo_toml_path[submodules_pos + 11..];
            if let Some(slash_pos) = after_submodules.find('/') {
                return after_submodules[..slash_pos].to_string();
            }
        }
        "unknown".to_string()
    }
    
    fn build_usage_relationships(&mut self) {
        println!("🔗 Building usage relationships...");
        
        // Build reverse dependency map
        let mut usage_map: HashMap<String, Vec<String>> = HashMap::new();
        
        for (module_name, module) in &self.modules {
            for dep in &module.dependencies {
                usage_map
                    .entry(dep.name.clone())
                    .or_insert_with(Vec::new)
                    .push(module_name.clone());
            }
        }
        
        // Update modules with usage information
        for (dep_name, users) in usage_map {
            if let Some(module) = self.modules.get_mut(&dep_name) {
                module.used_by = users;
            }
        }
        
        println!("  ✓ Built usage relationships");
    }
    
    fn generate_dependency_report(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📊 Generating cargo dependency mapping report...");
        
        let mut report = String::new();
        report.push_str("# Cargo Module Dependency Mapping Report\n\n");
        
        report.push_str("## Dependency Relationship Flow\n");
        report.push_str("cargo module < uses <- cargo module\n\n");
        
        report.push_str("## Cargo Module Dependencies\n\n");
        
        // Sort by number of dependencies (most dependent first)
        let mut sorted_modules: Vec<_> = self.modules.iter().collect();
        sorted_modules.sort_by(|a, b| b.1.dependencies.len().cmp(&a.1.dependencies.len()));
        
        for (module_name, module) in &sorted_modules {
            if module.dependencies.len() > 0 || module.used_by.len() > 0 {
                report.push_str(&format!("### {} ({})\n", module_name, module.git_module));
                report.push_str(&format!("- **Git Module**: `{}`\n", module.git_module));
                
                let short_path = module.cargo_toml_path.replace("../", "");
                report.push_str(&format!("- **Cargo.toml**: `{}`\n", short_path));
                
                if !module.dependencies.is_empty() {
                    report.push_str(&format!("- **Uses {} dependencies**:\n", module.dependencies.len()));
                    for (i, dep) in module.dependencies.iter().enumerate() {
                        if i < 10 { // Show first 10
                            let version_info = dep.version.as_ref()
                                .map(|v| format!(" ({})", v))
                                .unwrap_or_default();
                            
                            let source_info = dep.source.as_ref()
                                .map(|s| format!(" [{}]", s))
                                .unwrap_or_default();
                            
                            report.push_str(&format!("  - `{}` < **uses** <- `{}`{}{}\n", 
                                dep.name, module_name, version_info, source_info));
                        }
                    }
                    if module.dependencies.len() > 10 {
                        report.push_str(&format!("  - ... and {} more dependencies\n", module.dependencies.len() - 10));
                    }
                }
                
                if !module.used_by.is_empty() {
                    report.push_str(&format!("- **Used by {} modules**:\n", module.used_by.len()));
                    for (i, user) in module.used_by.iter().enumerate() {
                        if i < 5 { // Show first 5
                            report.push_str(&format!("  - `{}` < **uses** <- `{}`\n", module_name, user));
                        }
                    }
                    if module.used_by.len() > 5 {
                        report.push_str(&format!("  - ... and {} more users\n", module.used_by.len() - 5));
                    }
                }
                
                report.push_str("\n");
            }
        }
        
        // Statistics
        let total_deps: usize = self.modules.values().map(|m| m.dependencies.len()).sum();
        let modules_with_deps = self.modules.values().filter(|m| !m.dependencies.is_empty()).count();
        let modules_used_by_others = self.modules.values().filter(|m| !m.used_by.is_empty()).count();
        
        report.push_str("## Dependency Statistics\n");
        report.push_str(&format!("- Total cargo modules: {}\n", self.modules.len()));
        report.push_str(&format!("- Modules with dependencies: {}\n", modules_with_deps));
        report.push_str(&format!("- Modules used by others: {}\n", modules_used_by_others));
        report.push_str(&format!("- Total dependency relationships: {}\n", total_deps));
        
        if modules_with_deps > 0 {
            let avg_deps = total_deps as f64 / modules_with_deps as f64;
            report.push_str(&format!("- Average dependencies per module: {:.1}\n", avg_deps));
        }
        
        // Top modules by dependency count
        report.push_str("\n## Most Dependent Modules\n");
        for (i, (module_name, module)) in sorted_modules.iter().enumerate() {
            if i < 10 && module.dependencies.len() > 0 {
                report.push_str(&format!("{}. **{}** - {} dependencies\n", 
                    i + 1, module_name, module.dependencies.len()));
            }
        }
        
        // Most used modules
        let mut by_usage: Vec<_> = self.modules.iter().collect();
        by_usage.sort_by(|a, b| b.1.used_by.len().cmp(&a.1.used_by.len()));
        
        report.push_str("\n## Most Used Modules\n");
        for (i, (module_name, module)) in by_usage.iter().enumerate() {
            if i < 10 && module.used_by.len() > 0 {
                report.push_str(&format!("{}. **{}** - used by {} modules\n", 
                    i + 1, module_name, module.used_by.len()));
            }
        }
        
        fs::write("CARGO_DEPENDENCY_MAPPING.md", &report)?;
        
        println!("  ✓ Report written to CARGO_DEPENDENCY_MAPPING.md");
        Ok(())
    }
    
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🎯 Cargo Dependency Mapper - cargo module < uses <- cargo module");
        
        self.scan_cargo_modules()?;
        self.build_usage_relationships();
        self.generate_dependency_report()?;
        
        let total_deps: usize = self.modules.values().map(|m| m.dependencies.len()).sum();
        let modules_with_deps = self.modules.values().filter(|m| !m.dependencies.is_empty()).count();
        let modules_used = self.modules.values().filter(|m| !m.used_by.is_empty()).count();
        
        println!("\n🎯 === CARGO DEPENDENCY MAPPING COMPLETE ===");
        println!("  Total cargo modules: {}", self.modules.len());
        println!("  Modules with dependencies: {}", modules_with_deps);
        println!("  Modules used by others: {}", modules_used);
        println!("  Total dependency relationships: {}", total_deps);
        
        println!("\n🔗 USAGE RELATIONSHIP: cargo module < uses <- cargo module");
        
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut mapper = CargoDependencyMapper::new();
    mapper.run()
}
