use clap::{Arg, Command};
use std::path::Path;
use anyhow::Result;

mod file_scanner;
mod dependency_analyzer;
mod monster_factor_collector;
mod term_collector;
mod declarations;
mod inductive_declarations_collector;

use file_scanner::FileScanner;
use dependency_analyzer::DependencyAnalyzer;
use monster_factor_collector::{ASTFactorCollector, MonsterFactorSolver};
use term_collector::ComprehensiveTermCollector;
use inductive_declarations_collector::InductiveDeclarationsCollector;
use crate::declarations::NixDeclaration;
use rocksdb::{DB, Options}; // Import rocksdb
use syn::visit::Visit; // Add this import

fn main() -> Result<()> {
    let matches = Command::new("rust-src-scanner")
        .version("0.1.0")
        .author("Your Name <your.email@example.com>")
        .about("Scans Rust source code for various insights")
        .arg(
            Arg::new("rust-src-path")
                .long("rust-src-path")
                .help("Path to the Rust source code to scan")
                .required(true)
        )
        .arg(
            Arg::new("output-dir")
                .long("output-dir")
                .help("Directory to output results (e.g., RocksDB, JSON)")
                .required(true)
        )
        .arg(
            Arg::new("inductive-decls")
                .long("inductive-decls")
                .action(clap::ArgAction::SetTrue)
                .help("Collect inductive declarations from Rust source")
        )
        .get_matches();

    let rust_src_path = matches.get_one::<String>("rust-src-path").unwrap();
    let output_dir = matches.get_one::<String>("output-dir").unwrap();

    if *matches.get_one::<bool>("inductive-decls").unwrap_or(&false) {
        collect_inductive_declarations(rust_src_path, output_dir)?;
    } else {
        println!("No specific action requested. Use --inductive-decls.");
    }

    Ok(())
}

fn collect_comprehensive_terms(rust_src_path: &str, output_dir: &str) -> Result<()> {
    // ... (collect_comprehensive_terms function)
    Ok(())
}

fn collect_monster_group_factors(rust_src_path: &str, output_dir: &str) -> Result<()> {
    // ... (collect_monster_group_factors function)
    Ok(())
}

fn run_standard_analysis(rust_src_path: &str, output_dir: &str, cache_path: Option<&String>) -> Result<()> {
    // ... (run_standard_analysis function)
    Ok(())
}

fn collect_inductive_declarations(rust_src_path: &str, output_dir: &str) -> Result<()> {
    println!("⚛️ Starting inductive declarations collection");

    // User-provided monster primes for sizing
    const USER_MONSTER_PRIMES: &[u64] = &[1, 2, 3, 5, 7, 11, 13, 13, 23, 31, 71];

    let mut scanner = FileScanner::new();
    let mut all_declarations: Vec<NixDeclaration> = Vec::new();

    // Create output directory for JSON and RocksDB
    let db_path = Path::new(output_dir).join("inductive_decls.rocksdb");
    std::fs::create_dir_all(&db_path)?; // Create directory for RocksDB
    
    // Open RocksDB
    let db = DB::open_default(&db_path)?;
    println!("Opened RocksDB at {:?}", &db_path);

    // Scan all Rust files
    let rust_files = scanner.find_rust_files(Path::new(rust_src_path))?;
    println!("Found {} Rust files", rust_files.len());

    let mut processed_files = 0;
    for file_path in rust_files.iter().take(100) { // Limit for proof
        if let Ok(content) = std::fs::read_to_string(&file_path) {
            if let Ok(syntax_tree) = syn::parse_file(&content) {
                let mut collector = InductiveDeclarationsCollector::new(
                    file_path.to_str().unwrap_or("unknown").to_string()
                );
                Visit::visit_file(&mut collector, &syntax_tree);
                collector.finalize_declarations(USER_MONSTER_PRIMES);
                all_declarations.extend(collector.declarations);
                processed_files += 1;
            }
        }
    }

    println!("✅ Processed {} files for inductive declarations", processed_files);
    println!("Total NixDeclarations collected: {}", all_declarations.len());

    // Store in RocksDB
    for decl in all_declarations {
        let key = format!("{}:{}:{:?}", decl.path, decl.name, decl.kind);
        let value = serde_json::to_vec(&decl)?;
        db.put(key, value)?;
    }
    println!("Stored declarations in RocksDB.");

    Ok(())
}


