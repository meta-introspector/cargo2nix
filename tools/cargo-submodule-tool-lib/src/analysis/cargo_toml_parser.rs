use super::super::traits::cargo_toml_parser::CargoTomlParser;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

#[cfg(feature = "toml_edit_enabled")]
use toml_edit::{Document, Item};

#[cfg(feature = "toml_edit_enabled")]
pub struct RealCargoTomlParser;

#[cfg(feature = "toml_edit_enabled")]
impl CargoTomlParser for RealCargoTomlParser {
    fn get_package_repository(&self, path: &Path) -> Result<Option<String>> {
        let cargo_toml_content = fs::read_to_string(path)
            .context(format!("Failed to read Cargo.toml from {:?}", path))?;
        let doc = cargo_toml_content
            .parse::<Document<_>>()
            .context("Failed to parse Cargo.toml")?;

        let repo_url = doc
            .get("package")
            .and_then(Item::as_table)
            .and_then(|p| p.get("repository"))
            .and_then(Item::as_str)
            .map(|s| s.to_string());

        Ok(repo_url)
    }
}

#[cfg(not(feature = "toml_edit_enabled"))]
pub struct DummyCargoTomlParser;

#[cfg(not(feature = "toml_edit_enabled"))]
impl CargoTomlParser for DummyCargoTomlParser {
    fn get_package_repository(&self, _path: &Path) -> Result<Option<String>> {
        Ok(None) // Dummy implementation returns None
    }
}

#[cfg(feature = "toml_edit_enabled")]
pub type CurrentCargoTomlParser = RealCargoTomlParser;
#[cfg(not(feature = "toml_edit_enabled"))]
pub type CurrentCargoTomlParser = DummyCargoTomlParser;
