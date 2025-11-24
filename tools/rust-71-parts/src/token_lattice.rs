//! Token Lattice: Hierarchical ordering by semantic complexity
//! Monster Group Factor: 71^1 = 71

use crate::ast_extractor::TokenKind;

/// Semantic complexity level for tokens
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemanticLevel(pub u8);

impl SemanticLevel {
    pub const PRIMITIVE: Self = Self(0);    // No semantic components
    pub const SIMPLE: Self = Self(1);       // 1 semantic component
    pub const BINARY: Self = Self(2);       // 2 semantic components
    pub const COMPLEX: Self = Self(3);      // 3+ semantic components
    pub const CONTROL: Self = Self(4);      // Control flow semantics
    pub const META: Self = Self(5);         // Meta-programming semantics
}

/// Token with semantic level in lattice hierarchy
#[derive(Debug, Clone)]
pub struct LatticeToken {
    pub kind: TokenKind,
    pub level: SemanticLevel,
    pub monster_factor: u64,
}

impl LatticeToken {
    pub fn new(kind: TokenKind) -> Self {
        let level = Self::compute_semantic_level(&kind);
        Self {
            kind,
            level,
            monster_factor: 71,
        }
    }
    
    /// Compute semantic complexity level
    fn compute_semantic_level(kind: &TokenKind) -> SemanticLevel {
        match kind {
            // Level 0: Primitives (no semantic components)
            TokenKind::Eof | TokenKind::Unknown => SemanticLevel::PRIMITIVE,
            
            // Level 1: Simple tokens (1 semantic component)
            TokenKind::Ident | TokenKind::True | TokenKind::False |
            TokenKind::OpenParen | TokenKind::CloseParen |
            TokenKind::OpenBrace | TokenKind::CloseBrace |
            TokenKind::OpenBracket | TokenKind::CloseBracket |
            TokenKind::Comma | TokenKind::Semi | TokenKind::Dot => SemanticLevel::SIMPLE,
            
            // Level 2: Binary operations (2 semantic components)
            TokenKind::Plus | TokenKind::Minus | TokenKind::Star | TokenKind::Slash |
            TokenKind::Percent | TokenKind::And | TokenKind::Or |
            TokenKind::Eq | TokenKind::EqEq | TokenKind::Ne |
            TokenKind::Lt | TokenKind::Gt | TokenKind::Le | TokenKind::Ge => SemanticLevel::BINARY,
            
            // Level 3: Complex operations (3+ semantic components)
            TokenKind::AndAnd | TokenKind::OrOr | TokenKind::Shl | TokenKind::Shr |
            TokenKind::PlusEq | TokenKind::MinusEq | TokenKind::StarEq |
            TokenKind::DotDot | TokenKind::DotDotEq | TokenKind::ModSep |
            TokenKind::RArrow | TokenKind::FatArrow => SemanticLevel::COMPLEX,
            
            // Level 4: Control flow (control semantics)
            TokenKind::If | TokenKind::Else | TokenKind::While | TokenKind::For |
            TokenKind::Loop | TokenKind::Match | TokenKind::Return |
            TokenKind::Break | TokenKind::Continue => SemanticLevel::CONTROL,
            
            // Level 5: Meta-programming (meta semantics)
            TokenKind::Fn | TokenKind::Let | TokenKind::Mut |
            TokenKind::Pound | TokenKind::Dollar | TokenKind::At => SemanticLevel::META,
            
            // Default to simple for literals and others
            _ => SemanticLevel::SIMPLE,
        }
    }
}

/// Token lattice with hierarchical ordering
pub struct TokenLattice {
    tokens: Vec<LatticeToken>,
}

impl TokenLattice {
    pub fn new() -> Self {
        Self { tokens: Vec::new() }
    }
    
    pub fn add(&mut self, kind: TokenKind) {
        self.tokens.push(LatticeToken::new(kind));
    }
    
    /// Get tokens at specific semantic level
    pub fn at_level(&self, level: SemanticLevel) -> Vec<&LatticeToken> {
        self.tokens.iter().filter(|t| t.level == level).collect()
    }
    
    /// Get tokens below or at level (≤)
    pub fn below_level(&self, level: SemanticLevel) -> Vec<&LatticeToken> {
        self.tokens.iter().filter(|t| t.level <= level).collect()
    }
    
    /// Sort tokens by semantic level (lattice order)
    pub fn sorted_by_level(&self) -> Vec<&LatticeToken> {
        let mut sorted: Vec<_> = self.tokens.iter().collect();
        sorted.sort_by_key(|t| t.level);
        sorted
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_semantic_levels() {
        let eof = LatticeToken::new(TokenKind::Eof);
        let plus = LatticeToken::new(TokenKind::Plus);
        let if_token = LatticeToken::new(TokenKind::If);
        
        assert_eq!(eof.level, SemanticLevel::PRIMITIVE);
        assert_eq!(plus.level, SemanticLevel::BINARY);
        assert_eq!(if_token.level, SemanticLevel::CONTROL);
    }
    
    #[test]
    fn test_lattice_ordering() {
        let mut lattice = TokenLattice::new();
        lattice.add(TokenKind::If);
        lattice.add(TokenKind::Plus);
        lattice.add(TokenKind::Eof);
        
        let sorted = lattice.sorted_by_level();
        assert!(sorted[0].level <= sorted[1].level);
        assert!(sorted[1].level <= sorted[2].level);
    }
}
