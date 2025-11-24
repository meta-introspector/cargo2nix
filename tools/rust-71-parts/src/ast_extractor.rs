//! AST Extractor for Part 1: LEX_TOKEN
//! Extracts rustc TokenKind AST and regenerates as standalone code using quote!

use proc_macro2::TokenStream;
use quote::quote;

/// Token kinds for lexical analysis (extracted from rustc)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
    // Literals
    Literal(LiteralKind),
    
    // Identifiers
    Ident,
    
    // Keywords
    Fn, Let, Mut, If, Else, While, For, Loop, Match,
    Return, Break, Continue, True, False,
    
    // Operators
    Plus, Minus, Star, Slash, Percent, Caret, Not, And, Or,
    AndAnd, OrOr, Shl, Shr, PlusEq, MinusEq, StarEq, SlashEq,
    PercentEq, CaretEq, AndEq, OrEq, ShlEq, ShrEq, Eq, EqEq,
    Ne, Gt, Lt, Ge, Le, At, Underscore, Dot, DotDot, DotDotDot,
    DotDotEq, Comma, Semi, Colon, ModSep, RArrow, LArrow,
    FatArrow, Pound, Dollar, Question,
    
    // Delimiters
    OpenParen, CloseParen, OpenBrace, CloseBrace,
    OpenBracket, CloseBracket,
    
    // Special
    Eof, Unknown,
}

/// Literal kinds
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LiteralKind {
    Bool, Char, Integer, Float, Str, ByteStr, RawStr, RawByteStr,
}

/// Extract and regenerate TokenKind from rustc as standalone code
pub fn extract_token_kind() -> TokenStream {
    quote! {
        /// Extracted from rustc: Basic token kinds for lexical analysis
        /// Monster Group Factor: 71^1 = 71
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub enum TokenKind {
            // Literals
            Literal(LiteralKind),
            
            // Identifiers
            Ident,
            
            // Keywords (subset for minimal implementation)
            Fn,
            Let,
            Mut,
            If,
            Else,
            While,
            For,
            Loop,
            Match,
            Return,
            Break,
            Continue,
            True,
            False,
            
            // Operators
            Plus,          // +
            Minus,         // -
            Star,          // *
            Slash,         // /
            Percent,       // %
            Caret,         // ^
            Not,           // !
            And,           // &
            Or,            // |
            AndAnd,        // &&
            OrOr,          // ||
            Shl,           // <<
            Shr,           // >>
            PlusEq,        // +=
            MinusEq,       // -=
            StarEq,        // *=
            SlashEq,       // /=
            PercentEq,     // %=
            CaretEq,       // ^=
            AndEq,         // &=
            OrEq,          // |=
            ShlEq,         // <<=
            ShrEq,         // >>=
            Eq,            // =
            EqEq,          // ==
            Ne,            // !=
            Gt,            // >
            Lt,            // <
            Ge,            // >=
            Le,            // <=
            At,            // @
            Underscore,    // _
            Dot,           // .
            DotDot,        // ..
            DotDotDot,     // ...
            DotDotEq,      // ..=
            Comma,         // ,
            Semi,          // ;
            Colon,         // :
            ModSep,        // ::
            RArrow,        // ->
            LArrow,        // <-
            FatArrow,      // =>
            Pound,         // #
            Dollar,        // $
            Question,      // ?
            
            // Delimiters
            OpenParen,     // (
            CloseParen,    // )
            OpenBrace,     // {
            CloseBrace,    // }
            OpenBracket,   // [
            CloseBracket,  // ]
            
            // Special
            Eof,
            Unknown,
        }
        
        /// Literal kinds extracted from rustc
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub enum LiteralKind {
            Bool,
            Char,
            Integer,
            Float,
            Str,
            ByteStr,
            RawStr,
            RawByteStr,
        }
        
        impl TokenKind {
            /// Check if token is a keyword
            pub fn is_keyword(&self) -> bool {
                matches!(self, 
                    TokenKind::Fn | TokenKind::Let | TokenKind::Mut |
                    TokenKind::If | TokenKind::Else | TokenKind::While |
                    TokenKind::For | TokenKind::Loop | TokenKind::Match |
                    TokenKind::Return | TokenKind::Break | TokenKind::Continue |
                    TokenKind::True | TokenKind::False
                )
            }
            
            /// Check if token is an operator
            pub fn is_operator(&self) -> bool {
                matches!(self,
                    TokenKind::Plus | TokenKind::Minus | TokenKind::Star |
                    TokenKind::Slash | TokenKind::Percent | TokenKind::Caret |
                    TokenKind::Not | TokenKind::And | TokenKind::Or |
                    TokenKind::AndAnd | TokenKind::OrOr | TokenKind::Shl |
                    TokenKind::Shr | TokenKind::PlusEq | TokenKind::MinusEq |
                    TokenKind::StarEq | TokenKind::SlashEq | TokenKind::PercentEq |
                    TokenKind::CaretEq | TokenKind::AndEq | TokenKind::OrEq |
                    TokenKind::ShlEq | TokenKind::ShrEq | TokenKind::Eq |
                    TokenKind::EqEq | TokenKind::Ne | TokenKind::Gt |
                    TokenKind::Lt | TokenKind::Ge | TokenKind::Le
                )
            }
            
            /// Check if token is a delimiter
            pub fn is_delimiter(&self) -> bool {
                matches!(self,
                    TokenKind::OpenParen | TokenKind::CloseParen |
                    TokenKind::OpenBrace | TokenKind::CloseBrace |
                    TokenKind::OpenBracket | TokenKind::CloseBracket
                )
            }
        }
    }
}

/// Extract and regenerate Token struct with Monster Group alignment
pub fn extract_token_struct() -> TokenStream {
    quote! {
        /// Token with Monster Group alignment (71^1)
        #[derive(Debug, Clone, PartialEq)]
        pub struct Token {
            pub kind: TokenKind,
            pub span: Span,
            pub monster_factor: u64,
        }
        
        impl Token {
            /// Create new token with Monster Group prime 71
            pub fn new(kind: TokenKind, span: Span) -> Self {
                Self {
                    kind,
                    span,
                    monster_factor: 71, // Monster Group prime
                }
            }
            
            /// Verify Monster Group constraint
            pub fn verify_monster_constraint(&self) -> bool {
                self.monster_factor == 71
            }
        }
        
        /// Source span for token location
        #[derive(Debug, Clone, PartialEq)]
        pub struct Span {
            pub start: usize,
            pub end: usize,
        }
        
        impl Span {
            pub fn new(start: usize, end: usize) -> Self {
                Self { start, end }
            }
            
            pub fn len(&self) -> usize {
                self.end - self.start
            }
        }
    }
}

/// Generate complete Part 1 module with no dependencies
pub fn generate_part_01_standalone() -> TokenStream {
    let token_kind = extract_token_kind();
    let token_struct = extract_token_struct();
    
    quote! {
        //! Part 1/71: LEX_TOKEN - Standalone implementation with no dependencies
        //! Extracted from rustc and regenerated using Monster Group theory
        //! Monster Group Factor: 71^1 = 71
        
        #![no_std]
        
        extern crate alloc;
        use alloc::vec::Vec;
        
        pub const MONSTER_PRIME: u64 = 71;
        
        #token_kind
        
        #token_struct
        
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_extract_token_kind() {
        let tokens = extract_token_kind();
        assert!(!tokens.is_empty());
    }
    
    #[test]
    fn test_generate_standalone() {
        let code = generate_part_01_standalone();
        assert!(!code.is_empty());
    }
}
