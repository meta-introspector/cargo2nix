use std::env;
use rocksdb::{DB, Options};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct TermIndex {
    term: String,
    gram_size: usize,
    blocks: Vec<String>,
    frequency: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <index_db_path> <gram_size> [term]", args[0]);
        eprintln!("  gram_size: 1, 2, 3, 5, or 7");
        eprintln!("  term: optional search term (shows top terms if omitted)");
        std::process::exit(1);
    }

    let index_db_path = &args[1];
    let gram_size: usize = args[2].parse()?;
    let search_term = args.get(3);

    let opts = Options::default();
    let db = DB::open_for_read_only(&opts, index_db_path, false)?;

    if let Some(term) = search_term {
        // Search for specific term
        let key = format!("{}:{}", gram_size, term);
        if let Ok(Some(value)) = db.get(key.as_bytes()) {
            let index: TermIndex = serde_json::from_slice(&value)?;
            println!("🔍 Term: '{}' ({}-gram)", index.term, index.gram_size);
            println!("📊 Frequency: {} blocks", index.frequency);
            println!("📄 Found in blocks:");
            for (i, hash) in index.blocks.iter().enumerate() {
                println!("  {}: {}", i + 1, hash);
                if i >= 9 { // Show first 10
                    println!("  ... and {} more", index.blocks.len() - 10);
                    break;
                }
            }
        } else {
            println!("❌ Term '{}' not found as {}-gram", term, gram_size);
        }
    } else {
        // Show top terms for gram size
        let mut terms: Vec<TermIndex> = Vec::new();
        let prefix = format!("{}:", gram_size);
        
        let iter = db.iterator(rocksdb::IteratorMode::Start);
        for item in iter {
            let (key, value) = item?;
            let key_str = String::from_utf8_lossy(&key);
            
            if key_str.starts_with(&prefix) {
                if let Ok(index) = serde_json::from_slice::<TermIndex>(&value) {
                    terms.push(index);
                }
            }
        }
        
        terms.sort_by(|a, b| b.frequency.cmp(&a.frequency));
        
        println!("📊 Top {}-grams by frequency:", gram_size);
        for (i, term) in terms.iter().take(20).enumerate() {
            println!("  {}: '{}' ({} blocks)", i + 1, term.term, term.frequency);
        }
        println!("\nTotal {}-grams: {}", gram_size, terms.len());
    }

    Ok(())
}
