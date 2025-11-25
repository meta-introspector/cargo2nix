use std::collections::HashMap;
use std::env;
use rocksdb::{DB, Options};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct Level0Block {
    hash: String,
    content: String,
    file_path: String,
    block_index: usize,
}

#[derive(Debug, Serialize)]
struct TermIndex {
    term: String,
    gram_size: usize,
    blocks: Vec<String>, // block hashes
    frequency: usize,
}

fn extract_ngrams(text: &str, n: usize) -> Vec<String> {
    let tokens: Vec<&str> = text.split_whitespace().collect();
    if tokens.len() < n {
        return vec![];
    }
    
    tokens.windows(n)
        .map(|window| window.join(" "))
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <source_db_path> <index_db_path>", args[0]);
        std::process::exit(1);
    }

    let source_db_path = &args[1];
    let index_db_path = &args[2];

    println!("🔍 Building term index from {}", source_db_path);
    println!("📊 Output index: {}", index_db_path);

    let mut opts = Options::default();
    opts.create_if_missing(true);
    
    let source_db = DB::open_for_read_only(&opts, source_db_path, false)?;
    let index_db = DB::open(&opts, index_db_path)?;

    let mut term_map: HashMap<String, Vec<String>> = HashMap::new();
    let gram_sizes = [1, 2, 3, 5, 7];

    let iter = source_db.iterator(rocksdb::IteratorMode::Start);
    let mut processed = 0;

    for item in iter {
        let (key, value) = item?;
        let key_str = String::from_utf8_lossy(&key);
        
        if let Ok(block) = serde_json::from_slice::<Level0Block>(&value) {
            for &n in &gram_sizes {
                let ngrams = extract_ngrams(&block.content, n);
                for gram in ngrams {
                    let term_key = format!("{}:{}", n, gram);
                    term_map.entry(term_key).or_default().push(block.hash.clone());
                }
            }
            
            processed += 1;
            if processed % 100 == 0 {
                println!("  📄 Processed {} blocks", processed);
            }
        }
    }

    let total_terms = term_map.len();
    println!("💾 Writing {} terms to index...", total_terms);
    
    for (term_key, block_hashes) in term_map {
        let parts: Vec<&str> = term_key.splitn(2, ':').collect();
        let gram_size: usize = parts[0].parse()?;
        let term = parts[1].to_string();
        
        let index_entry = TermIndex {
            term: term.clone(),
            gram_size,
            blocks: block_hashes.clone(),
            frequency: block_hashes.len(),
        };
        
        let value = serde_json::to_vec(&index_entry)?;
        index_db.put(term_key.as_bytes(), value)?;
    }

    println!("✅ Term index complete!");
    println!("  📊 Total terms indexed: {}", total_terms);
    println!("  📄 Blocks processed: {}", processed);

    Ok(())
}
