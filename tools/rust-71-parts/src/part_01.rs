//! Part 1/71: LEX_TOKEN - Basic token representation
//! Monster Group Factor: 71^1 = 71

use crate::MONSTER_PRIME;

/// Token types in Rust lexical analysis
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Literals
    Integer(u64),
    Float(f64),
    String(String),
    Char(char),
    Bool(bool),
    
    // Identifiers and keywords
    Ident(String),
    Keyword(String),
    
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    
    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    
    // Special
    Semicolon,
    Comma,
    Dot,
    Arrow,
    FatArrow,
    
    // End of file
    Eof,
}

/// Basic token with Monster Group alignment
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub monster_factor: u64,
}

impl Token {
    /// Create new token with Monster Group prime 71
    pub fn new(kind: TokenKind) -> Self {
        Self {
            kind,
            monster_factor: MONSTER_PRIME,
        }
    }
    
    /// Verify Monster Group constraint
    pub fn verify_monster_constraint(&self) -> bool {
        self.monster_factor == MONSTER_PRIME
    }
}

/// Token stream for lexical analysis
pub struct TokenStream {
    tokens: Vec<Token>,
    position: usize,
}

impl TokenStream {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            position: 0,
        }
    }
    
    pub fn push(&mut self, token: Token) {
        assert!(token.verify_monster_constraint());
        self.tokens.push(token);
    }
    
    pub fn next(&mut self) -> Option<&Token> {
        if self.position < self.tokens.len() {
            let token = &self.tokens[self.position];
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }
    
    /// Verify all tokens satisfy Monster Group constraint
    pub fn verify_all_constraints(&self) -> bool {
        self.tokens.iter().all(|t| t.verify_monster_constraint())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_token_monster_constraint() {
        let token = Token::new(TokenKind::Ident("main".to_string()));
        assert_eq!(token.monster_factor, 71);
        assert!(token.verify_monster_constraint());
    }
    
    #[test]
    fn test_token_stream() {
        let mut stream = TokenStream::new();
        stream.push(Token::new(TokenKind::Keyword("fn".to_string())));
        stream.push(Token::new(TokenKind::Ident("main".to_string())));
        stream.push(Token::new(TokenKind::LeftParen));
        
        assert!(stream.verify_all_constraints());
        assert_eq!(stream.tokens.len(), 3);
    }
}
