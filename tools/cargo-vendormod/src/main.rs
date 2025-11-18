use anyhow::{Context, Result};
use clap::Parser;
use serde::Serialize;
use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::{Arc, Mutex}, // Added
};
use toml_edit::Document;
use walkdir::WalkDir;
use lazy_static::lazy_static;
use regex::Regex;

use crate::RollupLock; // Added
use crate::repo_sync_lib::git_snapshot::create_snapshot; // Added

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Perform a dry run without making actual changes.
    /// In this mode, the JSON plan will be printed to stdout.
    #[arg(long)]
    dry_run: bool,

    /// The root directory to start scanning for Cargo.toml files.
    #[arg(long, default_value = ".")]
    root_dir: PathBuf,

    /// The GitHub organization to fork repositories to.
    #[arg(long, default_value = "meta-introspector")]
    target_org: String,

    /// The branch to checkout and use for dependencies.
    #[arg(long, default_value = "feature/CRQ-016-nixify")]
    target_branch: String,

    /// Optional: Write the JSON plan to this file instead of stdout.
    #[arg(long)]
    output_file: Option<PathBuf>,
}

#[derive(Debug, Serialize)]
struct RepoAction {
    repo_url: String,
    owner: String,
    repo_name: String,
    submodule_path: PathBuf,
    target_org: String,
    target_branch: String,
    // Add fields for specific actions if needed, e.g.,
    // clone: bool,
    // manage_remotes: bool,
    // fork: bool,
    // checkout_branch: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // --- Read executable paths from Cargo.toml metadata ---
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let cargo_toml_path = manifest_dir.join("Cargo.toml");
    let cargo_toml_content = fs::read_to_string(&cargo_toml_path)
        .with_context(|| format!("Failed to read Cargo.toml at {:?}", cargo_toml_path))?;
    let cargo_toml_doc = cargo_toml_content
        .parse::<Document<String>>()
        .with_context(|| format!("Failed to parse Cargo.toml at {:?}", cargo_toml_path))?;

    let git_executable_path = cargo_toml_doc
        .get("package")
        .and_then(|item| item.as_table())
        .and_then(|table| table.get("metadata"))
        .and_then(|item| item.as_table())
        .and_then(|table| table.get("repo-manager"))
        .and_then(|item| item.as_table())
        .and_then(|table| table.get("git_path"))
        .and_then(|item| item.as_str())
        .map(PathBuf::from)
        .ok_or_else(|| anyhow::anyhow!("'git_path' not found in Cargo.toml metadata"))?;

    let gh_executable_path = cargo_toml_doc
        .get("package")
        .and_then(|item| item.as_table())
        .and_then(|table| table.get("metadata"))
        .and_then(|item| item.as_table())
        .and_then(|table| table.get("repo-manager"))
        .and_then(|item| item.as_table())
        .and_then(|table| table.get("gh_path"))
        .and_then(|item| item.as_str())
        .map(PathBuf::from)
        .ok_or_else(|| anyhow::anyhow!("'gh_path' not found in Cargo.toml metadata"))?;
    // --- End Read executable paths from Cargo.toml metadata ---

    // --- Diagnostic: Check for git and gh executables ---
    println!("Checking for 'git' executable...");
    let git_version_output = Command::new(&git_executable_path).arg("--version").output();
    match git_version_output {
        Ok(output) => {
            if output.status.success() {
                println!("'git' found: {}", String::from_utf8_lossy(&output.stdout).trim());
            } else {
                eprintln!("'git' command failed: {}", String::from_utf8_lossy(&output.stderr).trim());
                anyhow::bail!("'git' executable not working correctly. Please ensure Git is installed and in your PATH.");
            }
        },
        Err(e) => anyhow::bail!("Failed to execute '{:?} --version': {}. Please ensure Git is installed and in your PATH.", git_executable_path, e),
    }

    println!("Checking for 'gh' executable...");
    let gh_version_output = Command::new(&gh_executable_path).arg("--version").output();
    match gh_version_output {
        Ok(output) => {
            if output.status.success() {
                println!("'gh' found: {}", String::from_utf8_lossy(&output.stdout).trim());
            } else {
                eprintln!("'gh' command failed: {}", String::from_utf8_lossy(&output.stderr).trim());
                anyhow::bail!("'gh' executable not working correctly. Please ensure GitHub CLI is installed and in your PATH.");
            }
        },
        Err(e) => eprintln!("Warning: Failed to execute '{:?} --version': {}. GitHub CLI might not be installed or not working correctly. Forking operations might fail.", gh_executable_path, e),
    }
    // --- End Diagnostic ---

