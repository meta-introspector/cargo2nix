use std::{env, fs, io::Write, path::{Path, PathBuf}, collections::HashMap};
use std::fs::File;
use toml_edit::DocumentMut;

// Import modules from the library crate
use crate::cli; // Keep cli local
use cargo_repo_sync_lib::{
    commit_and_push_submodule, generate_submodule_patches, add_submodule, remove_submodule, fork_and_patch_submodules, get_submodule_status,
    CargoUpdateCommand, CargoVendorCommand, Cargo2NixCommand, CargoCommand, Plan, Task, get_cargo_command, rename_cargo_config, restore_cargo_config, RemoveRustVersionCommand, generate_nix_expression, analyze_repository, update_cargo_toml_files, process_tt_txt_files, collect_repository_state,
    WorkspaceGenerator, DefaultWorkspaceGenerator,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli::cli().get_matches();
    let dry_run = *matches.get_one::<bool>("dry-run").unwrap_or(&false);

    match matches.subcommand() {
        Some(("submodule", sub_matches)) => {
            match sub_matches.subcommand() {
                Some(("add", add_matches)) => {
                    let url = add_matches.get_one::<String>("url").expect("URL is required");
                    let path = add_matches.get_one::<String>("path").expect("Path is required");
                    let branch = add_matches.get_one::<String>("branch");
                    let name = add_matches.get_one::<String>("name");
                    println!("Adding submodule: URL={}, Path={}", url, path);
                    add_submodule(
                        url,
                        path,
                        branch,
                        name,
                        dry_run,
                    )?;
                },
                Some(("remove", remove_matches)) => {
                    let path = remove_matches.get_one::<String>("path").expect("Path is required");
                    println!("Removing submodule: Path={}", path);
                    remove_submodule(
                        path,
                        dry_run,
                    )?;
                },
                Some(("commit-and-push", commit_matches)) => {
                    let message = commit_matches.get_one::<String>("message").expect("Commit message is required");
                    println!("Committing and pushing submodules with message: {}", message);

                    let current_dir = env::current_dir().expect("Failed to get current directory");

                    // Create logs directory if it doesn't exist
                    let logs_dir = current_dir.join("logs");
                    fs::create_dir_all(&logs_dir)
                        .map_err(|e| format!("Failed to create logs directory: {}", e))?;

                    let log_file_path = logs_dir.join("submodule_commit_push.log");
                    let mut log_file = File::create(&log_file_path)
                        .map_err(|e| format!("Failed to create log file at {:?}: {}", log_file_path, e))?;

                    writeln!(log_file, "--- Submodule Commit and Push Log (Dry Run: {})", dry_run)?;
                    println!("Logging submodule commit and push output to {:?}", log_file_path);

                    let repo = git2::Repository::open(&current_dir)
                        .map_err(|e| format!("Failed to open parent repository at {:?}: {}", current_dir, e))?;

                    for submodule in repo.submodules()
                        .map_err(|e| format!("Failed to iterate submodules: {}", e))?
                    {
                        let submodule_path = current_dir.join(submodule.path());
                        match commit_and_push_submodule(&submodule_path, message, &mut log_file, dry_run) {
                            Ok(_) => writeln!(log_file, "Successfully processed submodule: {:?}", submodule_path)?,
                            Err(e) => {
                                writeln!(log_file, "Error processing submodule {:?}: {}", submodule_path, e)?;
                                eprintln!("Error processing submodule {:?}: {}", submodule_path, e);
                                return Err(e.into());
                            }
                        }
                    }
                    writeln!(log_file, "Submodule commit and push process completed.")?;
                    println!("Submodule commit and push process completed.");
                },
                Some(("patch", patch_matches)) => {
                    let output_file = patch_matches.get_one::<String>("output").expect("Output file is required");
                    let dry_run_patch = *patch_matches.get_one::<bool>("dry-run").unwrap_or(&false);
                    let overwrite = *patch_matches.get_one::<bool>("overwrite").unwrap_or(&false);
                    let git_url_template = patch_matches.get_one::<String>("git-url-template");
                    let branch_template = patch_matches.get_one::<String>("branch-template");
                    let recursive = *patch_matches.get_one::<bool>("recursive").unwrap_or(&false);

                    let current_dir = env::current_dir().expect("Failed to get current directory");

                    match generate_submodule_patches(
                        &current_dir,
                        output_file,
                        dry_run_patch, // Use dry_run_patch for this subcommand
                        overwrite,
                        git_url_template,
                        branch_template,
                        recursive,
                    ) {
                        Ok(message) => println!("{}", message),
                        Err(e) => eprintln!("Error generating submodule patches: {}", e),
                    }
                },
                Some(("fork-and-patch", fork_matches)) => {
                    let target_org = fork_matches.get_one::<String>("target-org").expect("Target organization is required");
                    let target_branch = fork_matches.get_one::<String>("target-branch").expect("Target branch is required");
                    let dry_run_fork = *fork_matches.get_one::<bool>("dry-run").unwrap_or(&false);

                    println!("Forking and patching submodules: Target Org={}, Target Branch={}, Dry Run={}", target_org, target_branch, dry_run_fork);

                    let current_dir = env::current_dir().expect("Failed to get current directory");

                    // Create logs directory if it doesn't exist
                    let logs_dir = current_dir.join("logs");
                    fs::create_dir_all(&logs_dir)
                        .map_err(|e| format!("Failed to create logs directory: {}", e))?;

                    let log_file_path = logs_dir.join("fork_and_patch.log");
                    let mut log_file = File::create(&log_file_path)
                        .map_err(|e| format!("Failed to create log file at {:?}: {}", log_file_path, e))?;

                    writeln!(log_file, "--- Fork and Patch Log (Dry Run: {})", dry_run_fork)?;
                    println!("Logging fork and patch output to {:?}", log_file_path);

                    match fork_and_patch_submodules(
                        &current_dir,
                        target_org,
                        target_branch,
                        &mut log_file,
                        dry_run_fork,
                    ) {
                        Ok(_) => {
                            writeln!(log_file, "Fork and patch process completed successfully.")?;
                            println!("Fork and patch process completed successfully.");
                        },
                        Err(e) => {
                            writeln!(log_file, "Error during fork and patch process: {}", e)?;
                            eprintln!("Error during fork and patch process: {}", e);
                            return Err(e.into());
                        }
                    }
                },
                Some(("status", _)) => {
                    println!("Submodule status subcommand invoked.");
                    let current_dir = env::current_dir().expect("Failed to get current directory");
                    get_submodule_status(&current_dir)?;
                },
                _ => unreachable!(),
            }
        },
        Some(("generate-nix", _)) => {
            println!("Generate Nix subcommand invoked.");
            let current_dir = env::current_dir().expect("Failed to get current directory");
            let mut log_file = File::create(current_dir.join("logs/generate_nix.log"))?;
            generate_nix_expression(&current_dir, &mut log_file, dry_run)?;
        },
        Some(("analyze", _)) => {
            println!("Analyze subcommand invoked.");
            let current_dir = env::current_dir().expect("Failed to get current directory");
            let mut log_file = File::create(current_dir.join("logs/analyze.log"))?;
            analyze_repository(&current_dir, &mut log_file, dry_run)?;
        },
        Some(("update-cargo-toml", _)) => {
            println!("Update Cargo.toml subcommand invoked.");
            let current_dir = env::current_dir().expect("Failed to get current directory");
            let mut log_file = File::create(current_dir.join("logs/update_cargo_toml.log"))?;
            update_cargo_toml_files(&current_dir, &mut log_file, dry_run)?;
        },
        Some(("process-tt-txt", _)) => {
            println!("Process tt.txt subcommand invoked.");
            let current_dir = env::current_dir().expect("Failed to get current directory");
            let mut log_file = File::create(current_dir.join("logs/process_tt_txt.log"))?;
            process_tt_txt_files(&current_dir, &mut log_file, dry_run)?;
        },
        Some(("collect-repo-state", _)) => {
            println!("Collect repo state subcommand invoked.");
            let current_dir = env::current_dir().expect("Failed to get current directory");
            let mut log_file = File::create(current_dir.join("logs/collect_repo_state.log"))?;
            collect_repository_state(&current_dir, &mut log_file, dry_run)?;
        },
        Some(("generate-workspace-deps", _)) => {
            println!("Generate workspace dependencies subcommand invoked.");
            let current_dir = env::current_dir().expect("Failed to get current directory");
            let generator = DefaultWorkspaceGenerator;
            match generator.generate_workspace_dependencies(&current_dir, dry_run) {
                Ok(_) => println!("Successfully generated workspace dependencies."),
                Err(e) => eprintln!("Error generating workspace dependencies: {}", e),
            }
        },
        _ => {
            eprintln!("No subcommand provided or unknown subcommand. Use --help for more information.");
            std::process::exit(1);
        }
    }
    Ok(())
}

