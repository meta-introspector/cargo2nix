use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum DeclarationKind {
    Fn,
    Struct,
    Enum,
    Const,
    Static,
    Mod,
    Use,
    Type,
    Trait,
    Impl,
    LetBinding,
    Field,
    Variant,
    // Add more as needed
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct NixDeclaration {
    pub kind: DeclarationKind,
    pub name: String,
    pub path: String, // Format: file_path:line:col
    pub bit_size: Option<u64>,
    pub value: Option<String>,     // For literal values of Const, Static
    pub monster_factors: Vec<u64>, // List of monster primes derived from properties
}
