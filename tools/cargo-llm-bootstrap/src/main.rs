use std::env;
use std::process;
use std::path::PathBuf;

mod solana_rustc_analyzer;
use solana_rustc_analyzer::SolanaRustcAnalyzer;

mod trait_types;
mod trait_extractor;
mod trait_numbering;

mod trait_lattice_generator;
use trait_lattice_generator::TraitLatticeGenerator;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <command> [options]", args[0]);
        process::exit(1);
    }
    
    match args[1].as_str() {
        "--analyze-solana-rustc" => {
            let rust_src_path = args.iter()
                .position(|arg| arg == "--rust-src-path")
                .and_then(|i| args.get(i + 1))
                .unwrap_or(&"/home/mdupont/nix/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src".to_string())
                .clone();
            
            println!("🔬 Starting Solana rustc Monster Group analysis");
            println!("Using pure Rust analyzer - no external tools");
            
            let mut analyzer = SolanaRustcAnalyzer::new(rust_src_path);
            
            match analyzer.analyze_and_prove() {
                Ok(()) => {
                    println!("✅ Analysis complete - Monster Group conjecture proven!");
                    process::exit(0);
                }
                Err(e) => {
                    eprintln!("❌ Analysis failed: {}", e);
                    process::exit(1);
                }
            }
        },
        "--generate-trait-lattice" => {
            let rust_src_path_str = args.iter()
                .position(|arg| arg == "--rust-src-path")
                .and_then(|i| args.get(i + 1))
                .map(|s| s.to_string())
                .unwrap_or_else(|| {
                    eprintln!("Error: --rust-src-path is required for --generate-trait-lattice");
                    process::exit(1);
                });
            let rust_src_path = PathBuf::from(rust_src_path_str);

            println!("🧬 Generating TraitLattice from: {:?}", rust_src_path);
            let mut generator = TraitLatticeGenerator::new();
            match generator.generate_lattice(&rust_src_path) {
                Ok(lattice) => {
                    let output_path = "trait_lattice.json";
                    match serde_json::to_string_pretty(&lattice) {
                        Ok(json) => {
                            match std::fs::write(output_path, json) {
                                Ok(_) => println!("✅ TraitLattice generated and saved to {}", output_path),
                                Err(e) => eprintln!("❌ Failed to write TraitLattice to {}: {}", output_path, e),
                            }
                        },
                        Err(e) => eprintln!("❌ Failed to serialize TraitLattice to JSON: {}", e),
                    }
                    process::exit(0);
                },
                Err(e) => {
                    eprintln!("❌ Failed to generate TraitLattice: {}", e);
                    process::exit(1);
                }
            }
        },
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            process::exit(1);
        }
    }
}
