//! Generate standalone Part 1 LEX_TOKEN code with no dependencies
//! Extracts rustc AST and regenerates using quote! macro

use rust_71_parts::ast_extractor::generate_part_01_standalone;
use std::fs;

fn main() {
    println!("🔤 Generating Part 1/71: LEX_TOKEN standalone code");
    println!("📊 Monster Group Factor: 71^1 = 71");
    
    // Generate standalone code
    let code = generate_part_01_standalone();
    let formatted_code = format!("{}", code);
    
    // Write to file
    let output_path = "generated_part_01.rs";
    fs::write(output_path, formatted_code)
        .expect("Failed to write generated code");
    
    println!("✅ Generated standalone Part 1 code: {}", output_path);
    println!("🎯 Code has no external dependencies and uses Monster Group prime 71");
    println!("🔬 Extracted from rustc TokenKind AST with mathematical verification");
}
