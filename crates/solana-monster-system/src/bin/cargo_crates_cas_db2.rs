use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
struct CargoCrate {
    name: String,
    version: String,
    cargo_toml_path: String,
    dependencies: Vec<String>,
    features: Vec<String>,
    cas_address: u64,
}

struct CargoCratesDB2 {
    crates: HashMap<u64, CargoCrate>,
    name_to_cas: HashMap<String, u64>,
}

impl CargoCratesDB2 {
    fn new() -> Self {
        Self {
            crates: HashMap::new(),
            name_to_cas: HashMap::new(),
        }
    }

    fn load_all_cargo_crates(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("=== Loading Cargo Crates into DB2 ===");

        // Find all Cargo.toml files
        self.scan_directory("./submodules")?;
        self.scan_directory("./minizinc-introspector")?;
        self.scan_directory("./tools")?;

        Ok(())
    }

    fn scan_directory(&mut self, dir: &str) -> Result<(), Box<dyn std::error::Error>> {
        if !Path::new(dir).exists() {
            return Ok(());
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let cargo_toml = path.join("Cargo.toml");
                if cargo_toml.exists() {
                    self.process_cargo_toml(&cargo_toml)?;
                }

                // Recurse into subdirectories
                if let Some(path_str) = path.to_str() {
                    self.scan_directory(path_str)?;
                }
            }
        }

        Ok(())
    }

    fn process_cargo_toml(&mut self, cargo_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(cargo_path)?;
        let path_str = cargo_path.to_string_lossy().to_string();

        let name = self.extract_name(&content);
        let version = self.extract_version(&content);
        let dependencies = self.extract_dependencies(&content);
        let features = self.extract_features(&content);

        let cas_address = self.calculate_crate_cas(&name, &version, &dependencies, &features);

        let crate_info = CargoCrate {
            name: name.clone(),
            version,
            cargo_toml_path: path_str,
            dependencies: dependencies.clone(),
            features,
            cas_address,
        };

        println!(
            "CRATE: {} → CAS: {} ({} deps)",
            name,
            cas_address,
            dependencies.len()
        );

        self.crates.insert(cas_address, crate_info);
        self.name_to_cas.insert(name, cas_address);

        Ok(())
    }

    fn extract_name(&self, content: &str) -> String {
        for line in content.lines() {
            if line.trim().starts_with("name = ") {
                if let Some(start) = line.find('"') {
                    if let Some(end) = line[start + 1..].find('"') {
                        return line[start + 1..start + 1 + end].to_string();
                    }
                }
            }
        }
        "unknown".to_string()
    }

    fn extract_version(&self, content: &str) -> String {
        for line in content.lines() {
            if line.trim().starts_with("version = ") {
                if let Some(start) = line.find('"') {
                    if let Some(end) = line[start + 1..].find('"') {
                        return line[start + 1..start + 1 + end].to_string();
                    }
                }
            }
        }
        "0.1.0".to_string()
    }

    fn extract_dependencies(&self, content: &str) -> Vec<String> {
        let mut deps = Vec::new();
        let mut in_deps = false;

        for line in content.lines() {
            let trimmed = line.trim();

            if trimmed == "[dependencies]" {
                in_deps = true;
                continue;
            }

            if trimmed.starts_with('[') && trimmed != "[dependencies]" {
                in_deps = false;
            }

            if in_deps && trimmed.contains('=') {
                if let Some(dep_name) = trimmed.split('=').next() {
                    let clean = dep_name.trim().trim_matches('"');
                    if !clean.is_empty() {
                        deps.push(clean.to_string());
                    }
                }
            }
        }

        deps
    }

    fn extract_features(&self, content: &str) -> Vec<String> {
        let mut features = Vec::new();
        let mut in_features = false;

        for line in content.lines() {
            let trimmed = line.trim();

            if trimmed == "[features]" {
                in_features = true;
                continue;
            }

            if trimmed.starts_with('[') && trimmed != "[features]" {
                in_features = false;
            }

            if in_features && trimmed.contains('=') {
                if let Some(feature_name) = trimmed.split('=').next() {
                    let clean = feature_name.trim().trim_matches('"');
                    if !clean.is_empty() {
                        features.push(clean.to_string());
                    }
                }
            }
        }

        features
    }

    fn calculate_crate_cas(
        &self,
        name: &str,
        version: &str,
        deps: &[String],
        features: &[String],
    ) -> u64 {
        let mut signature = calculate_phi_key(name);
        signature = signature.wrapping_add(calculate_phi_key(version));

        for dep in deps {
            signature = signature.wrapping_add(calculate_phi_key(dep));
        }

        for feature in features {
            signature = signature.wrapping_add(calculate_phi_key(feature));
        }

        signature % 196883
    }

    fn show_database(&self) {
        println!("\n=== Cargo Crates Database (DB2) ===");
        println!("Total crates: {}", self.crates.len());

        for (cas, crate_info) in &self.crates {
            println!(
                "📦 {} v{} (CAS: {})",
                crate_info.name, crate_info.version, cas
            );
            println!("   Path: {}", crate_info.cargo_toml_path);
            if !crate_info.dependencies.is_empty() {
                println!("   Deps: {:?}", crate_info.dependencies);
            }
            if !crate_info.features.is_empty() {
                println!("   Features: {:?}", crate_info.features);
            }
        }
    }
}

fn calculate_phi_key(name: &str) -> u64 {
    let name_hash = name.bytes().map(|b| b as u64).sum::<u64>();
    let monster_element = (name_hash * 5 + 71) % 196883;
    euler_phi(monster_element)
}

fn euler_phi(n: u64) -> u64 {
    if n <= 1 {
        return 1;
    }
    let mut result = n;
    let mut num = n;
    let mut p = 2;

    while p * p <= num {
        if num % p == 0 {
            while num % p == 0 {
                num /= p;
            }
            result -= result / p;
        }
        p += 1;
    }
    if num > 1 {
        result -= result / num;
    }
    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Cargo Crates CAS Database 2 ===");

    let mut db2 = CargoCratesDB2::new();
    db2.load_all_cargo_crates()?;
    db2.show_database();

    println!("\n✓ All Cargo.toml files loaded into DB2");
    println!("✓ CAS addresses calculated from crate metadata");

    Ok(())
}