    let root_dir = args.root_dir.canonicalize().context("Failed to canonicalize root_dir")?;
    let submodules_dir = root_dir.join("submodules");

    if args.dry_run {
        println!("--- DRY RUN MODE ACTIVE ---");
    }

    // Create the submodules directory if it doesn't exist
    if args.dry_run {
        println!("[DRY RUN] Would create directory: {:?}", submodules_dir);
    } else {
        fs::create_dir_all(&submodules_dir).context("Failed to create submodules directory")?;
    }

    let mut unique_repo_urls: HashSet<String> = HashSet::new();

    // Run cargo metadata to get the dependency graph
    println!("Running `cargo metadata` to resolve dependencies...");
    let cargo_metadata_output = Command::new("cargo")
        .arg("metadata")
        .arg("--format-version=1")
        .output()
        .context("Failed to execute `cargo metadata`")?;

    if !cargo_metadata_output.status.success() {
        anyhow::bail!(
            "`cargo metadata` failed: {}",
            String::from_utf8_lossy(&cargo_metadata_output.stderr)
        );
    }

    let metadata: serde_json::Value = serde_json::from_slice(&cargo_metadata_output.stdout)
        .context("Failed to parse `cargo metadata` output")?;

    // Iterate over packages and their dependencies
    if let Some(packages) = metadata["packages"].as_array() {
        for pkg in packages {
            if let Some(dependencies) = pkg["dependencies"].as_array() {
                for dep in dependencies {
                    if let Some(source) = dep["source"].as_str() {
                        // Filter for git dependencies
                        if source.starts_with("git+") && source.contains("github.com") {
                            // Clean up the source URL to get the base repository URL
                            let cleaned_source = source
                                .split('#') // Remove branch/tag info
                                .next()
                                .unwrap_or(source)
                                .trim_start_matches("git+")
                                .to_string();
                            unique_repo_urls.insert(cleaned_source);
                        }
                    }
                }
            }
        }
    }

    println!("Found {} unique git repository dependencies.", unique_repo_urls.len());

    let mut actions_plan: Vec<RepoAction> = Vec::new();

    for repo_url_str in unique_repo_urls {
        // Use regex to extract owner and repo name more robustly
        lazy_static! {
            static ref GITHUB_URL_RE: Regex = Regex::new(r"github\.com/([^/]+)/([^/.]+)(?:/tree/[^/]+/.+)?(\.git)?").unwrap();
        }

        let (owner, repo_name) = if let Some(captures) = GITHUB_URL_RE.captures(&repo_url_str) {
            let owner = captures.get(1).map_or("", |m| m.as_str());
            let repo_name = captures.get(2).map_or("", |m| m.as_str());
            (owner.to_string(), repo_name.to_string())
        } else {
            eprintln!("Could not extract owner or repository name from {}. Skipping.", repo_url_str);
            continue;
        };

        if owner.is_empty() || repo_name.is_empty() {
            eprintln!("Could not extract owner or repository name from {}. Skipping.", repo_url_str);
            continue;
        }

        // Construct the canonical GitHub repository URL
        let canonical_repo_url = format!("https://github.com/{}/{}.git", owner, repo_name);
        let submodule_path = submodules_dir.join(&repo_name);

        actions_plan.push(RepoAction {
            repo_url: canonical_repo_url,
            owner,
            repo_name,
            submodule_path,
            target_org: args.target_org.clone(),
            target_branch: args.target_branch.clone(),
        });
    }

    // Serialize the actions plan to JSON
    let json_plan = serde_json::to_string_pretty(&actions_plan)
        .context("Failed to serialize actions plan to JSON")?;

    let rollup_lock_data = Arc::new(Mutex::new(RollupLock::load(&root_dir)?));

    if !args.dry_run {
        execute_actions_plan(actions_plan, &args, &git_executable_path, &gh_executable_path, rollup_lock_data, &root_dir)?;
    } else {
        // If dry_run, print the plan to stdout or file
        if let Some(output_file_path) = args.output_file {
            fs::write(&output_file_path, json_plan)
                .with_context(|| format!("Failed to write JSON plan to {:?}", output_file_path))?;
            eprintln!("JSON plan written to {:?}", output_file_path);
        } else {
            println!("{}", json_plan);
        }
    }

    Ok(())
}

