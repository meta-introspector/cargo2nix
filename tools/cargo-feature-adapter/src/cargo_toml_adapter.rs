// tools/cargo-feature-adapter/src/cargo_toml_adapter.rs
use anyhow::{Context, Result};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use toml_edit::{Array, Document, Item, Table, Value};

pub fn adapt_cargo_toml(input_path: &Path, output_path: &Path) -> Result<()> {
    let original_cargo_toml_path = input_path.join("Cargo.toml");
    let adapted_cargo_toml_path = output_path.join("Cargo.toml");

    // 1. Read Original Cargo.toml
    let original_content = fs::read_to_string(&original_cargo_toml_path).context(format!(
        "Failed to read original Cargo.toml from {:?}",
        original_cargo_toml_path
    ))?;
    let mut doc = original_content
        .parse::<Document<_>>()
        .context("Failed to parse original Cargo.toml")?;

    // Ensure output directory exists
    fs::create_dir_all(output_path).context(format!(
        "Failed to create output directory {:?}",
        output_path
    ))?;

    // 2. Identify Dependencies and Generate New Features
    let mut new_features = BTreeMap::new();
    let mut default_features_list = Vec::new();

    // Process [dependencies]
    if let Some(Item::Table(dependencies)) = doc.get_mut("dependencies") {
        let mut deps_to_modify = Vec::new();
        for (key, value) in dependencies.iter() {
            deps_to_modify.push((key.to_string(), value.clone()));
        }

        for (key, value) in deps_to_modify {
            let feature_name = format!("with-{}", key.replace('-', "_"));
            let mut dep_table = Table::new();
            dep_table.insert("optional", Item::Value(Value::from(true)));

            match value {
                Item::Value(v) => {
                    if let Some(s) = v.as_str() {
                        dep_table.insert("version", Item::Value(Value::from(s)));
                    } else if let Some(inline_table) = v.as_inline_table() {
                        for (inline_key, inline_value) in inline_table.iter() {
                            dep_table.insert(inline_key, Item::Value(inline_value.clone()));
                        }
                    }
                }
                Item::Table(t) => {
                    for (inline_key, inline_value) in t.iter() {
                        dep_table.insert(inline_key, inline_value.clone());
                    }
                }
                _ => {} // Should not happen for dependencies
            }

            // Ensure optional is true
            dep_table.insert("optional", Item::Value(Value::from(true)));

            dependencies.insert(&key, Item::Table(dep_table));
            new_features.insert(
                feature_name.clone(),
                Item::Value(Value::Array(Array::from_iter(vec![Value::from(format!(
                    "dep:{}",
                    key
                ))]))),
            );
            default_features_list.push(feature_name);
        }
    }

    // Add [features] section if it doesn't exist
    if doc.get("features").is_none() {
        doc.insert("features", Item::Table(Table::new()));
    }

    if let Some(Item::Table(features_table)) = doc.get_mut("features") {
        for (feature_name, feature_deps) in new_features {
            features_table.insert(&feature_name, feature_deps);
        }
        // Set default features
        features_table.insert(
            "default",
            Item::Value(Value::Array(Array::from_iter(
                default_features_list.into_iter().map(Value::from),
            ))),
        );
    }

    // 3. Update package name
    if let Some(Item::Table(package_table)) = doc.get_mut("package") {
        if let Some(Item::Value(name_value)) = package_table.get_mut("name") {
            if let Some(s) = name_value.as_str() {
                let new_name = format!("{}-adaptive", s);
                *name_value = Value::from(new_name);
            }
        }
    }

    // 4. Write the new Cargo.toml
    fs::write(&adapted_cargo_toml_path, doc.to_string()).context(format!(
        "Failed to write adapted Cargo.toml to {:?}",
        adapted_cargo_toml_path
    ))?;

    println!(
        "Adapted Cargo.toml written to {:?}",
        adapted_cargo_toml_path
    );
    Ok(())
}
