use anyhow::{Context, Result};
use clap::Parser;
use serde::{Serialize, Deserialize};
use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
    process::Command,
};
//use toml_edit::DocumentMut;
use walkdir::WalkDir;
use lazy_static::lazy_static;
use regex::Regex;
mod execute_actions_plan;
use execute_actions_plan::*;

mod update_cargo_config;
use update_cargo_config::*;

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

    /// Optional: Read the actions plan from a JSON file instead of discovering repositories.
    #[arg(long)]
    json_input_file: Option<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize)]
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
        .parse::<toml_edit::DocumentMut>()
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

    let actions_plan: Vec<RepoAction>;

    if let Some(json_path) = &args.json_input_file {
        println!("Reading actions plan from JSON file: {:?}", json_path);
        let json_content = fs::read_to_string(json_path)
            .with_context(|| format!("Failed to read JSON input file at {:?}", json_path))?;
        actions_plan = serde_json::from_str(&json_content)
            .with_context(|| format!("Failed to parse JSON from {:?}", json_path))?;
    } else {
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

        println!("Found {} unique git repository dependencies from `cargo metadata`.", unique_repo_urls.len());

        // Find all Cargo.toml files and extract repository URLs from [package] repository field
        println!("Scanning for Cargo.toml files to extract `[package] repository` fields...");
        for entry in WalkDir::new(&root_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file() && e.file_name() == "Cargo.toml")
            .filter(|e| {
                let path = e.path();
                !(path.components().any(|c| {
                    let component_str = c.as_os_str().to_string_lossy();
                    component_str == "target" || component_str == "tests" || component_str == "examples" || component_str == "submodules"
                }))
            })
        {
            let cargo_toml_path = entry.path();
            let content = fs::read_to_string(&cargo_toml_path)
                .with_context(|| format!("Failed to read Cargo.toml at {:?}", cargo_toml_path))?;
                    let doc = content
                        .parse::<toml_edit::DocumentMut>()                .with_context(|| format!("Failed to parse Cargo.toml at {:?}", cargo_toml_path))?;

            if let Some(repository_url) = doc
                .get("package")
                .and_then(|item| item.as_table())
                .and_then(|table| table.get("repository"))
                .and_then(|item| item.as_str())
            {
                // Only add if it's a GitHub URL
                if repository_url.contains("github.com") {
                    unique_repo_urls.insert(repository_url.to_string());
                }
            }
        }

        println!("Found a total of {} unique repository URLs after scanning `[package] repository` fields.", unique_repo_urls.len());

        let mut discovered_actions_plan: Vec<RepoAction> = Vec::new();

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

            discovered_actions_plan.push(RepoAction {
                repo_url: canonical_repo_url,
                owner,
                repo_name,
                submodule_path,
                target_org: args.target_org.clone(),
                target_branch: args.target_branch.clone(),
            });
        }
        actions_plan = discovered_actions_plan;

        // Serialize the actions plan to JSON (only if not reading from JSON input)
        let json_plan = serde_json::to_string_pretty(&actions_plan)
            .context("Failed to serialize actions plan to JSON")?;

        if args.dry_run {
            // If dry_run, print the plan to stdout or file
            if let Some(output_file_path) = args.output_file {
                fs::write(&output_file_path, json_plan)
                    .with_context(|| format!("Failed to write JSON plan to {:?}", output_file_path))?;
                eprintln!("JSON plan written to {:?}", output_file_path);
            } else {
                println!("{}", json_plan);
            }
            return Ok(()); // Exit after printing dry run plan
        }
    }

    // Execute actions plan (either discovered or read from JSON)
    execute_actions_plan(&actions_plan, &args, &git_executable_path, &gh_executable_path)?;

    Ok(())
}



