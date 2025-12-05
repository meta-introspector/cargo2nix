use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
struct ASTDeclaration {
    name: String,
    decl_type: String, // "fn", "struct", "enum", "trait", "impl"
    source_file: String,
    line_number: usize,
    content: String,
    dependencies: Vec<String>, // What this decl uses
    cas_address: u64,
}

struct ASTDeclsDB3 {
    declarations: HashMap<u64, ASTDeclaration>,
    name_to_cas: HashMap<String, Vec<u64>>, // name -> multiple CAS addresses
}

impl ASTDeclsDB3 {
    fn new() -> Self {
        Self {
            declarations: HashMap::new(),
            name_to_cas: HashMap::new(),
        }
    }

    fn load_all_ast_declarations(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("=== Loading AST Declarations into DB3 ===");

        // Scan our actual Rust source files
        self.scan_rust_files("./src")?;
        self.scan_rust_files("./tools")?;
        self.scan_rust_files("./submodules/BLAKE3/src")?;
        self.scan_rust_files("./tools/monster_protocol/src")?;

        Ok(())
    }

    fn scan_rust_files(&mut self, dir: &str) -> Result<(), Box<dyn std::error::Error>> {
        if !Path::new(dir).exists() {
            return Ok(());
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
                self.process_rust_file(&path)?;
            } else if path.is_dir() {
                if let Some(path_str) = path.to_str() {
                    self.scan_rust_files(path_str)?;
                }
            }
        }

