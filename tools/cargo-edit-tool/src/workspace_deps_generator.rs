use anyhow::{Context, Result};
use cargo_metadata::MetadataCommand; // Import cargo_metadata
use git2::Repository; // Import git2
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir; // Import walkdir

pub trait WorkspaceDepsGenerator {
    fn generate_submodule_path_map(
        &self,
        submodules_dir: &Path,
    ) -> Result<HashMap<String, PathBuf>>;
    fn process_tt_txt_accurate(&self, tt_txt_path: &Path, submodules_dir: &Path) -> Result<String>;
}

pub struct RealWorkspaceDepsGenerator;

impl WorkspaceDepsGenerator for RealWorkspaceDepsGenerator {
    fn generate_submodule_path_map(
        &self,
        submodules_dir: &Path,
    ) -> Result<HashMap<String, PathBuf>> {
        let mut submodule_path_map = HashMap::new();

        // The project root is the parent of the 'submodules' directory
        let project_root = submodules_dir
            .parent()
            .context("Submodules directory has no parent")?;
        let repo = Repository::open(project_root).context(format!(
            "Failed to open git repository at {:?}",
            project_root
        ))?;

        for submodule in repo.submodules().context("Failed to read submodules")? {
            let submodule_path_rel = submodule.path();
            let submodule_abs_path = project_root.join(submodule_path_rel);

            // Ensure the submodule directory exists
            if !submodule_abs_path.is_dir() {
                eprintln!(
                    "Warning: Submodule directory {:?} does not exist, skipping.",
                    submodule_abs_path
                );
                continue;
            }

            // Recursively find all Cargo.toml files within the submodule
            for entry in WalkDir::new(&submodule_abs_path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file() && e.file_name() == "Cargo.toml")
            {
                let cargo_toml_path = entry.path();

                // Get metadata for each Cargo.toml
                let metadata = MetadataCommand::new()
                    .manifest_path(&cargo_toml_path)
                    .current_dir(project_root)
                    .no_deps() // We only need info about the package itself, not its dependencies
                    .exec()
                    .with_context(|| {
                        format!(
                            "Failed to get cargo metadata for manifest: {:?}",
                            cargo_toml_path
                        )
                    })?;

                for package in metadata.packages {
                    // Calculate the relative path from the project root to the package's manifest directory
                    let package_manifest_dir = PathBuf::from(package.manifest_path)
                        .parent()
                        .unwrap()
                        .to_path_buf();
                    let relative_package_path = package_manifest_dir
                        .strip_prefix(project_root)
                        .context(format!(
                            "Failed to get relative path for package {:?} from project root {:?}",
                            package.name, project_root
                        ))?;

                    submodule_path_map.insert(
                        package.name.to_string(),
                        relative_package_path.to_path_buf(),
                    );
                }
            }
        }
        Ok(submodule_path_map)
    }

    fn process_tt_txt_accurate(&self, tt_txt_path: &Path, submodules_dir: &Path) -> Result<String> {
        let submodule_map = self.generate_submodule_path_map(submodules_dir)?;
        let mut output_lines = Vec::new();

        let content = fs::read_to_string(tt_txt_path)
            .with_context(|| format!("Failed to read tt.txt file: {}", tt_txt_path.display()))?;

        let dep_re = Regex::new(r"(\w[\w-]*)\s*=\s*(.*)")?;
        let path_match_re = Regex::new(r#"path\s*=\s*"(.*?)""#)?;

        for line in content.lines() {
            if line.trim().is_empty() {
                output_lines.push("".to_string());
                continue;
            }

            if let Some(captures) = dep_re.captures(line) {
                let crate_name = captures[1].to_string();
                let dep_definition = captures[2].to_string();

                if path_match_re.is_match(&dep_definition) {
                    // If it's already a path dependency
                    if let Some(submodule_path) = submodule_map.get(&crate_name) {
                        output_lines.push(format!(
                            "{} = {{ path = \"{}\" }}",
                            crate_name,
                            submodule_path.display()
                        ));
                    } else {
                        // If it's a path dependency but not in our submodule map, keep original
                        output_lines.push(line.to_string());
                    }
                } else {
                    // It's a version dependency or other type, try to find a submodule
                    if let Some(submodule_path) = submodule_map.get(&crate_name) {
                        output_lines.push(format!(
                            "{} = {{ path = \"{}\" }}",
                            crate_name,
                            submodule_path.display()
                        ));
                    } else {
                        // If no submodule found, keep the original line (version dependency)
                        output_lines.push(line.to_string());
                    }
                }
            } else {
                output_lines.push(line.to_string()); // Keep lines that don't match the pattern
            }
        }
        Ok(output_lines.join("\n"))
    }
}
