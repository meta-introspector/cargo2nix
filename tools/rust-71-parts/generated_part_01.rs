#![doc = r" Part 1/71: LEX_TOKEN - Standalone implementation with no dependencies"]
#![doc = r" Extracted from rustc and regenerated using Monster Group theory"]
#![doc = r" Monster Group Factor: 71^1 = 71"]
#![no_std]
extern crate alloc;
use alloc::vec::Vec;
pub const MONSTER_PRIME: u64 = 71;
#[doc = r" Extracted from rustc: Basic token kinds for lexical analysis"]
#[doc = r" Monster Group Factor: 71^1 = 71"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Literal(LiteralKind),
    Ident,
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
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    Not,
    And,
    Or,
    AndAnd,
    OrOr,
    Shl,
    Shr,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    PercentEq,
    CaretEq,
    AndEq,
    OrEq,
    ShlEq,
    ShrEq,
    Eq,
    EqEq,
    Ne,
    Gt,
    Lt,
    Ge,
    Le,
    At,
    Underscore,
    Dot,
    DotDot,
    DotDotDot,
    DotDotEq,
    Comma,
    Semi,
    Colon,
    ModSep,
    RArrow,
    LArrow,
    FatArrow,
    Pound,
    Dollar,
    Question,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Eof,
    Unknown,
}
#[doc = r" Literal kinds extracted from rustc"]
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
    #[doc = r" Check if token is a keyword"]
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            TokenKind::Fn
                | TokenKind::Let
                | TokenKind::Mut
                | TokenKind::If
                | TokenKind::Else
                | TokenKind::While
                | TokenKind::For
                | TokenKind::Loop
                | TokenKind::Match
                | TokenKind::Return
                | TokenKind::Break
                | TokenKind::Continue
                | TokenKind::True
                | TokenKind::False
        )
    }
    #[doc = r" Check if token is an operator"]
    pub fn is_operator(&self) -> bool {
        matches!(
            self,
            TokenKind::Plus
                | TokenKind::Minus
                | TokenKind::Star
                | TokenKind::Slash
                | TokenKind::Percent
                | TokenKind::Caret
                | TokenKind::Not
                | TokenKind::And
                | TokenKind::Or
                | TokenKind::AndAnd
                | TokenKind::OrOr
                | TokenKind::Shl
                | TokenKind::Shr
                | TokenKind::PlusEq
                | TokenKind::MinusEq
                | TokenKind::StarEq
                | TokenKind::SlashEq
                | TokenKind::PercentEq
                | TokenKind::CaretEq
                | TokenKind::AndEq
                | TokenKind::OrEq
                | TokenKind::ShlEq
                | TokenKind::ShrEq
                | TokenKind::Eq
                | TokenKind::EqEq
                | TokenKind::Ne
                | TokenKind::Gt
                | TokenKind::Lt
                | TokenKind::Ge
                | TokenKind::Le
        )
    }
    #[doc = r" Check if token is a delimiter"]
    pub fn is_delimiter(&self) -> bool {
        matches!(
            self,
            TokenKind::OpenParen
                | TokenKind::CloseParen
                | TokenKind::OpenBrace
                | TokenKind::CloseBrace
                | TokenKind::OpenBracket
                | TokenKind::CloseBracket
        )
    }
}
#[doc = r" Token with Monster Group alignment (71^1)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub monster_factor: u64,
}
impl Token {
    #[doc = r" Create new token with Monster Group prime 71"]
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self {
            kind,
            span,
            monster_factor: 71,
        }
    }
    #[doc = r" Verify Monster Group constraint"]
    pub fn verify_monster_constraint(&self) -> bool {
        self.monster_factor == 71
    }
}
#[doc = r" Source span for token location"]
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
#[doc = r" Token stream for lexical analysis"]
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
    #[doc = r" Verify all tokens satisfy Monster Group constraint"]
    pub fn verify_all_constraints(&self) -> bool {
        self.tokens.iter().all(|t| t.verify_monster_constraint())
    }
}
