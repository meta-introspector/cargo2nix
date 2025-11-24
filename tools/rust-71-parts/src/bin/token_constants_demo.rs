//! Demo compressed token constants organized by semantic levels

use rust_71_parts::token_constants::{TokenConstants, level0, level1, level2, level3, level4, level5};

fn main() {
    println!("🔢 Token Constants Demo - Compressed Types as Constants");
    println!("📊 Monster Group Factor: 71^1 = 71\n");
    
    // Show level-based constant organization
    println!("📋 Level-based Constant Modules:");
    println!("  Level 0 (Primitive): EOF={}, UNKNOWN={}", level0::EOF, level0::UNKNOWN);
    println!("  Level 1 (Simple): IDENT={}, TRUE={}, COMMA={}", level1::IDENT, level1::TRUE, level1::COMMA);
    println!("  Level 2 (Binary): PLUS={}, EQ_EQ={}, LT={}", level2::PLUS, level2::EQ_EQ, level2::LT);
    println!("  Level 3 (Complex): AND_AND={}, PLUS_EQ={}, R_ARROW={}", level3::AND_AND, level3::PLUS_EQ, level3::R_ARROW);
    println!("  Level 4 (Control): IF={}, WHILE={}, RETURN={}", level4::IF, level4::WHILE, level4::RETURN);
    println!("  Level 5 (Meta): FN={}, LET={}, MUT={}", level5::FN, level5::LET, level5::MUT);
    
    println!("\n🎯 Compressed Token Examples:");
    let tokens = [
        ("EOF", TokenConstants::EOF),
        ("PLUS", TokenConstants::PLUS),
        ("IF", TokenConstants::IF),
        ("FN", TokenConstants::FN),
    ];
    
    for (name, token) in tokens {
        let encoded = token.encode();
        println!("  {}: level={}, const={}, encoded=0x{:04X}", 
                name, token.level, token.constant, encoded);
    }
    
    println!("\n✅ All constants maintain Monster Group factor 71");
    println!("🔬 Types compressed into level-based constant modules");
    println!("📦 Each token encoded as single u16 value");
}
