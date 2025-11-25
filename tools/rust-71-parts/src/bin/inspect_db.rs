use std::env;
use rocksdb::{DB, Options};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <db_path>", args[0]);
        std::process::exit(1);
    }

    let db_path = &args[1];
    println!("🔍 Inspecting database: {}", db_path);

    let opts = Options::default();
    let db = DB::open_for_read_only(&opts, db_path, false)?;

    let iter = db.iterator(rocksdb::IteratorMode::Start);
    let mut count = 0;
    
    for item in iter {
        let (key, value) = item?;
        let key_str = String::from_utf8_lossy(&key);
        
        count += 1;
        if count <= 5 {
            println!("Key {}: {}", count, key_str);
            println!("  Value size: {} bytes", value.len());
            
            // Try to parse as JSON
            if let Ok(json_value) = serde_json::from_slice::<serde_json::Value>(&value) {
                println!("  JSON structure: {}", serde_json::to_string_pretty(&json_value)?);
            } else {
                println!("  Raw value (first 100 chars): {}", 
                    String::from_utf8_lossy(&value[..std::cmp::min(100, value.len())]));
            }
            println!();
        }
        
        if count % 1000 == 0 {
            println!("  ... processed {} entries", count);
        }
    }

    println!("📊 Total entries: {}", count);
    Ok(())
}