fn execute_actions_plan(
    actions_plan: Vec<RepoAction>,
    args: &Args,
    git_executable_path: &Path,
    gh_executable_path: &Path,
    rollup_lock: Arc<Mutex<RollupLock>>,
    root_dir: &Path,
) -> Result<()> {
    println!("Executing actions plan...");

    for action in actions_plan {
        println!("Processing repository: {}", action.repo_name);

        // 1. Fork the repository if it doesn't exist in target_org
        let forked_repo_url = format!("https://github.com/{}/{}.git", args.target_org, action.repo_name);
        let gh_repo_check_output = Command::new(gh_executable_path)
            .arg("repo")
            .arg("view")
            .arg(&forked_repo_url)
            .arg("--json")
            .arg("name")
            .output()
            .context(format!("Failed to check if {} exists in {}", action.repo_name, args.target_org))?;

        if !gh_repo_check_output.status.success() || String::from_utf8_lossy(&gh_repo_check_output.stdout).trim().is_empty() {
            println!("Forking {} to {}...", action.repo_name, args.target_org);
            let fork_output = Command::new(gh_executable_path)
                .arg("repo")
                .arg("fork")
                .arg(&action.repo_url)
                .arg("--org")
                .arg(&args.target_org)
                .arg("--remote") // Add remote to the forked repo
                .arg("--clone=false") // Don't clone immediately, we'll add as submodule
                .output()
                .context(format!("Failed to fork {} to {}", action.repo_name, args.target_org))?;

            if !fork_output.status.success() {
                eprintln!(
                    "Failed to fork {}: {}",
                    action.repo_name,
                    String::from_utf8_lossy(&fork_output.stderr)
                );
                anyhow::bail!("Forking failed for {}", action.repo_name);
            }
            println!("Successfully forked {}.", action.repo_name);
        } else {
            println!("Repository {} already exists in {}. Skipping fork.", action.repo_name, args.target_org);
        }

        // 2. Add as git submodule
        if !action.submodule_path.exists() {
            println!("Adding {} as submodule...", action.repo_name);
            let add_submodule_output = Command::new(git_executable_path)
                .arg("submodule")
                .arg("add")
                .arg(&forked_repo_url)
                .arg(&action.submodule_path)
                .output()
                .context(format!("Failed to add {} as submodule", action.repo_name))?;

            if !add_submodule_output.status.success() {
                eprintln!(
                    "Failed to add submodule {}: {}",
                    action.repo_name,
                    String::from_utf8_lossy(&add_submodule_output.stderr)
                );
                anyhow::bail!("Adding submodule failed for {}", action.repo_name);
            }
            println!("Successfully added {} as submodule.", action.repo_name);
            create_snapshot(root_dir, rollup_lock.clone())?;
        } else {
            println!("Submodule {} already exists at {:?}. Skipping add.", action.repo_name, action.submodule_path);
        }

        // 3. Checkout target branch in submodule
        println!("Checking out branch '{}' in submodule {}...", action.target_branch, action.repo_name);
        let checkout_output = Command::new(git_executable_path)
            .arg("-C")
            .arg(&action.submodule_path)
            .arg("checkout")
            .arg(&action.target_branch)
            .output()
            .context(format!("Failed to checkout branch {} in submodule {}", action.target_branch, action.repo_name))?;

        if !checkout_output.status.success() {
            eprintln!(
                "Failed to checkout branch '{}' in submodule {}: {}",
                action.target_branch,
                action.repo_name,
                String::from_utf8_lossy(&checkout_output.stderr)
            );
            anyhow::bail!("Branch checkout failed for {}", action.repo_name);
        }
        println!("Successfully checked out branch '{}' in submodule {}.", action.target_branch, action.repo_name);
        create_snapshot(root_dir, rollup_lock.clone())?;
    }

    update_cargo_config(&actions_plan, &args.root_dir)?;

    println!("Successfully executed all actions.");
    Ok(())
}

fn update_cargo_config(actions_plan: &[RepoAction], root_dir: &Path) -> Result<()> {
    println!("Updating .cargo/config.toml...");
    let cargo_config_dir = root_dir.join(".cargo");
    fs::create_dir_all(&cargo_config_dir).context("Failed to create .cargo directory")?;
    let cargo_config_path = cargo_config_dir.join("config.toml");

    let mut config_doc = if cargo_config_path.exists() {
        let content = fs::read_to_string(&cargo_config_path)
            .with_context(|| format!("Failed to read {:?}", cargo_config_path))?;
        content
            .parse::<Document<String>>()
            .context("Failed to parse .cargo/config.toml")?
    } else {
        Document::new()
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
        patch_crates_io.insert(
            &action.repo_name, // Use repo_name as crate name for now, might need refinement
            toml_edit::value(toml_edit::table().insert("path", toml_edit::value(path_str))),
        );
    }

    fs::write(&cargo_config_path, config_doc.to_string())
        .with_context(|| format!("Failed to write to {:?}", cargo_config_path))?;

    println!("Successfully updated .cargo/config.toml.");
    Ok(())
}
