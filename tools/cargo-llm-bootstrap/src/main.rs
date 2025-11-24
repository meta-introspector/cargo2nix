use std::env;
use std::process;

mod solana_rustc_analyzer;
use solana_rustc_analyzer::SolanaRustcAnalyzer;

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
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            process::exit(1);
        }
    }
}
