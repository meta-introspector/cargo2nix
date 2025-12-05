use thiserror::Error;
use std::io;
use toml::de::Error as TomlDeError;
use syn::Error as SynError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("TOML deserialization error: {0}")]
    TomlDe(#[from] TomlDeError),
    #[error("Syn error: {0}")]
    Syn(#[from] SynError),
    #[error("Serde JSON error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Walkdir error: {0}")]
    Walkdir(#[from] walkdir::Error),
    #[error("Custom error: {0}")]
    Custom(String),
    #[error("Compilation error: {0}")]
    CompilationError(String),
}

impl From<String> for AppError {
    fn from(err: String) -> Self {
        AppError::Custom(err)
    }
}