use std::fs;
use std::process::Command;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct SolanaCargoAccount {
    account_address: String,
    cargo_crate_name: String,
    cargo_version: String,
    cargo_toml_path: String,
    content_hash: String,
    git_object: String,
    git_repo: String,
}

struct SolanaCargoAccountMapper {
    accounts: Vec<SolanaCargoAccount>,
}

impl SolanaCargoAccountMapper {
    fn new() -> Self {
        Self {
            accounts: Vec::new(),
        }
    }
    
    fn scan_cargo_tomls_for_accounts(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📦 Scanning Cargo.toml files for Solana account creation...");
        
        let output = Command::new("find")
            .args(&["../submodules", "-name", "Cargo.toml", "-type", "f"])
            .output()?;
        
        if !output.status.success() {
            return Ok(());
        }
        
        let cargo_files = String::from_utf8_lossy(&output.stdout);
        
        for cargo_toml_path in cargo_files.lines() {
            if let Ok(content) = fs::read_to_string(cargo_toml_path) {
                if let Some(account) = self.create_solana_account(cargo_toml_path, &content) {
                    self.accounts.push(account);
                }
            }
        }
        
        println!("  ✓ Created {} Solana accounts for cargo crates", self.accounts.len());
        Ok(())
    }
    
    fn create_solana_account(&self, cargo_toml_path: &str, content: &str) -> Option<SolanaCargoAccount> {
        let (name, version) = self.extract_name_version(content);
        let content_hash = self.calculate_content_hash(content);
        let git_repo = self.extract_git_repo(cargo_toml_path);
        let git_object = self.get_git_object(&git_repo);
        
        // Generate Solana account address from content hash
        let account_address = self.generate_solana_address(&content_hash, &name);
        
        Some(SolanaCargoAccount {
            account_address,
            cargo_crate_name: name,
            cargo_version: version,
            cargo_toml_path: cargo_toml_path.to_string(),
            content_hash,
            git_object,
            git_repo,
        })
    }
    
    fn generate_solana_address(&self, content_hash: &str, crate_name: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        // Create deterministic Solana-style address from content hash + crate name
        let seed = format!("{}:{}", content_hash, crate_name);
        let mut hasher = DefaultHasher::new();
        seed.hash(&mut hasher);
        
        // Generate 32-byte address (Solana public key format)
        let hash = hasher.finish();
        format!("{:032x}", hash)
    }
    
    fn calculate_content_hash(&self, content: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        format!("{:016x}", hasher.finish())
    }
    
    fn extract_name_version(&self, content: &str) -> (String, String) {
        let mut name = "unknown".to_string();
        let mut version = "unknown".to_string();
        
        for line in content.lines() {
            let line = line.trim();
            
            if line.starts_with("name = ") {
                if let Some(n) = line.split('"').nth(1) {
                    name = n.to_string();
                }
            } else if line.starts_with("version = ") {
                if let Some(v) = line.split('"').nth(1) {
                    version = v.to_string();
                }
            }
        }
        
        (name, version)
    }
    
    fn extract_git_repo(&self, cargo_toml_path: &str) -> String {
        if let Some(submodules_pos) = cargo_toml_path.find("submodules/") {
            let after_submodules = &cargo_toml_path[submodules_pos + 11..];
            if let Some(slash_pos) = after_submodules.find('/') {
                return after_submodules[..slash_pos].to_string();
            }
        }
        "unknown".to_string()
    }
    
