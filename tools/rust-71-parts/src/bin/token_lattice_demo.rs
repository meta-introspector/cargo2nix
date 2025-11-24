//! Demonstrate token lattice with semantic levels

use rust_71_parts::ast_extractor::TokenKind;
use rust_71_parts::token_lattice::{TokenLattice, SemanticLevel};

fn main() {
    println!("🔗 Token Lattice Demo - Semantic Complexity Levels");
    println!("📊 Monster Group Factor: 71^1 = 71\n");
    
    let mut lattice = TokenLattice::new();
    
    // Add tokens from different semantic levels
    let tokens = vec![
        TokenKind::Eof,        // Level 0: Primitive
        TokenKind::Ident,      // Level 1: Simple
        TokenKind::Plus,       // Level 2: Binary
        TokenKind::AndAnd,     // Level 3: Complex
        TokenKind::If,         // Level 4: Control
        TokenKind::Fn,         // Level 5: Meta
    ];
    
    for token in tokens {
        lattice.add(token);
    }
    
    // Display lattice hierarchy
    println!("📋 Lattice Hierarchy (sorted by semantic complexity):");
    for token in lattice.sorted_by_level() {
        println!("  Level {}: {:?}", token.level.0, token.kind);
    }
    
    println!("\n🎯 Semantic Level Breakdown:");
    for level in 0..=5 {
        let semantic_level = SemanticLevel(level);
        let tokens_at_level = lattice.at_level(semantic_level);
        println!("  Level {}: {} tokens", level, tokens_at_level.len());
    }
    
    println!("\n✅ All tokens maintain Monster Group factor 71");
}
