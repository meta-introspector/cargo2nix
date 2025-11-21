use anyhow::{Context, Result};
use std::collections::HashSet; // Import HashSet
use std::fs;
use std::path::Path;
use toml_edit::{DocumentMut, Item, RawString};

pub fn patch_cargo_toml(
    cargo_toml_path: &Path,
    _submodule_name: &str, // submodule_name is not directly used in this revised logic
    vendored_crates: &HashSet<String>, // Changed to HashSet
) -> Result<()> {
    println!("Patching Cargo.toml: {:?}", cargo_toml_path);

    let content = fs::read_to_string(cargo_toml_path)
        .with_context(|| format!("Failed to read Cargo.toml at {:?}", cargo_toml_path))?;

    let mut doc = content
        .parse::<DocumentMut>()
        .with_context(|| format!("Failed to parse Cargo.toml at {:?}", cargo_toml_path))?;

    // Comment out rust-version
    if let Some(package_table) = doc.get_mut("package").and_then(|item| item.as_table_mut()) {
        if let Some(rust_version_item) = package_table.get_mut("rust-version") {
                if rust_version_item.is_value() {
                    let decor = rust_version_item.as_value_mut().unwrap().decor_mut();
                    let current_prefix_raw = decor.prefix().map_or_else(|| RawString::from(""), |s| s.clone());
                    let mut current_prefix_string = current_prefix_raw.as_str().unwrap_or("").to_string();
                    current_prefix_string.insert_str(0, "# ");
                    let new_prefix_raw = RawString::from(current_prefix_string);
                    decor.set_prefix(new_prefix_raw);
                    println!("  - Commented out rust-version in {:?}", cargo_toml_path);
                }
        }
    }

    // Process dependencies sections
    let sections = ["dependencies", "dev-dependencies", "build-dependencies"];
    for section_name in sections {
        if let Some(deps_table) = doc.get_mut(section_name).and_then(|item| item.as_table_mut()) {
            for (key, item) in deps_table.iter_mut() {
                let dep_name = key.to_string();
                if vendored_crates.contains(&dep_name) {
                    // If it's a simple version string (e.g., `foo = "1.0"`), change it to `foo.workspace = true`.
                    if item.is_value() && item.as_value().map_or(false, |v| v.is_str()) {
                        *item = toml_edit::Value::InlineTable(toml_edit::InlineTable::new()).into();
                        if let Some(dep_table) = item.as_inline_table_mut() {
                            dep_table.insert("workspace", true.into());
                            println!("  - Modified simple dependency '{}' to use workspace = true in {:?}", dep_name, cargo_toml_path);
                        }
                    }
                    // If it's a table (e.g., `foo = { version = "1.0" }` or `foo = { path = "../foo" }`),
                    // remove `version`, `git`, `branch`, `path` keys, and add `workspace = true`.
                    else if let Some(dep_table) = item.as_table_mut() {
                        dep_table.remove("version");
                        dep_table.remove("git");
                        dep_table.remove("branch");
                        dep_table.remove("path"); // Remove explicit path
                        dep_table.insert("workspace", Item::Value(true.into()));
                        println!("  - Modified table dependency '{}' to use workspace = true in {:?}", dep_name, cargo_toml_path);
                    }
                    // If it's an inline table (e.g., `foo = { version = "1.0" }`),
                    // remove `version`, `git`, `branch`, `path` keys, and add `workspace = true`.
                    else if let Some(dep_table) = item.as_inline_table_mut() {
                        dep_table.remove("version");
                        dep_table.remove("git");
                        dep_table.remove("branch");
                        dep_table.remove("path"); // Remove explicit path
                        dep_table.insert("workspace", true.into());
                        println!("  - Modified inline table dependency '{}' to use workspace = true in {:?}", dep_name, cargo_toml_path);
                    }
                }
            }
        }
    }

    fs::write(cargo_toml_path, doc.to_string())
        .with_context(|| format!("Failed to write patched Cargo.toml to {:?}", cargo_toml_path))?;

    Ok(())
}