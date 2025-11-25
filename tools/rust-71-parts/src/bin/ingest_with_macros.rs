//! Ingest rustc with complete macro expansion pipeline

use rust_71_parts::macro_expansion_ingester::MacroExpansionIngester;
use std::env;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <rustc_source_path> [db_path] [cache_dir]", args[0]);
        eprintln!("Example: {} /path/to/rust/compiler ./rustc_expansion.db ./expansion_cache", args[0]);
        return Ok(());
    }
    
    let rustc_path = &args[1];
    let default_db = "./rustc_expansion.db".to_string();
    let db_path = args.get(2).unwrap_or(&default_db);
    let default_cache = "./expansion_cache".to_string();
    let cache_dir = args.get(3).unwrap_or(&default_cache);
    
    println!("🦀 Rustc Macro Expansion Pipeline Ingester");
    println!("Source: {}", rustc_path);
    println!("Database: {}", db_path);
    println!("Cache: {}", cache_dir);
    
    let mut ingester = MacroExpansionIngester::new(db_path, cache_dir)?;
    
    println!("\n📦 Processing files through expansion pipeline...");
    println!("Stages: Raw → Macro Expansion → AST Parsing");
    
    // Find all Rust files
    let rust_files = find_rust_files(rustc_path)?;
    println!("Found {} Rust files", rust_files.len());
    
    let crate_root = Path::new(rustc_path);
    let mut processed = 0;
    
    for file_path in &rust_files {
        match ingester.ingest_with_expansion(file_path, crate_root) {
            Ok(()) => {
                processed += 1;
                if processed % 50 == 0 {
                    println!("  ✓ Processed {} files...", processed);
                }
            }
            Err(e) => {
                eprintln!("  ✗ Failed to process {}: {}", file_path.display(), e);
            }
        }
    }
    
    let (total, expanded, ast) = ingester.get_expansion_stats();
    
    println!("\n📊 Expansion Pipeline Complete:");
    println!("  📄 Total files: {}", total);
    println!("  🔧 Macro expanded: {}", expanded);
    println!("  🌳 AST parsed: {}", ast);
    
    println!("\n📈 Exporting expansion pipeline graph...");
    let expansion_graph = ingester.export_expansion_graph();
    std::fs::write("expansion_pipeline.dot", expansion_graph)?;
    println!("✓ Saved to expansion_pipeline.dot");
    
    println!("\n🎯 Multi-stage ingestion complete!");
    println!("Each file stored as: raw → expanded → AST with Monster coordinates");
    println!("📊 Use: dot -Tpng expansion_pipeline.dot -o expansion_pipeline.png");
    
    Ok(())
}

fn find_rust_files(dir: &str) -> Result<Vec<std::path::PathBuf>, Box<dyn std::error::Error>> {
    let mut rust_files = Vec::new();
    
    fn visit_dir(dir: &Path, files: &mut Vec<std::path::PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                visit_dir(&path, files)?;
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                files.push(path);
            }
        }
        Ok(())
    }
    
    visit_dir(Path::new(dir), &mut rust_files)?;
    Ok(rust_files)
}
