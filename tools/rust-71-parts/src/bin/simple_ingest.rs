use std::env;
use std::fs;
use rocksdb::{DB, Options};
use serde::{Serialize};
use sha2::{Sha256, Digest};

#[derive(Debug, Serialize)]
struct Level0Block {
    hash: String,
    content: String,
    file_path: String,
    block_index: usize,
}

fn visit_dir(dir: &std::path::Path, db: &DB, processed: &mut usize) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            visit_dir(&path, db, processed)?;
        } else if path.extension().map_or(false, |ext| ext == "rs") {
            let content = fs::read_to_string(&path)?;
            let mut hasher = Sha256::new();
            hasher.update(&content);
            let hash = format!("{:x}", hasher.finalize());
            
            let block = Level0Block {
                hash: hash.clone(),
                content,
                file_path: path.to_string_lossy().to_string(),
                block_index: *processed,
            };
            
            let value = serde_json::to_vec(&block)?;
            db.put(hash.as_bytes(), value)?;
            
            *processed += 1;
            if *processed % 10 == 0 {
                println!("  📄 Processed {} files", processed);
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <source_dir> <db_path>", args[0]);
        std::process::exit(1);
    }

    let source_dir = &args[1];
    let db_path = &args[2];

    println!("📁 Ingesting Rust files from: {}", source_dir);
    println!("💾 Database: {}", db_path);

    let mut opts = Options::default();
    opts.create_if_missing(true);
    let db = DB::open(&opts, db_path)?;

    let mut processed = 0;
    visit_dir(std::path::Path::new(source_dir), &db, &mut processed)?;

    println!("✅ Ingestion complete!");
    println!("  📄 Files processed: {}", processed);

    Ok(())
}
