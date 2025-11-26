use std::process::Command;

#[derive(Debug)]
struct CargoProject {
    name: String,
    path: String,
    git_hash: String,
    criticality: i32,
}

struct Database;

impl Database {
    fn find_all_cargo_locks(&self) -> Vec<CargoProject> {
        let mut projects = Vec::new();
        let solana_root = "/home/mdupont/nix/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src";
        
        // Find all Cargo.lock files
        if let Ok(output) = Command::new("find")
            .args(&[solana_root, "-name", "Cargo.lock"])
            .output() {
            
            let paths = String::from_utf8_lossy(&output.stdout);
            
            for lock_path in paths.lines() {
                let dir_path = lock_path.replace("/Cargo.lock", "");
                let name = self.extract_project_name(&dir_path);
                let criticality = self.calculate_criticality(&name);
                
                projects.push(CargoProject {
                    name,
                    path: dir_path,
                    git_hash: self.get_git_hash(solana_root),
                    criticality,
                });
            }
        }
        
        projects.sort_by(|a, b| b.criticality.cmp(&a.criticality));
        projects
    }
    
    fn extract_project_name(&self, path: &str) -> String {
        if let Some(name) = path.split('/').last() {
            name.to_string()
        } else {
            "unknown".to_string()
        }
    }
    
    fn calculate_criticality(&self, name: &str) -> i32 {
        match name {
            "rustc" | "compiler" => 9,
            n if n.contains("rustc") => 8,
            "library" | "std" | "core" | "alloc" => 7,
            "cargo" => 6,
            _ => 1,
        }
    }
    
    fn get_git_hash(&self, path: &str) -> String {
        if let Ok(output) = Command::new("git")
            .args(&["rev-parse", "--short", "HEAD"])
            .current_dir(path)
            .output() {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        } else {
            "no-git".to_string()
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Solana Rustc Cargo.lock Analysis ===");
    
    let db = Database;
    let projects = db.find_all_cargo_locks();
    
    println!("query SolanaRustcCargoProjects {{");
    println!("  cargoProjects(orderBy: CRITICALITY_DESC) {{");
    
    for (i, project) in projects.iter().enumerate() {
        println!("    {}. [{}] {} | {} | git:{}", 
            i + 1,
            project.criticality,
            project.name,
            project.path,
            project.git_hash
        );
    }
    
    println!("  }}");
    println!("}}");
    println!("\nTotal Cargo projects found: {}", projects.len());
    
    Ok(())
}
