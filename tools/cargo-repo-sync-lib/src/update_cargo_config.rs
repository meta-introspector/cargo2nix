use crate::RepoAction;
//use crate::Args;
use crate::Path;
//use crate::Command;
use anyhow::Context;
use std::fs;


pub fn update_cargo_config(actions_plan: &[RepoAction], root_dir: &Path) -> Result<()> {
    println!("Updating .cargo/config.toml...");
    let cargo_config_dir = root_dir.join(".cargo");
    fs::create_dir_all(&cargo_config_dir).context("Failed to create .cargo directory")?;
    let cargo_config_path = cargo_config_dir.join("config.toml");

    let mut config_doc = if cargo_config_path.exists() {
        let content = fs::read_to_string(&cargo_config_path)
            .with_context(|| format!("Failed to read {:?}", cargo_config_path))?;
        content
            .parse::<toml_edit::DocumentMut>()
            .context("Failed to parse .cargo/config.toml")?
    } else {
        "".parse::<toml_edit::DocumentMut>().context("Failed to create empty Document")?
    };

    // Ensure [patch.crates-io] section exists
    let patch_crates_io = config_doc
        .entry("patch")
        .or_insert(toml_edit::table())
        .as_table_mut()
        .context("patch entry is not a table")?
        .entry("crates-io")
        .or_insert(toml_edit::table())
        .as_table_mut()
        .context("crates-io entry is not a table")?;

    for action in actions_plan {
        let relative_submodule_path = pathdiff::diff_paths(&action.submodule_path, root_dir)
            .context(format!("Failed to get relative path for {:?}", action.submodule_path))?;
        let path_str = relative_submodule_path.to_string_lossy().to_string();

        // Add/update entry for this crate
        let mut crate_entry_table = toml_edit::Table::new();
        crate_entry_table.insert("path", toml_edit::value(path_str));
        patch_crates_io.insert(
            &action.repo_name,
            toml_edit::Item::Table(crate_entry_table),
        );
    }

    fs::write(&cargo_config_path, config_doc.to_string())
        .with_context(|| format!("Failed to write to {:?}", cargo_config_path))?;

    println!("Successfully updated .cargo/config.toml.");
    Ok(())
}
