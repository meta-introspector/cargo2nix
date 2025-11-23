use proc_macro2::TokenStream;
use syn::File;

use std::path::Path;
use async_trait::async_trait;

mod error;
use crate::error::AppError;

/// Trait for parsing Rust code into an Abstract Syntax Tree (AST).
pub trait RustParser<E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn parse_str(&self, code: &str) -> Result<File, E>;
}

/// Trait for generating Rust code from an AST-like structure.
pub trait CodeGenerator<E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn generate_code(&self, ast: &File) -> Result<TokenStream, E>;
}

/// A combined processor that can parse and generate code.
pub trait CodeProcessor<E>: RustParser<E> + CodeGenerator<E>
where
    E: std::error::Error + Send + Sync + 'static,
{}

impl<T, E> CodeProcessor<E> for T
where
    T: RustParser<E> + CodeGenerator<E>,
    E: std::error::Error + Send + Sync + 'static,
{}

// --- New Traits for the Lattice of Features ---

/// Trait for abstracting I/O operations.
#[async_trait]
pub trait IoHandler<E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    async fn read_file(&self, path: &Path) -> Result<String, E>;
    async fn write_file(&self, path: &Path, content: &str) -> Result<(), E>;
    // Add other I/O related methods as needed (e.g., list_dir, create_dir)
}

/// Trait for abstracting serialization/deserialization.
pub trait SerdeHandler<E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn serialize<T: ?Sized + serde::Serialize>(&self, value: &T) -> Result<String, E>;
    fn deserialize<'de, T: serde::Deserialize<'de>>(&self, s: &'de str) -> Result<T, E>;
    // Add other serialization formats (e.g., JSON, YAML, TOML)
}

/// Trait for abstracting system calls (e.g., executing external commands).
#[async_trait]
pub trait SyscallHandler<E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    async fn execute_command(&self, command: &str, args: &[String]) -> Result<String, E>;
    // Add other syscall related methods (e.g., get_env, set_env)
}

/// Trait for abstracting interactions with Rust libraries (e.g., `syn`, `quote`).
pub trait LibHandler<E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    // This trait will be more specific to the kind of library.
    // For `syn` and `quote`, it might involve methods for AST manipulation.
}

/// Trait for abstracting shared object (SO) bindings/FFI.
pub trait SoBindingsHandler<E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    // Methods for loading dynamic libraries, calling foreign functions.
}

/// Trait for handling configuration loading and merging.
pub trait ConfigHandler<C>
where
    C: for<'de> serde::Deserialize<'de> + serde::Serialize + Default,
{
    fn load_config(&self, path: &Path) -> Result<C, AppError>;
    fn merge_configs(&mut self, base: C, overlay: C) -> C;
}

/// Trait for abstracting the Rust compiler invocation.
#[async_trait]
pub trait RustcCompiler<E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    async fn compile(
        &self,
        source_file: &Path,
        output_dir: &Path,
        crate_name: &str,
        crate_type: &str,
        edition: &str,
        extern_libs: &[String], // Paths to .rlib files for --extern
        cfg_flags: &[String], // --cfg flags for features
    ) -> Result<(), E>;
}
