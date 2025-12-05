// crates/tool-refactorings/src/cargo_toml_patcher.rs

use crate::cargo_toml_patcher_config::CargoTomlPatcherConfig;
use gemini_utils::gemini_eprintln;
use std::fs;
use std::path::Path;
use toml_edit::{table, value, Document, DocumentMut, Item, Table};

/// Trait for patching `Cargo.toml` files.
pub trait CargoTomlPatcher {
    /// Patches a `Cargo.toml` file according to predefined rules (standardizing `cargo_metadata`,
    /// adding external and local dependencies with feature flags).
    ///
    /// `cargo_toml_path`: The path to the `Cargo.toml` file to patch.
    /// `dry_run`: If true, no changes are written to the file system.
    ///
    /// Returns a message indicating the outcome.
    fn patch_cargo_toml(&self, cargo_toml_path: &Path, dry_run: bool) -> Result<String, String>;
}

/// Dummy implementation of `CargoTomlPatcher` using `toml_edit` for structured modification.
pub struct DefaultCargoTomlPatcher<C: CargoTomlPatcherConfig> {
    config: C,
}

impl<C: CargoTomlPatcherConfig> DefaultCargoTomlPatcher<C> {
    pub fn new(config: C) -> Self {
        Self { config }
    }

    fn get_or_create_table_mut<'a>(doc: &'a mut toml_edit::DocumentMut, section: &str) -> &'a mut Table {
        if doc.get(section).is_none() {
            doc.insert(section, toml_edit::table());
        }
        doc[section].as_table_mut().expect("Section should be a table after creation/check")
    }

    fn add_dependency(doc: &mut toml_edit::DocumentMut, section: &str, name: &str, dep_str: &str) {
        let mut deps = Self::get_or_create_table_mut(doc, section);

        if deps.get(name).is_none() {
            let dep_item: Item = dep_str.parse().unwrap_or_else(|_| value(dep_str).into());
            deps.insert(name, dep_item);
        }
    }

    fn add_feature(doc: &mut toml_edit::DocumentMut, feature_name: &str, dep_name: &str) {
        let mut features = Self::get_or_create_table_mut(doc, "features");

        if features.get(feature_name).is_none() {
            features.insert(feature_name, value(toml_edit::Array::from_iter(vec![format!("dep:{}", dep_name)])));
        }
    }
}

