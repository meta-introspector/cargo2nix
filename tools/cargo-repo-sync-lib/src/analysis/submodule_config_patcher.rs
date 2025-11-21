use anyhow::{Result, Context};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use regex::Regex;

pub trait SubmoduleConfigPatcher {
    fn generate_submodule_patches(
        &self,
        members_file: &Path,
        project_root: &Path,
    ) -> Result<HashMap<String, Vec<String>>>;
}

pub struct RealSubmoduleConfigPatcher;

impl SubmoduleConfigPatcher for RealSubmoduleConfigPatcher {
    fn generate_submodule_patches(
        &self,
        members_file: &Path,
        project_root: &Path,
    ) -> Result<HashMap<String, Vec<String>>> {
        let mut generated_patches = HashMap::new();

        let content = std::fs::read_to_string(members_file)
            .with_context(|| format!("Failed to read members file: {}", members_file.display()))?;

        let member_line_re = Regex::new(r"(submodules/[^/]+/Cargo\.toml):members = \[(.*)\]")?;
        let member_name_re = Regex::new(r#"([^\"]+)""#)?;

        for line in content.lines() {
            if let Some(captures) = member_line_re.captures(line) {
                let cargo_toml_rel_path = captures[1].to_string();
                let members_str = captures[2].to_string();

                let submodule_base_path_rel = PathBuf::from(cargo_toml_rel_path)
                    .parent()
                    .context("Invalid Cargo.toml path in members file")?
                    .to_path_buf();
                let submodule_name = submodule_base_path_rel
                    .file_name()
                    .context("Invalid submodule base path")?
                    .to_string_lossy()
                    .to_string();

                let patch_section_header = format!("[patch.\"https://github.com/meta-introspector/{}\"]", submodule_name);
                let mut entries: Vec<String> = Vec::new();

                for member_capture in member_name_re.captures_iter(&members_str) {
                    let member_name = member_capture[1].to_string();
                    if member_name != "." { // Filter out '.' members
                        let member_abs_path = project_root
                            .join(&submodule_base_path_rel)
                            .join(&member_name);
                        entries.push(format!("{} = {{ path = \"{}\" }}", member_name, member_abs_path.display()));
                    }
                }
                if !entries.is_empty() {
                    generated_patches.insert(patch_section_header, entries);
                }
            }
        }
        Ok(generated_patches)
    }
}
