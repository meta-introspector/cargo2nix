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
    #[error("Custom error: {0}")]
    Custom(String),
}