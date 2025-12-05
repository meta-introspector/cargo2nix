use std::fs;
use std::path::Path;
use std::process::Command;
use std::collections::{HashMap, HashSet};

/// Solana Submodule Driver - builds rustc using only submodules, no cargo registry
struct SolanaSubmoduleDriver {
    submodules_path: String,
    build_graph: HashMap<String, CrateInfo>,
    nix_build_order: Vec<String>,
}

#[derive(Debug, Clone)]
struct CrateInfo {
    name: String,
    path: String,
    dependencies: Vec<String>,
    nix_expression: String,
    is_rustc_component: bool,
}

impl SolanaSubmoduleDriver {
    pub fn new() -> Self {
        Self {
            submodules_path: "submodules".to_string(),
            build_graph: HashMap::new(),
            nix_build_order: Vec::new(),
        }
    }
    
    /// Main driver function - builds Solana rustc from submodules only
    pub fn build_solana_rustc(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Solana Rustc Submodule-Only Driver");
        println!("===================================");
        
        // 1. Discover all available crates in submodules
        self.discover_submodule_crates()?;
        
        // 2. Build dependency graph from Cargo.toml files
        self.build_dependency_graph()?;
        
        // 3. Generate Nix expressions for each crate
        self.generate_nix_expressions()?;
        
        // 4. Create topological build order
        self.create_build_order()?;
        
        // 5. Generate master Nix build files
        self.generate_master_nix_build()?;
        
        // 6. Execute the build
        self.execute_nix_build()?;
        
        Ok(())
    }
    
