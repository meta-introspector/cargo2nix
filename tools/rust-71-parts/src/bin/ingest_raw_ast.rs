//! Simple ingester: Raw source + AST with streaming and progress

use rust_71_parts::content_addressable_memory::*;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Instant;
use syn::parse_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <source_path> [db_path] [threads]", args[0]);
        return Ok(());
    }
    
    let source_path = &args[1];
    let default_db = "./raw_ast.db".to_string();
    let db_path = args.get(2).unwrap_or(&default_db);
    let threads = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(8);
    
    println!("🦀 Raw Source + AST Ingester");
    println!("Source: {}", source_path);
    println!("Database: {}", db_path);
    println!("Threads: {}", threads);
    
    let start_time = Instant::now();
    let processed = Arc::new(Mutex::new(0usize));
    let memory = Arc::new(Mutex::new(MonsterMemory::new(db_path)?));
    
    // Channel for streaming files to workers
    let (tx, rx) = mpsc::channel::<std::path::PathBuf>();
    let rx = Arc::new(Mutex::new(rx));
    let mut handles = vec![];
    
    // Start worker threads
    for thread_id in 0..threads {
        let rx = Arc::clone(&rx);
        let processed = Arc::clone(&processed);
        let memory = Arc::clone(&memory);
        
        let handle = thread::spawn(move || {
            loop {
                let file_path = {
                    let rx = rx.lock().unwrap();
                    match rx.recv() {
                        Ok(path) => path,
                        Err(_) => break, // Channel closed
                    }
                };
                
                match process_file(&memory, &file_path) {
                    Ok(()) => {
                        let mut p = processed.lock().unwrap();
                        *p += 1;
                        if *p % 100 == 0 {
                            println!("  Thread {}: ✓ {} files processed", thread_id, *p);
                        }
                    }
                    Err(e) => {
                        eprintln!("  Thread {}: ✗ {}: {}", thread_id, file_path.display(), e);
                    }
                }
            }
        });
        handles.push(handle);
    }
    
    // Scan and stream files
    println!("📁 Scanning and processing files...");
    let mut total_files = 0;
    scan_and_stream(Path::new(source_path), &tx, &mut total_files)?;
    
    // Close channel and wait
    drop(tx);
    for handle in handles {
        handle.join().unwrap();
    }
    
    let elapsed = start_time.elapsed();
    let processed_count = *processed.lock().unwrap();
    
    println!("\n📊 Ingestion Complete in {:.2}s:", elapsed.as_secs_f64());
    println!("  ✓ Processed: {}", processed_count);
    println!("  📄 Total files: {}", total_files);
    println!("  ⚡ Rate: {:.1} files/sec", processed_count as f64 / elapsed.as_secs_f64());
    
    Ok(())
}

fn scan_and_stream(
    dir: &Path, 
    tx: &mpsc::Sender<std::path::PathBuf>,
    total_files: &mut usize
) -> Result<(), Box<dyn std::error::Error>> {
    println!("📂 Scanning: {}", dir.display());
    
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            scan_and_stream(&path, tx, total_files)?;
        } else if path.extension().map_or(false, |ext| ext == "rs") {
            *total_files += 1;
            if *total_files % 1000 == 0 {
                println!("  📊 Found {} Rust files so far...", *total_files);
            }
            
            // Send to worker immediately
            if tx.send(path).is_err() {
                break; // Channel closed
            }
        }
    }
    Ok(())
}

fn process_file(memory: &Arc<Mutex<MonsterMemory>>, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let raw_source = fs::read_to_string(file_path)?;
    
    // Store raw source
    let location = SourceLocation {
        file: format!("{}#raw", file_path.display()),
        line: 1,
        column: 0,
    };
    
    {
        let mut mem = memory.lock().unwrap();
        mem.store(&raw_source, location)?;
    }
    
    // Try to parse AST
    if let Ok(ast) = parse_file(&raw_source) {
        let ast_repr = format!("{:#?}", ast);
        let ast_location = SourceLocation {
            file: format!("{}#ast", file_path.display()),
            line: 1,
            column: 0,
        };
        
        let mut mem = memory.lock().unwrap();
        mem.store(&ast_repr, ast_location)?;
    }
    
    Ok(())
}
