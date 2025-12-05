//! Ingest entire rustc codebase into level0 blocks with dependency arrows

use rust_71_parts::rustc_ingester::RustcIngester;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <rustc_source_path> [db_path]", args[0]);
        eprintln!("Example: {} /path/to/rust/compiler/rustc_ast ./rustc_blocks.db", args[0]);
        return Ok(());
    }
    
    let rustc_path = &args[1];
    let default_db = "./rustc_blocks.db".to_string();
    let db_path = args.get(2).unwrap_or(&default_db);
    
    println!("🦀 Rustc Level0 Block Ingester");
    println!("Source: {}", rustc_path);
    println!("Database: {}", db_path);
    
    let mut ingester = RustcIngester::new(db_path)?;
    
    println!("\n📦 Breaking rustc into atomic level0 blocks...");
    ingester.ingest_rustc(rustc_path)?;
    
    let (blocks, edges) = ingester.get_stats();
    println!("\n📊 Ingestion Complete:");
    println!("  🧱 Level0 blocks: {}", blocks);
    println!("  ➡️  Dependency arrows: {}", edges);
    
    println!("\n📈 Exporting dependency graph...");
    let dot_graph = ingester.export_dot();
    std::fs::write("rustc_dependency_graph.dot", dot_graph)?;
    println!("✓ Saved to rustc_dependency_graph.dot");
    
    println!("\n🎯 Each block stored with Monster Group coordinates");
    println!("🔗 Dependency arrows map the rustc compilation lattice");
    println!("📊 Use: dot -Tpng rustc_dependency_graph.dot -o rustc_graph.png");
    
    Ok(())
}
