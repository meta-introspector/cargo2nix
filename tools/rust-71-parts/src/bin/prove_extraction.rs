//! Proof of Concept: Extract real traits from rustc and verify Monster Group mapping

use rust_71_parts::verified_trait_extractor::VerifiedTraitExtractor;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <rust_source_file>", args[0]);
        eprintln!("Example: {} /path/to/rustc/src/lib.rs");
        return Ok(());
    }
    
    let file_path = &args[1];
    let mut extractor = VerifiedTraitExtractor::new();
    
    println!("Extracting traits from: {}", file_path);
    
    match extractor.extract_from_file(file_path) {
        Ok(()) => {
            println!("✓ Successfully extracted traits and features");
            println!("✓ Found {} traits", extractor.verified_traits.len());
            println!("✓ Found {} features", extractor.verified_features.len());
            
            if extractor.verify_monster_constraints() {
                println!("✓ All Monster Group constraints satisfied");
            } else {
                println!("✗ Monster Group constraints violated");
            }
            
            // Generate and print proof report
            let report = extractor.generate_proof_report();
            println!("\n{}", report);
            
            // Save proof to file
            std::fs::write("extraction_proof.md", report)?;
            println!("✓ Proof report saved to extraction_proof.md");
            
        }
        Err(e) => {
            eprintln!("✗ Extraction failed: {}", e);
            return Err(e);
        }
    }
    
    Ok(())
}
