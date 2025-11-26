use std::path::Path;
use std::fs;

pub struct RepoDatabase {
    pub path: String,
}

impl RepoDatabase {
    pub fn new(db_path: &str) -> Self {
        fs::create_dir_all(Path::new(db_path).parent().unwrap()).unwrap();
        Self { path: db_path.to_string() }
    }
    
    pub fn init_git_repos(&self) -> Vec<String> {
        vec![
            "https://github.com/rust-lang/rust.git".to_string(),
            "https://github.com/rust-lang/cargo.git".to_string(),
            "https://github.com/tokio-rs/tokio.git".to_string(),
            "https://github.com/serde-rs/serde.git".to_string(),
            "https://github.com/clap-rs/clap.git".to_string(),
        ]
    }
}

fn main() {
    let db = RepoDatabase::new("tools/rust-71-parts/git_repos.db");
    let repos = db.init_git_repos();
    println!("Initialized {} repositories", repos.len());
}