        Ok(())
    }

    fn process_rust_file(&mut self, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;
        let path_str = file_path.to_string_lossy().to_string();

        println!("Processing: {}", path_str);

        for (line_num, line) in content.lines().enumerate() {
            let trimmed = line.trim();

            // Extract function declarations
            if trimmed.starts_with("fn ") || trimmed.starts_with("pub fn ") {
                self.extract_function_decl(trimmed, &path_str, line_num + 1, &content);
            }

            // Extract struct declarations
            if trimmed.starts_with("struct ") || trimmed.starts_with("pub struct ") {
                self.extract_struct_decl(trimmed, &path_str, line_num + 1, &content);
            }

            // Extract enum declarations
            if trimmed.starts_with("enum ") || trimmed.starts_with("pub enum ") {
                self.extract_enum_decl(trimmed, &path_str, line_num + 1, &content);
            }

            // Extract trait declarations
            if trimmed.starts_with("trait ") || trimmed.starts_with("pub trait ") {
                self.extract_trait_decl(trimmed, &path_str, line_num + 1, &content);
            }
        }

        Ok(())
    }

    fn extract_function_decl(&mut self, line: &str, file: &str, line_num: usize, content: &str) {
        let fn_start = if line.starts_with("pub fn ") { 7 } else { 3 };

        if let Some(paren_pos) = line.find('(') {
            let fn_name = &line[fn_start..paren_pos];
            let dependencies = self.extract_dependencies_from_content(content);
            let cas_address = self.calculate_decl_cas(fn_name, "fn", &dependencies, line);

            let decl = ASTDeclaration {
                name: fn_name.to_string(),
                decl_type: "fn".to_string(),
                source_file: file.to_string(),
                line_number: line_num,
                content: line.to_string(),
                dependencies,
                cas_address,
            };

            self.add_declaration(decl);
        }
    }

    fn extract_struct_decl(&mut self, line: &str, file: &str, line_num: usize, content: &str) {
        let struct_start = if line.starts_with("pub struct ") {
            11
        } else {
            7
        };
        let parts: Vec<&str> = line[struct_start..].split_whitespace().collect();

        if !parts.is_empty() {
            let struct_name = parts[0];
            let dependencies = self.extract_dependencies_from_content(content);
            let cas_address = self.calculate_decl_cas(struct_name, "struct", &dependencies, line);

            let decl = ASTDeclaration {
                name: struct_name.to_string(),
                decl_type: "struct".to_string(),
                source_file: file.to_string(),
                line_number: line_num,
                content: line.to_string(),
                dependencies,
                cas_address,
            };

            self.add_declaration(decl);
        }
    }

    fn extract_enum_decl(&mut self, line: &str, file: &str, line_num: usize, content: &str) {
        let enum_start = if line.starts_with("pub enum ") { 9 } else { 5 };
        let parts: Vec<&str> = line[enum_start..].split_whitespace().collect();

        if !parts.is_empty() {
            let enum_name = parts[0];
            let dependencies = self.extract_dependencies_from_content(content);
            let cas_address = self.calculate_decl_cas(enum_name, "enum", &dependencies, line);

            let decl = ASTDeclaration {
                name: enum_name.to_string(),
                decl_type: "enum".to_string(),
                source_file: file.to_string(),
                line_number: line_num,
                content: line.to_string(),
                dependencies,
                cas_address,
            };

            self.add_declaration(decl);
        }
    }

    fn extract_trait_decl(&mut self, line: &str, file: &str, line_num: usize, content: &str) {
        let trait_start = if line.starts_with("pub trait ") {
            10
        } else {
            6
        };
        let parts: Vec<&str> = line[trait_start..].split_whitespace().collect();

        if !parts.is_empty() {
            let trait_name = parts[0];
            let dependencies = self.extract_dependencies_from_content(content);
            let cas_address = self.calculate_decl_cas(trait_name, "trait", &dependencies, line);

            let decl = ASTDeclaration {
                name: trait_name.to_string(),
                decl_type: "trait".to_string(),
                source_file: file.to_string(),
                line_number: line_num,
                content: line.to_string(),
                dependencies,
                cas_address,
            };

            self.add_declaration(decl);
        }
    }

    fn extract_dependencies_from_content(&self, content: &str) -> Vec<String> {
        let mut deps = Vec::new();

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("use ") {
                let use_part = trimmed
                    .strip_prefix("use ")
                    .unwrap_or("")
                    .strip_suffix(';')
                    .unwrap_or("");
                if let Some(last_colon) = use_part.rfind("::") {
                    let item = &use_part[last_colon + 2..];
                    deps.push(item.to_string());
                }
            }
        }

        deps
    }

    fn calculate_decl_cas(
        &self,
        name: &str,
        decl_type: &str,
        deps: &[String],
        content: &str,
    ) -> u64 {
        let mut signature = calculate_phi_key(name);
        signature = signature.wrapping_add(calculate_phi_key(decl_type));
        signature = signature.wrapping_add(content.bytes().map(|b| b as u64).sum::<u64>());

        for dep in deps {
            signature = signature.wrapping_add(calculate_phi_key(dep));
        }

        signature % 196883
    }

    fn add_declaration(&mut self, decl: ASTDeclaration) {
        let cas = decl.cas_address;
        let name = decl.name.clone();

        println!(
            "  {} {} → CAS: {} ({} deps)",
            decl.decl_type,
            name,
            cas,
            decl.dependencies.len()
        );

        self.declarations.insert(cas, decl);
        self.name_to_cas
            .entry(name)
            .or_insert_with(Vec::new)
            .push(cas);
    }

    fn show_database(&self) {
        println!("\n=== AST Declarations Database (DB3) ===");
        println!("Total declarations: {}", self.declarations.len());

        let mut by_type: HashMap<String, usize> = HashMap::new();
        for decl in self.declarations.values() {
            *by_type.entry(decl.decl_type.clone()).or_insert(0) += 1;
        }

        for (decl_type, count) in &by_type {
            println!("  {}: {}", decl_type, count);
        }

        println!("\nSample declarations:");
        for (cas, decl) in self.declarations.iter().take(10) {
            println!(
                "🔧 {} {} (CAS: {}) at {}:{}",
                decl.decl_type,
                decl.name,
                cas,
                decl.source_file.split('/').last().unwrap_or(""),
                decl.line_number
            );
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
    println!("=== AST Declarations CAS Database 3 ===");

    let mut db3 = ASTDeclsDB3::new();
    db3.load_all_ast_declarations()?;
    db3.show_database();

    println!("\n✓ All AST declarations loaded into DB3");
    println!("✓ CAS addresses calculated from declaration signatures");

    Ok(())
}