    fn get_git_object(&self, git_repo: &str) -> String {
        let submodule_path = format!("submodules/{}", git_repo);
        
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
    
    fn generate_rocksdb_entries(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🗄️ Generating RocksDB entries for Solana accounts...");
        
        let mut rocksdb_script = String::new();
        rocksdb_script.push_str("# RocksDB Solana Account Entries\n");
        rocksdb_script.push_str("# One Solana account per Rust cargo crate Cargo.toml file\n\n");
        
        for account in &self.accounts {
            rocksdb_script.push_str(&format!("# Solana Account for {} v{}\n", 
                account.cargo_crate_name, account.cargo_version));
            
            rocksdb_script.push_str(&format!("PUT solana_account:{} {{\n", account.account_address));
            rocksdb_script.push_str(&format!("  \"account_address\": \"{}\",\n", account.account_address));
            rocksdb_script.push_str(&format!("  \"cargo_crate_name\": \"{}\",\n", account.cargo_crate_name));
            rocksdb_script.push_str(&format!("  \"cargo_version\": \"{}\",\n", account.cargo_version));
            rocksdb_script.push_str(&format!("  \"cargo_toml_path\": \"{}\",\n", account.cargo_toml_path));
            rocksdb_script.push_str(&format!("  \"content_hash\": \"{}\",\n", account.content_hash));
            rocksdb_script.push_str(&format!("  \"git_object\": \"{}\",\n", account.git_object));
            rocksdb_script.push_str(&format!("  \"git_repo\": \"{}\",\n", account.git_repo));
            rocksdb_script.push_str("  \"account_type\": \"cargo_crate\"\n");
            rocksdb_script.push_str("}\n\n");
        }
        
        fs::write("ROCKSDB_SOLANA_ACCOUNTS.txt", &rocksdb_script)?;
        
        println!("  ✓ Generated RocksDB entries in ROCKSDB_SOLANA_ACCOUNTS.txt");
        Ok(())
    }
    
    fn generate_account_report(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📊 Generating Solana account mapping report...");
        
        let mut report = String::new();
        report.push_str("# Solana Account per Cargo Crate Report\n\n");
        
        report.push_str("## Account Mapping Flow\n");
        report.push_str("Cargo.toml -> content hash -> Solana account address -> RocksDB entry\n\n");
        
        report.push_str("## Solana Accounts for Cargo Crates\n\n");
        
        for (i, account) in self.accounts.iter().enumerate() {
            report.push_str(&format!("### {} v{}\n", account.cargo_crate_name, account.cargo_version));
            report.push_str(&format!("- **Solana Account**: `{}`\n", &account.account_address[..32]));
            report.push_str(&format!("- **Cargo Crate**: `{}` v`{}`\n", account.cargo_crate_name, account.cargo_version));
            report.push_str(&format!("- **Content Hash**: `{}`\n", &account.content_hash[..16]));
            report.push_str(&format!("- **Git Repository**: `{}`\n", account.git_repo));
            report.push_str(&format!("- **Git Object**: `{}`\n", &account.git_object[..16]));
            
            let short_path = account.cargo_toml_path.replace("../", "");
            report.push_str(&format!("- **Cargo.toml Path**: `{}`\n", short_path));
            
            report.push_str(&format!("- **Account Mapping**: `{}` -> Solana Account `{}`\n", 
                account.cargo_crate_name, &account.account_address[..16]));
            
            report.push_str("\n");
            
            if i >= 15 { // Show first 15
                report.push_str(&format!("... and {} more accounts\n\n", self.accounts.len() - 15));
                break;
            }
        }
        
        // Group by repository
        let mut by_repo: HashMap<String, Vec<&SolanaCargoAccount>> = HashMap::new();
        for account in &self.accounts {
            by_repo.entry(account.git_repo.clone()).or_insert_with(Vec::new).push(account);
        }
        
        report.push_str("## Accounts by Repository\n");
        for (repo, accounts) in &by_repo {
            report.push_str(&format!("- **{}**: {} Solana accounts\n", repo, accounts.len()));
        }
        
        report.push_str("\n## Account Statistics\n");
        report.push_str(&format!("- Total Solana accounts: {}\n", self.accounts.len()));
        report.push_str(&format!("- Repositories with accounts: {}\n", by_repo.len()));
        report.push_str(&format!("- Average accounts per repo: {:.1}\n", 
            self.accounts.len() as f64 / by_repo.len() as f64));
        
        report.push_str("\n## RocksDB Schema\n");
        report.push_str("```\n");
        report.push_str("Key: solana_account:{account_address}\n");
        report.push_str("Value: {\n");
        report.push_str("  account_address: string,\n");
        report.push_str("  cargo_crate_name: string,\n");
        report.push_str("  cargo_version: string,\n");
        report.push_str("  cargo_toml_path: string,\n");
        report.push_str("  content_hash: string,\n");
        report.push_str("  git_object: string,\n");
        report.push_str("  git_repo: string,\n");
        report.push_str("  account_type: \"cargo_crate\"\n");
        report.push_str("}\n");
        report.push_str("```\n");
        
        report.push_str("\n## Monster Protocol Integration\n");
        report.push_str("- Each Cargo.toml file has unique Solana account\n");
        report.push_str("- Content-addressable storage via content hash\n");
        report.push_str("- Git object tracking for version control\n");
        report.push_str("- RocksDB storage for efficient queries\n");
        report.push_str("- Deterministic account generation from content\n");
        
        fs::write("SOLANA_CARGO_ACCOUNTS.md", &report)?;
        
        println!("  ✓ Report written to SOLANA_CARGO_ACCOUNTS.md");
        Ok(())
    }
    
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🎯 Solana Account per Cargo Crate Mapper");
        
        self.scan_cargo_tomls_for_accounts()?;
        self.generate_rocksdb_entries()?;
        self.generate_account_report()?;
        
        // Group by repository for stats
        let mut by_repo: HashMap<String, Vec<&SolanaCargoAccount>> = HashMap::new();
        for account in &self.accounts {
            by_repo.entry(account.git_repo.clone()).or_insert_with(Vec::new).push(account);
        }
        
        println!("\n🎯 === SOLANA ACCOUNT MAPPING COMPLETE ===");
        println!("  Total Solana accounts: {}", self.accounts.len());
        println!("  Repositories covered: {}", by_repo.len());
        println!("  RocksDB entries generated: {}", self.accounts.len());
        
        println!("\n🔗 SOLANA MAPPING: Cargo.toml -> Solana Account -> RocksDB");
        println!("💾 ONE ROCKSDB SOLANA ACCOUNT PER RUST CARGO CRATE TOML FILE");
        
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut mapper = SolanaCargoAccountMapper::new();
    mapper.run()
}
