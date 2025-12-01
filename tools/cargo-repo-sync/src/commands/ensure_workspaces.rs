use anyhow::{Context, Result};
use clap::Parser;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader}; // Add this for CLI argument parsing

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    /// Path to the project root.
    #[arg(long)]
    project_root: PathBuf,
    /// Path to the file containing a list of non-vendored modules.
    #[arg(long)]
    non_vendored_modules_file: PathBuf,
    /// Perform a dry-run without making actual changes.
    #[arg(long)]
    dry_run: bool,
}

/// Ensures that non-vendored modules have a `[workspace]` section in their `Cargo.toml`.
///
/// This function replicates the functionality of `ensure_non_vendored_workspaces.sh`.
pub async fn ensure_non_vendored_workspaces(
    project_root: &Path,
    non_vendored_modules_file: &Path,
    dry_run: bool,
) -> Result<()> {
    if !non_vendored_modules_file.exists() {
        return Err(anyhow::anyhow!(
            "Error: Non-vendored modules list not found at {}",
            non_vendored_modules_file.display()
        ));
    }

    println!("Ensuring non-vendored modules have a [workspace] section in their Cargo.toml...");

    let file = fs::File::open(non_vendored_modules_file).await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        let module_name = line
            .split_whitespace()
            .next()
            .context("Invalid line format in non_vendored_modules.txt")?;

        let mut cargo_toml_path = None;

        // Prioritize submodules/, then vendor/
        let submodule_path = project_root.join("submodules").join(module_name);
        if submodule_path.is_dir() {
            cargo_toml_path = Some(submodule_path.join("Cargo.toml"));
        } else {
            let vendor_path = project_root.join("vendor").join(module_name);
            if vendor_path.is_dir() {
                cargo_toml_path = Some(vendor_path.join("Cargo.toml"));
            }
        }

        if let Some(path) = cargo_toml_path {
            if path.exists() {
                let cargo_toml_content = fs::read_to_string(&path).await?;
                if !cargo_toml_content.contains("\n[workspace]")
                    && !cargo_toml_content.starts_with("[workspace]")
                {
                    println!(
                        "ACTION: Would add [workspace] to {}/Cargo.toml ({})",
                        module_name,
                        path.display()
                    );
                    if !dry_run {
                        let mut file = fs::OpenOptions::new().append(true).open(&path).await?;
                        file.write_all(b"\n[workspace]\n").await?;
                        println!("INFO: Added [workspace] to {}", path.display());
                    }
                } else {
                    println!(
                        "INFO: [workspace] already exists in {}/Cargo.toml ({})",
                        module_name,
                        path.display()
                    );
                }
            } else {
                eprintln!(
                    "WARNING: Cargo.toml not found for non-vendored module {} at expected paths.",
                    module_name
                );
            }
        } else {
            eprintln!(
                "WARNING: Module directory not found for non-vendored module {} at expected paths.",
                module_name
            );
        }
    }

    println!("Finished ensuring [workspace] sections for non-vendored modules.");
    Ok(())
}

// Add a test module to verify functionality
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use tokio::fs;

    async fn setup_test_project() -> Result<(PathBuf, PathBuf)> {
        let tmp_dir = tempdir()?;
        let project_root = tmp_dir.path().to_path_buf();

        fs::create_dir_all(&project_root.join("submodules/test_module_1")).await?;
        fs::write(
            project_root.join("submodules/test_module_1/Cargo.toml"),
            "[package]\nname=\"test_module_1\"\nversion=\"0.1.0\"\nedition=\"2021\"\n",
        )
        .await?;

        fs::create_dir_all(&project_root.join("vendor/test_module_2")).await?;
        fs::write(
            project_root.join("vendor/test_module_2/Cargo.toml"),
            "[package]\nname=\"test_module_2\"\nversion=\"0.1.0\"\nedition=\"2021\"\n",
        )
        .await?;

        fs::create_dir_all(&project_root.join("submodules/test_module_3")).await?;
        fs::write(
            project_root.join("submodules/test_module_3/Cargo.toml"),
            "[package]\nname=\"test_module_3\"\nversion=\"0.1.0\"\nedition=\"2021\"\n[workspace]\n",
        )
        .await?;

        let non_vendored_modules_file = project_root.join(".cargo/non_vendored_modules.txt");
        fs::create_dir_all(non_vendored_modules_file.parent().unwrap()).await?;
        fs::write(
            &non_vendored_modules_file,
            "test_module_1 10\ntest_module_2 20\ntest_module_3 30\ntest_module_4 40\n",
        )
        .await?;

        Ok((project_root, non_vendored_modules_file))
    }

    #[tokio::test]
    async fn test_ensure_workspaces_dry_run() -> Result<()> {
        let (project_root, non_vendored_modules_file) = setup_test_project().await?;

        ensure_non_vendored_workspaces(&project_root, &non_vendored_modules_file, true).await?;

        let content1 =
            fs::read_to_string(project_root.join("submodules/test_module_1/Cargo.toml")).await?;
        assert!(!content1.contains("[workspace]")); // Should not have been added

        let content2 =
            fs::read_to_string(project_root.join("vendor/test_module_2/Cargo.toml")).await?;
        assert!(!content2.contains("[workspace]")); // Should not have been added

        let content3 =
            fs::read_to_string(project_root.join("submodules/test_module_3/Cargo.toml")).await?;
        assert!(content3.contains("[workspace]")); // Should already exist

        Ok(())
    }

    #[tokio::test]
    async fn test_ensure_workspaces_actual_run() -> Result<()> {
        let (project_root, non_vendored_modules_file) = setup_test_project().await?;

        ensure_non_vendored_workspaces(&project_root, &non_vendored_modules_file, false).await?;

        let content1 =
            fs::read_to_string(project_root.join("submodules/test_module_1/Cargo.toml")).await?;
        assert!(content1.contains("[workspace]")); // Should have been added

        let content2 =
            fs::read_to_string(project_root.join("vendor/test_module_2/Cargo.toml")).await?;
        assert!(content2.contains("[workspace]")); // Should have been added

        let content3 =
            fs::read_to_string(project_root.join("submodules/test_module_3/Cargo.toml")).await?;
        assert!(content3.contains("[workspace]")); // Should already exist

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = CliArgs::parse();

    if args.dry_run {
        println!("Running in DRY-RUN mode. No changes will be written to files.");
    }

    ensure_non_vendored_workspaces(
        &args.project_root,
        &args.non_vendored_modules_file,
        args.dry_run,
    )
    .await?;

    Ok(())
}