    fn discover_submodule_crates(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut crate_count = 0;
        
        if let Ok(entries) = fs::read_dir(&self.submodules_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        let cargo_toml = path.join("Cargo.toml");
                        if cargo_toml.exists() {
                            let name = path.file_name().unwrap().to_string_lossy().to_string();
                            let is_rustc = self.is_rustc_component(&name, &cargo_toml);
                            
                            let crate_info = CrateInfo {
                                name: name.clone(),
                                path: path.to_string_lossy().to_string(),
                                dependencies: Vec::new(),
                                nix_expression: String::new(),
                                is_rustc_component: is_rustc,
                            };
                            
                            self.build_graph.insert(name, crate_info);
                            crate_count += 1;
                        }
                    }
                }
            }
        }
        
        println!("Discovered {} crates in submodules", crate_count);
        Ok(())
    }
    
    fn is_rustc_component(&self, name: &str, cargo_toml: &Path) -> bool {
        // Check if this is a rustc/compiler component
        if name.starts_with("rustc") || name.starts_with("compiler") {
            return true;
        }
        
        // Check Cargo.toml content for rustc indicators
        if let Ok(content) = fs::read_to_string(cargo_toml) {
            return content.contains("rustc") || 
                   content.contains("compiler") ||
                   content.contains("solana");
        }
        
        false
    }
    
    fn build_dependency_graph(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let crate_names: Vec<String> = self.build_graph.keys().cloned().collect();
        
        for (crate_name, crate_info) in self.build_graph.iter_mut() {
            let cargo_toml_path = format!("{}/Cargo.toml", crate_info.path);
            
            if let Ok(content) = fs::read_to_string(&cargo_toml_path) {
                crate_info.dependencies = self.extract_submodule_deps(&content, &crate_names);
            }
        }
        
        println!("Built dependency graph for {} crates", self.build_graph.len());
        Ok(())
    }
    
    fn extract_submodule_deps(&self, cargo_toml: &str, available_crates: &[String]) -> Vec<String> {
        let mut deps = Vec::new();
        let mut in_deps = false;
        
        for line in cargo_toml.lines() {
            let line = line.trim();
            
            if line == "[dependencies]" || line == "[dev-dependencies]" {
                in_deps = true;
            } else if line.starts_with('[') && line != "[dependencies]" && line != "[dev-dependencies]" {
                in_deps = false;
            } else if in_deps && line.contains('=') {
                if let Some(dep_name) = line.split('=').next() {
                    let dep_name = dep_name.trim().trim_matches('"');
                    if available_crates.contains(&dep_name.to_string()) {
                        deps.push(dep_name.to_string());
                    }
                }
            }
        }
        
        deps
    }
    
    fn generate_nix_expressions(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all("nix/crates")?;
        
        for (_, crate_info) in self.build_graph.iter_mut() {
            crate_info.nix_expression = self.create_crate_nix_expression(crate_info)?;
            
            // Write individual nix file
            let nix_file = format!("nix/crates/{}.nix", crate_info.name);
            fs::write(&nix_file, &crate_info.nix_expression)?;
        }
        
        println!("Generated Nix expressions for {} crates", self.build_graph.len());
        Ok(())
    }
    
    fn create_crate_nix_expression(&self, crate_info: &CrateInfo) -> Result<String, Box<dyn std::error::Error>> {
        let deps_list = if crate_info.dependencies.is_empty() {
            "[]".to_string()
        } else {
            format!("[ {} ]", 
                crate_info.dependencies.iter()
                    .map(|d| format!("crates.{}", d.replace('-', "_")))
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        };
        
        let nix_expr = format!(r#"
{{ pkgs, rustPlatform, crates ? {{}} }}:

rustPlatform.buildRustPackage {{
  pname = "{}";
  version = "0.1.0";
  
  src = ../..{};
  
  # Force offline mode - only use submodules
  cargoLock = {{
    lockFile = ../..{}/Cargo.lock;
    allowBuiltinFetchGit = false;
  }};
  
  buildInputs = with pkgs; [
    # System dependencies
    openssl
    pkg-config
  ] ++ {};
  
  # Ensure we only use local submodules
  preBuild = ''
    export CARGO_NET_OFFLINE=true
    export CARGO_HOME=$PWD/.cargo
    mkdir -p .cargo
    cat > .cargo/config.toml << EOF
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "${{src}}/vendor"
EOF
  '';
  
  # Rustc-specific build flags
  {} = with pkgs.lib; {{
    description = "Solana rustc component: {}";
    license = licenses.mit;
    platforms = platforms.unix;
  }};
}}
"#, 
            crate_info.name,
            crate_info.path.strip_prefix("submodules").unwrap_or(&crate_info.path),
            crate_info.path.strip_prefix("submodules").unwrap_or(&crate_info.path),
            deps_list,
            if crate_info.is_rustc_component { "rustcBuildFlags = [\"-C\" \"opt-level=2\"];\n  meta" } else { "meta" },
            crate_info.name
        );
        
        Ok(nix_expr)
    }
    
    fn create_build_order(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut visited = HashSet::new();
        let mut temp_visited = HashSet::new();
        
        for crate_name in self.build_graph.keys() {
            if !visited.contains(crate_name) {
                self.topological_visit(crate_name, &mut visited, &mut temp_visited)?;
            }
        }
        
        self.nix_build_order.reverse();
        println!("Created build order for {} crates", self.nix_build_order.len());
        Ok(())
    }
    
    fn topological_visit(&mut self, crate_name: &str, visited: &mut HashSet<String>, temp_visited: &mut HashSet<String>) -> Result<(), Box<dyn std::error::Error>> {
        if temp_visited.contains(crate_name) {
            return Err(format!("Circular dependency detected involving {}", crate_name).into());
        }
        
        if visited.contains(crate_name) {
            return Ok(());
        }
        
        temp_visited.insert(crate_name.to_string());
        
        if let Some(crate_info) = self.build_graph.get(crate_name) {
            for dep in &crate_info.dependencies {
                self.topological_visit(dep, visited, temp_visited)?;
            }
        }
        
        temp_visited.remove(crate_name);
        visited.insert(crate_name.to_string());
        self.nix_build_order.push(crate_name.to_string());
        
        Ok(())
    }
    
    fn generate_master_nix_build(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut master_nix = String::from(r#"
{ pkgs ? import <nixpkgs> {} }:

let
  rustPlatform = pkgs.rustPlatform;
  
  # All crates built from submodules only
  crates = rec {
"#);
        
        // Add each crate in build order
        for crate_name in &self.nix_build_order {
            let safe_name = crate_name.replace('-', "_");
            master_nix.push_str(&format!(
                "    {} = pkgs.callPackage ./nix/crates/{}.nix {{ inherit rustPlatform crates; }};\n",
                safe_name, crate_name
            ));
        }
        
        master_nix.push_str("  };\n\n");
        
        // Expose rustc components
        master_nix.push_str("  # Rustc components for Solana\n");
        master_nix.push_str("  rustc-components = {\n");
        
        for crate_name in &self.nix_build_order {
            if let Some(crate_info) = self.build_graph.get(crate_name) {
                if crate_info.is_rustc_component {
                    let safe_name = crate_name.replace('-', "_");
                    master_nix.push_str(&format!("    {} = crates.{};\n", safe_name, safe_name));
                }
            }
        }
        
        master_nix.push_str("  };\n\nin {\n");
        master_nix.push_str("  inherit crates rustc-components;\n");
        master_nix.push_str("  \n  # Main Solana rustc build\n");
        master_nix.push_str("  solana-rustc = pkgs.symlinkJoin {\n");
        master_nix.push_str("    name = \"solana-rustc\";\n");
        master_nix.push_str("    paths = builtins.attrValues rustc-components;\n");
        master_nix.push_str("  };\n}\n");
        
        fs::write("solana-rustc.nix", master_nix)?;
        
        // Generate build script
        let build_script = format!(r#"#!/usr/bin/env bash
set -e

echo "Building Solana rustc from submodules only..."
echo "Build order: {} crates"

# Build all crates
nix-build solana-rustc.nix -A solana-rustc

echo "Solana rustc build complete!"
echo "Result: ./result"
"#, self.nix_build_order.len());
        
        fs::write("build-solana-rustc.sh", build_script)?;
        
        // Make executable
        Command::new("chmod").args(&["+x", "build-solana-rustc.sh"]).output()?;
        
        println!("Generated master build files:");
        println!("  - solana-rustc.nix");
        println!("  - build-solana-rustc.sh");
        
        Ok(())
    }
    
    fn execute_nix_build(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Executing Solana rustc build...");
        
        let output = Command::new("bash")
            .arg("build-solana-rustc.sh")
            .output()?;
        
        if output.status.success() {
            println!("Build successful!");
            println!("Solana rustc available at: ./result");
        } else {
            println!("Build failed:");
            println!("{}", String::from_utf8_lossy(&output.stderr));
        }
        
        Ok(())
    }
    
    /// Show build plan without executing
    pub fn show_build_plan(&self) {
        println!("\nBuild Plan:");
        println!("===========");
        
        let rustc_components: Vec<_> = self.build_graph.values()
            .filter(|c| c.is_rustc_component)
            .collect();
        
        println!("Total crates: {}", self.build_graph.len());
        println!("Rustc components: {}", rustc_components.len());
        
        println!("\nBuild order:");
        for (i, crate_name) in self.nix_build_order.iter().enumerate() {
            let is_rustc = self.build_graph.get(crate_name)
                .map(|c| c.is_rustc_component)
                .unwrap_or(false);
            
            println!("  {}. {} {}", 
                i + 1, 
                crate_name,
                if is_rustc { "(rustc)" } else { "" }
            );
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut driver = SolanaSubmoduleDriver::new();
    
    // Build Solana rustc using only submodules
    driver.build_solana_rustc()?;
    
    // Show final build plan
    driver.show_build_plan();
    
    Ok(())
}
