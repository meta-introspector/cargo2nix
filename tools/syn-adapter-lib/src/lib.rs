use anyhow::{Context, Result};
use std::any::Any;
use std::fs;
use std::path::Path;

#[cfg(feature = "syn-parsing")]
use syn::{File, Item};

/// A trait for parsing Rust code, abstracting different parsing implementations.
pub trait SynAdapter: Send + Sync {
    /// Parses a Rust file from a given path into a `syn::File` (or equivalent AST).
    fn parse_file(&self, path: &Path) -> Result<File>;

    /// Parses a string containing Rust code into a `syn::File` (or equivalent AST).
    fn parse_str(&self, code: &str) -> Result<File>;

    /// Returns a reference to `Any` for downcasting.
    fn as_any(&self) -> &dyn Any;
}

/// Mock implementation of `SynAdapter` for dry-run or testing.
pub struct MockSynAdapter;

impl MockSynAdapter {
    pub fn new() -> Self {
        MockSynAdapter
    }
}

impl SynAdapter for MockSynAdapter {
    fn parse_file(&self, path: &Path) -> Result<File> {
        println!("[MockSynAdapter] Mock parsing file: {:?}", path);
        // Return a minimal, valid mock syn::File
        Ok(File {
            shebang: None,
            attrs: vec![],
            items: vec![],
        })
    }

    fn parse_str(&self, code: &str) -> Result<File> {
        println!("[MockSynAdapter] Mock parsing string: {}", code);
        // Return a minimal, valid mock syn::File
        Ok(File {
            shebang: None,
            attrs: vec![],
            items: vec![],
        })
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(feature = "syn-parsing")]
pub struct LibSynAdapter;

#[cfg(feature = "syn-parsing")]
impl LibSynAdapter {
    pub fn new() -> Self {
        LibSynAdapter
    }
}

#[cfg(feature = "syn-parsing")]
impl SynAdapter for LibSynAdapter {
    fn parse_file(&self, path: &Path) -> Result<File> {
        println!("[LibSynAdapter] Parsing file: {:?}", path);
        let content =
            fs::read_to_string(path).context(format!("Failed to read file {:?}", path))?;
        syn::parse_file(&content).context(format!("Failed to parse Rust file {:?}", path))
    }

    fn parse_str(&self, code: &str) -> Result<File> {
        println!("[LibSynAdapter] Parsing string: {}", code);
        syn::parse_file(code).context("Failed to parse Rust string")
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
