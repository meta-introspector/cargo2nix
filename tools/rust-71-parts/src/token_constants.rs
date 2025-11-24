//! Token Constants: Compress token types into level-based constant modules
//! Monster Group Factor: 71^1 = 71

/// Level 0: Primitive Constants (no semantic components)
pub mod level0 {
    pub const EOF: u8 = 0;
    pub const UNKNOWN: u8 = 1;
    pub const LEVEL_FACTOR: u64 = 71;
}

/// Level 1: Simple Constants (1 semantic component)
pub mod level1 {
    pub const IDENT: u8 = 0;
    pub const TRUE: u8 = 1;
    pub const FALSE: u8 = 2;
    pub const OPEN_PAREN: u8 = 3;
    pub const CLOSE_PAREN: u8 = 4;
    pub const OPEN_BRACE: u8 = 5;
    pub const CLOSE_BRACE: u8 = 6;
    pub const OPEN_BRACKET: u8 = 7;
    pub const CLOSE_BRACKET: u8 = 8;
    pub const COMMA: u8 = 9;
    pub const SEMI: u8 = 10;
    pub const DOT: u8 = 11;
    pub const LEVEL_FACTOR: u64 = 71;
}

/// Level 2: Binary Constants (2 semantic components)
pub mod level2 {
    pub const PLUS: u8 = 0;
    pub const MINUS: u8 = 1;
    pub const STAR: u8 = 2;
    pub const SLASH: u8 = 3;
    pub const PERCENT: u8 = 4;
    pub const AND: u8 = 5;
    pub const OR: u8 = 6;
    pub const EQ: u8 = 7;
    pub const EQ_EQ: u8 = 8;
    pub const NE: u8 = 9;
    pub const LT: u8 = 10;
    pub const GT: u8 = 11;
    pub const LE: u8 = 12;
    pub const GE: u8 = 13;
    pub const LEVEL_FACTOR: u64 = 71;
}

/// Level 3: Complex Constants (3+ semantic components)
pub mod level3 {
    pub const AND_AND: u8 = 0;
    pub const OR_OR: u8 = 1;
    pub const SHL: u8 = 2;
    pub const SHR: u8 = 3;
    pub const PLUS_EQ: u8 = 4;
    pub const MINUS_EQ: u8 = 5;
    pub const STAR_EQ: u8 = 6;
    pub const DOT_DOT: u8 = 7;
    pub const DOT_DOT_EQ: u8 = 8;
    pub const MOD_SEP: u8 = 9;
    pub const R_ARROW: u8 = 10;
    pub const FAT_ARROW: u8 = 11;
    pub const LEVEL_FACTOR: u64 = 71;
}

/// Level 4: Control Constants (control flow semantics)
pub mod level4 {
    pub const IF: u8 = 0;
    pub const ELSE: u8 = 1;
    pub const WHILE: u8 = 2;
    pub const FOR: u8 = 3;
    pub const LOOP: u8 = 4;
    pub const MATCH: u8 = 5;
    pub const RETURN: u8 = 6;
    pub const BREAK: u8 = 7;
    pub const CONTINUE: u8 = 8;
    pub const LEVEL_FACTOR: u64 = 71;
}

/// Level 5: Meta Constants (meta-programming semantics)
pub mod level5 {
    pub const FN: u8 = 0;
    pub const LET: u8 = 1;
    pub const MUT: u8 = 2;
    pub const POUND: u8 = 3;
    pub const DOLLAR: u8 = 4;
    pub const AT: u8 = 5;
    pub const LEVEL_FACTOR: u64 = 71;
}

/// Compressed token representation using level + constant
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompressedToken {
    pub level: u8,
    pub constant: u8,
    pub monster_factor: u64,
}

impl CompressedToken {
    pub const fn new(level: u8, constant: u8) -> Self {
        Self {
            level,
            constant,
            monster_factor: 71,
        }
    }
    
    /// Encode token as single u16: level (3 bits) + constant (8 bits)
    pub const fn encode(&self) -> u16 {
        ((self.level as u16) << 8) | (self.constant as u16)
    }
    
    /// Decode u16 back to compressed token
    pub const fn decode(encoded: u16) -> Self {
        Self::new((encoded >> 8) as u8, (encoded & 0xFF) as u8)
    }
}

/// Token constant factory
pub struct TokenConstants;

impl TokenConstants {
    // Level 0 constructors
    pub const EOF: CompressedToken = CompressedToken::new(0, level0::EOF);
    pub const UNKNOWN: CompressedToken = CompressedToken::new(0, level0::UNKNOWN);
    
    // Level 1 constructors
    pub const IDENT: CompressedToken = CompressedToken::new(1, level1::IDENT);
    pub const TRUE: CompressedToken = CompressedToken::new(1, level1::TRUE);
    pub const FALSE: CompressedToken = CompressedToken::new(1, level1::FALSE);
    
    // Level 2 constructors
    pub const PLUS: CompressedToken = CompressedToken::new(2, level2::PLUS);
    pub const MINUS: CompressedToken = CompressedToken::new(2, level2::MINUS);
    pub const EQ_EQ: CompressedToken = CompressedToken::new(2, level2::EQ_EQ);
    
    // Level 3 constructors
    pub const AND_AND: CompressedToken = CompressedToken::new(3, level3::AND_AND);
    pub const PLUS_EQ: CompressedToken = CompressedToken::new(3, level3::PLUS_EQ);
    
    // Level 4 constructors
    pub const IF: CompressedToken = CompressedToken::new(4, level4::IF);
    pub const WHILE: CompressedToken = CompressedToken::new(4, level4::WHILE);
    
    // Level 5 constructors
    pub const FN: CompressedToken = CompressedToken::new(5, level5::FN);
    pub const LET: CompressedToken = CompressedToken::new(5, level5::LET);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_compression() {
        let token = TokenConstants::PLUS;
        let encoded = token.encode();
        let decoded = CompressedToken::decode(encoded);
        
        assert_eq!(token, decoded);
        assert_eq!(token.level, 2);
        assert_eq!(token.constant, level2::PLUS);
    }
    
    #[test]
    fn test_monster_factor() {
        assert_eq!(TokenConstants::FN.monster_factor, 71);
        assert_eq!(level0::LEVEL_FACTOR, 71);
        assert_eq!(level5::LEVEL_FACTOR, 71);
    }
}
