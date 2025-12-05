//! Chunk rustc source into 4K compressed blocks with Monster Group filtering

use rust_71_parts::chunked_ast_processor::{ChunkedAstProcessor, CompressionStats};
use std::env;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <rustc_source_path> [cache_file]", args[0]);
        eprintln!("Example: {} /path/to/rustc/compiler/rustc_ast/src/ast.rs", args[0]);
        return Ok(());
    }
    
    let source_path = &args[1];
    let cache_file = args.get(2).map(|s| s.as_str()).unwrap_or("ast_cache.bin");
    
    let mut processor = ChunkedAstProcessor::new();
    
    // Load existing cache
    if Path::new(cache_file).exists() {
        println!("Loading cache from {}", cache_file);
        processor.load_cache(cache_file)?;
    }
    
    println!("Processing rustc source: {}", source_path);
    
    // Process the file
    let chunks = processor.process_file(source_path)?;
    
    println!("✓ Generated {} chunks", chunks.len());
    
    // Show compression stats
    let stats = processor.get_compression_stats();
    print_compression_stats(&stats);
    
    // Query examples
    println!("\n🔍 Query Examples:");
    
    // Find chunks with specific Monster factors
    for factor in [1, 7, 13, 71] {
        let matching_chunks = processor.query_chunks_by_factor(factor);
        if !matching_chunks.is_empty() {
            println!("  Monster factor {}: {} chunks", factor, matching_chunks.len());
        }
    }
    
    // Find files with many traits
    let trait_heavy_files = processor.query_by_ast_summary(|summary| summary.trait_count > 5);
    println!("  Files with >5 traits: {}", trait_heavy_files.len());
    
    // Find files with many functions
    let function_heavy_files = processor.query_by_ast_summary(|summary| summary.function_count > 20);
    println!("  Files with >20 functions: {}", function_heavy_files.len());
    
    // Save cache
    processor.save_cache(cache_file)?;
    println!("✓ Cache saved to {}", cache_file);
    
    // Test decompression of first chunk
    if let Some(chunk) = chunks.first() {
        println!("\n🧪 Testing decompression:");
        let decompressed = processor.decompress_chunk(chunk)?;
        println!("  Chunk {} decompressed: {} bytes", chunk.chunk_id, decompressed.len());
        println!("  Monster factor: {}", chunk.monster_factor);
        println!("  Compression ratio: {:.2}%", 
            (chunk.compressed_data.len() as f64 / chunk.original_size as f64) * 100.0);
    }
    
    Ok(())
}

fn print_compression_stats(stats: &CompressionStats) {
    println!("\n📊 Compression Statistics:");
    println!("  Total chunks: {}", stats.chunk_count);
    println!("  Original size: {} bytes ({:.2} MB)", 
        stats.total_original_bytes, 
        stats.total_original_bytes as f64 / 1_048_576.0);
    println!("  Compressed size: {} bytes ({:.2} MB)", 
        stats.total_compressed_bytes,
        stats.total_compressed_bytes as f64 / 1_048_576.0);
    println!("  Compression ratio: {:.2}% ({:.2}x reduction)", 
        stats.compression_ratio * 100.0,
        1.0 / stats.compression_ratio);
    
    let savings = stats.total_original_bytes.saturating_sub(stats.total_compressed_bytes);
    println!("  Space saved: {} bytes ({:.2} MB)", 
        savings,
        savings as f64 / 1_048_576.0);
}
