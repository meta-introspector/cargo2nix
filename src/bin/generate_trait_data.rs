use std::path::PathBuf;
use cargo2nix::{BlockAnalyzer, extract_traits_from_blocks, generate_monster_data_file};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut analyzer = BlockAnalyzer::new();
    analyzer.load_compiler_blocks("rustc_blocks.db")?;
    analyzer.analyze_tool_blocks(".")?;
    
    let extracted_traits = extract_traits_from_blocks(&analyzer);
    let output_path = PathBuf::from("trait_data.dzn");
    
    generate_monster_data_file(extracted_traits, &output_path)?;
    println!("Generated trait data: {}", output_path.display());
    
    Ok(())
}
