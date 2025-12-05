//! Store AST chunks in Solana file storage

use rust_71_parts::chunked_ast_processor::ChunkedAstProcessor;
use rust_71_parts::solana_rocksdb_storage::{SolanaRocksStorage, StorageStats};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <command> [args...]", args[0]);
        eprintln!("Commands:");
        eprintln!("  store <rust_file> [storage_path] - Store file chunks in file storage");
        eprintln!("  query <factor> [storage_path]    - Query chunks by Monster factor");
        eprintln!("  stats [storage_path]             - Show storage statistics");
        eprintln!("  list [storage_path]              - List all chunks");
        return Ok(());
    }
    
    let command = &args[1];
    let storage_path = args.get(3).map(|s| s.as_str()).unwrap_or("./solana_ast_storage");
    
    match command.as_str() {
        "store" => {
            if args.len() < 3 {
                eprintln!("Usage: {} store <rust_file> [storage_path]", args[0]);
                return Ok(());
            }
            
            let rust_file = &args[2];
            store_file_chunks(rust_file, storage_path)?;
        }
        
        "query" => {
            if args.len() < 3 {
                eprintln!("Usage: {} query <factor> [storage_path]", args[0]);
                return Ok(());
            }
            
            let factor: u64 = args[2].parse()?;
            query_by_factor(factor, storage_path)?;
        }
        
        "stats" => {
            show_storage_stats(storage_path)?;
        }
        
        "list" => {
            list_all_chunks(storage_path)?;
        }
        
        _ => {
            eprintln!("Unknown command: {}", command);
            eprintln!("Use 'store', 'query', 'stats', or 'list'");
        }
    }
    
    Ok(())
}

fn store_file_chunks(rust_file: &str, storage_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Processing file: {}", rust_file);
    
    // Process file into chunks
    let mut processor = ChunkedAstProcessor::new();
    let chunks = processor.process_file(rust_file)?;
    
    println!("✓ Generated {} chunks", chunks.len());
    
    // Store in RocksDB
    let storage = SolanaRocksStorage::new(storage_path)?;
    storage.store_chunks(&chunks)?;
    
    println!("✓ Stored {} chunks in file storage", chunks.len());
    
    // Show chunk IDs
    for chunk in &chunks {
        println!("  Chunk {}: Monster factor {}, {} bytes -> {} bytes", 
            chunk.chunk_id, 
            chunk.monster_factor,
            chunk.original_size,
            chunk.compressed_data.len());
    }
    
    // Show storage stats
    let stats = storage.get_stats();
    print_storage_stats(&stats);
    
    Ok(())
}

fn query_by_factor(factor: u64, storage_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Querying Monster factor: {}", factor);
    
    let storage = SolanaRocksStorage::new(storage_path)?;
    let chunks = storage.get_chunks_by_factor(factor)?;
    
    if chunks.is_empty() {
        println!("No chunks found for Monster factor {}", factor);
        return Ok(());
    }
    
    println!("✓ Found {} chunks with Monster factor {}", chunks.len(), factor);
    
    for (i, chunk) in chunks.iter().enumerate() {
        println!("  Chunk {}: ID {}", i, chunk.chunk_id);
        println!("    File: {}", chunk.file_path);
        println!("    Original size: {} bytes", chunk.original_size);
        println!("    Compressed size: {} bytes", chunk.compressed_data.len());
        println!("    Compression: {:.1}%", 
            (chunk.compressed_data.len() as f64 / chunk.original_size as f64) * 100.0);
        println!("    Byte range: {:?}", chunk.byte_range);
    }
    
    Ok(())
}

fn show_storage_stats(storage_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 Storage Statistics for: {}", storage_path);
    
    let storage = SolanaRocksStorage::new(storage_path)?;
    let stats = storage.get_stats();
    
    print_storage_stats(&stats);
    
    // Show Monster factors
    let chunks = storage.get_all_chunks()?;
    let mut factor_counts = std::collections::HashMap::new();
    
    for chunk in chunks {
        *factor_counts.entry(chunk.monster_factor).or_insert(0) += 1;
    }
    
    println!("\n🎯 Monster Factors in storage:");
    let mut factors: Vec<_> = factor_counts.keys().collect();
    factors.sort();
    
    for factor in factors {
        let count = factor_counts[factor];
        println!("  Factor {}: {} chunks", factor, count);
    }
    
    Ok(())
}

fn list_all_chunks(storage_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("📋 Listing all chunks in: {}", storage_path);
    
    let storage = SolanaRocksStorage::new(storage_path)?;
    let chunk_ids = storage.list_chunk_ids()?;
    
    if chunk_ids.is_empty() {
        println!("No chunks found in storage");
        return Ok(());
    }
    
    println!("✓ Found {} chunks", chunk_ids.len());
    
    for chunk_id in chunk_ids {
        if let Some(chunk) = storage.get_chunk(chunk_id)? {
            println!("  Chunk {}: Factor {}, File: {}, Size: {} -> {} bytes", 
                chunk.chunk_id,
                chunk.monster_factor,
                chunk.file_path,
                chunk.original_size,
                chunk.compressed_data.len());
        }
    }
    
    Ok(())
}

fn print_storage_stats(stats: &StorageStats) {
    println!("\n📊 File Storage Statistics:");
    println!("  Storage path: {}", stats.storage_path);
    println!("  Total chunks: {}", stats.chunk_count);
    println!("  Total size: {} bytes ({:.2} MB)", 
        stats.total_size_bytes, 
        stats.total_size_bytes as f64 / 1_048_576.0);
}