impl<C: CargoTomlPatcherConfig> CargoTomlPatcher for DefaultCargoTomlPatcher<C> {
    fn patch_cargo_toml(&self, cargo_toml_path: &Path, dry_run: bool) -> Result<String, String> {
        gemini_eprintln!("Patching Cargo.toml: :path:", path = cargo_toml_path.display());

        let content = fs::read_to_string(cargo_toml_path)
            .map_err(|e| format!("Failed to read Cargo.toml {:?}: {}", cargo_toml_path, e))?;
        
        let mut doc = content.parse::<toml_edit::DocumentMut>()
            .map_err(|e| format!("Failed to parse Cargo.toml {:?}: {}", cargo_toml_path, e))?;

        // 1. Standardize cargo_metadata version
        if let Some(deps) = doc.get_mut("dependencies").and_then(|item| item.as_table_mut()) {
            if let Some(metadata_dep) = deps.get_mut("cargo_metadata") {
                // Check if it's not already in the desired format
                if !metadata_dep.as_table().map_or(false, |t| {
                    t.get("workspace").map_or(false, |v| v.as_value().map_or(false, |val| val.as_bool() == Some(true))) &&
                    t.get("optional").map_or(false, |v| v.as_value().map_or(false, |val| val.as_bool() == Some(true)))
                }) {
                    gemini_eprintln!("Standardizing cargo_metadata in :path:", path = cargo_toml_path.display());
                    let mut inline_table = toml_edit::InlineTable::new();
                    inline_table.insert("workspace", toml_edit::Value::Boolean(toml_edit::Formatted::new(true)));
                    inline_table.insert("optional", toml_edit::Value::Boolean(toml_edit::Formatted::new(true)));
                    *metadata_dep = Item::Value(toml_edit::Value::InlineTable(inline_table));
                }
            }
        }
        
        // 2. Add common external dependencies
        for (module_name, feature_name) in self.config.get_external_deps() {
            let dep_string = self.config.get_external_dep_string(module_name);
            Self::add_dependency(&mut doc, "dependencies", module_name, &dep_string);
            Self::add_feature(&mut doc, feature_name, module_name);
        }

        // 3. Add local dependencies
        for (module_name, feature_name) in self.config.get_local_deps() {
            let dep_string = self.config.get_local_dep_string(module_name);
            Self::add_dependency(&mut doc, "dependencies", module_name, &dep_string);
            Self::add_feature(&mut doc, feature_name, module_name);
        }

        let new_content = doc.to_string();

        if !dry_run {
            fs::write(cargo_toml_path, new_content)
                .map_err(|e| format!("Failed to write patched Cargo.toml {:?}: {}", cargo_toml_path, e))?;
            Ok(format!("Successfully patched {}", cargo_toml_path.display()))
        } else {
            Ok(format!("DRY-RUN: Would patch {}", cargo_toml_path.display()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cargo_toml_patcher_config::DefaultCargoTomlPatcherConfig;
    use tempfile::tempdir;

    #[test]
    fn test_patch_cargo_toml_standardize_metadata() {
        let dir = tempdir().unwrap();
        let cargo_toml_path = dir.path().join("Cargo.toml");
        fs::write(&cargo_toml_path, r#"[package]
name = "test-crate"
version = "0.1.0"

[dependencies]
cargo_metadata = "0.18.1"
"#).unwrap();

        let config = DefaultCargoTomlPatcherConfig;
        let patcher = DefaultCargoTomlPatcher::new(config);
        let result = patcher.patch_cargo_toml(&cargo_toml_path, false);
        assert!(result.is_ok());

        let content = fs::read_to_string(&cargo_toml_path).unwrap();
        let doc = content.parse::<toml_edit::DocumentMut>().unwrap();
        assert_eq!(doc["dependencies"]["cargo_metadata"].to_string().trim(), "{ workspace = true, optional = true }");
    }

    #[test]
    fn test_patch_cargo_toml_add_external_dep() {
        let dir = tempdir().unwrap();
        let cargo_toml_path = dir.path().join("Cargo.toml");
        fs::write(&cargo_toml_path, r#"[package]
name = "test-crate"
version = "0.1.0"

[dependencies]

[features]
"#).unwrap();

        let config = DefaultCargoTomlPatcherConfig;
        let patcher = DefaultCargoTomlPatcher::new(config);
        let result = patcher.patch_cargo_toml(&cargo_toml_path, false);
        assert!(result.is_ok());

        let content = fs::read_to_string(&cargo_toml_path).unwrap();
        let doc = content.parse::<toml_edit::DocumentMut>().unwrap();
        assert_eq!(doc["dependencies"]["anyhow"].to_string().trim(), "{ workspace = true, optional = true }");
        assert_eq!(doc["features"]["anyhow_enabled"].to_string().trim(), "[\"dep:anyhow\"]");
    }

    #[test]
    fn test_patch_cargo_toml_add_local_dep() {
        let dir = tempdir().unwrap();
        let cargo_toml_path = dir.path().join("Cargo.toml");
        fs::write(&cargo_toml_path, r#"[package]
name = "test-crate"
version = "0.1.0"

[dependencies]

[features]
"#).unwrap();

        let config = DefaultCargoTomlPatcherConfig;
        let patcher = DefaultCargoTomlPatcher::new(config);
        let result = patcher.patch_cargo_toml(&cargo_toml_path, false);
        assert!(result.is_ok());

        let content = fs::read_to_string(&cargo_toml_path).unwrap();
        let doc = content.parse::<toml_edit::DocumentMut>().unwrap();
        assert_eq!(doc["dependencies"]["cargo-edit-lib"].to_string().trim(), "{ path = \"../cargo-edit-lib\", optional = true }");
        assert_eq!(doc["features"]["cargo_edit_lib_enabled"].to_string().trim(), "[\"dep:cargo-edit-lib\"]");
    }

    #[test]
    fn test_patch_cargo_toml_dry_run() {
        let dir = tempdir().unwrap();
        let cargo_toml_path = dir.path().join("Cargo.toml");
        fs::write(&cargo_toml_path, r#"[package]
name = "test-crate"
version = "0.1.0"
"#).unwrap();

        let config = DefaultCargoTomlPatcherConfig;
        let patcher = DefaultCargoTomlPatcher::new(config);
        let result = patcher.patch_cargo_toml(&cargo_toml_path, true);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("DRY-RUN"));

        let content = fs::read_to_string(&cargo_toml_path).unwrap();
        // Content should not have changed in dry run
        assert!(!content.contains("[dependencies.anyhow]"));
    }
}
