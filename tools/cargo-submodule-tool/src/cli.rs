use clap::{Arg, ArgAction, Command as ClapCommand};

pub fn cli() -> ClapCommand {
    ClapCommand::new("cargo-repo-sync")
        .about("A cargo subcommand for managing Git repositories and Nix integration.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(
            Arg::new("dry-run")
                .long("dry-run")
                .action(ArgAction::SetTrue)
                .help("Perform a dry run without making actual changes."),
        )
        .arg(
            Arg::new("json-log-file")
                .long("json-log-file")
                .value_name("FILE")
                .help("Capture all executed commands and their output to a JSON file."),
        )
        .arg(
            Arg::new("report")
                .long("report")
                .action(ArgAction::SetTrue)
                .help("Report detailed information about executed commands to stdout."),
        )
        .arg(
            Arg::new("pure-rust-git")
                .long("pure-rust-git")
                .action(ArgAction::SetTrue)
                .help("Use the pure Rust Git implementation instead of shelling out to the system 'git' command."),
        )
        .subcommand(
            ClapCommand::new("update")
                .about("Updates Cargo.lock and Cargo.nix files.")
                .arg(
                    Arg::new("force")
                        .long("force")
                        .short('f')
                        .action(ArgAction::SetTrue)
                        .help("Force update even if not needed."),
                ),
        )
        .subcommand(
            ClapCommand::new("vendor")
                .about("Vendors Cargo dependencies.")
                .arg(
                    Arg::new("force")
                        .long("force")
                        .short('f')
                        .action(ArgAction::SetTrue)
                        .help("Force vendoring even if not needed."),
                ),
        )
        .subcommand(
            ClapCommand::new("cargo2nix")
                .about("Generates Cargo.nix from Cargo.lock.")
                .arg(
                    Arg::new("force")
                        .long("force")
                        .short('f')
                        .action(ArgAction::SetTrue)
                        .help("Force cargo2nix even if not needed."),
                ),
        )
        .subcommand(
            ClapCommand::new("plan")
                .about("Manages and executes task plans.")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    ClapCommand::new("generate")
                        .about("Generates a plan.lock file from task TOMLs.")
                        .arg(
                            Arg::new("tasks-dir")
                                .long("tasks-dir")
                                .short('t')
                                .value_name("DIR")
                                .help("Directory containing task TOML files.")
                                .default_value("tasks"),
                        ),
                )
                .subcommand(
                    ClapCommand::new("run")
                        .about("Executes tasks from a plan.lock file.")
                        .arg(
                            Arg::new("plan-file")
                                .long("plan-file")
                                .short('p')
                                .value_name("FILE")
                                .help("Path to the plan.lock file.")
                                .default_value("plan.lock"),
                        )
                        .arg(
                            Arg::new("step")
                                .long("step")
                                .short('s')
                                .value_name("STEP_ID")
                                .help("Execute a specific step by ID."),
                        ),
                ),
        )
        .subcommand(
            ClapCommand::new("generate-nix")
                .about("Generates Cargo.nix files for all discovered Cargo.toml/Cargo.lock pairs.")
                .arg(
                    Arg::new("root-dir")
                        .long("root-dir")
                        .default_value(".")
                        .help("The root directory to start scanning for Cargo.toml files."),
                ),
        )
        .subcommand(
            ClapCommand::new("analyze")
                .about("Analyzes dependency graph, non-vendored modules, and generates config patches.")
                .arg(
                    Arg::new("depgraph-dot-file")
                        .long("depgraph-dot-file")
                        .default_value("depgraph.dot")
                        .help("Path to the depgraph.dot file."),
                )
                .arg(
                    Arg::new("tree-file")
                        .long("tree-file")
                        .default_value("tree.txt")
                        .help("Path to the tree.txt file."),
                )
                .arg(
                    Arg::new("cargo-lock-file")
                        .long("cargo-lock-file")
                        .default_value("Cargo.lock")
                        .help("Path to the Cargo.lock file."),
                )
                .arg(
                    Arg::new("cargo-config-file")
                        .long("cargo-config-file")
                        .default_value(".cargo/config.toml")
                        .help("Path to the .cargo/config.toml file."),
                )
                .arg(
                    Arg::new("members-file")
                        .long("members-file")
                        .default_value("submodules/members.txt")
                        .help("Path to the submodules/members.txt file."),
                )
                .arg(
                    Arg::new("submodules-dir")
                        .long("submodules-dir")
                        .default_value("submodules")
                        .help("Path to the submodules directory."),
                )
                .arg(
                    Arg::new("project-root")
                        .long("project-root")
                        .default_value(".")
                        .help("The root directory of the project."),
                ),
        )
        .subcommand(
            ClapCommand::new("update-cargo-toml")
                .about("Updates the [workspace.dependencies] section of a Cargo.toml file.")
                .arg(
                    Arg::new("cargo-toml-path")
                        .long("cargo-toml-path")
                        .default_value("Cargo.toml")
                        .help("Path to the Cargo.toml file to be updated."),
                )
                .arg(
                    Arg::new("project-root")
                        .long("project-root")
                        .default_value(".")
                        .help("The root directory of the project."),
                ),
        )
        .subcommand(
            ClapCommand::new("process-tt-txt")
                .about("Processes the tt.txt file to generate workspace dependencies.")
                .arg(
                    Arg::new("tt-txt-path")
                        .long("tt-txt-path")
                        .default_value("tt.txt")
                        .help("Path to the tt.txt file to be processed."),
                )
                .arg(
                    Arg::new("submodules-dir")
                        .long("submodules-dir")
                        .default_value("submodules")
                        .help("Path to the submodules directory."),
                ),
        )
        .subcommand(
            ClapCommand::new("collect-repo-state")
                .about("Collects and displays the current state of the repository (Git, Cargo, Nix).")
                .arg(
                    Arg::new("project-root")
                        .long("project-root")
                        .default_value(".")
                        .help("The root directory of the project."),
                ),
        )
        .subcommand(
            ClapCommand::new("submodule")
                .about("Manages Git submodules.")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    ClapCommand::new("add")
                        .about("Adds a Git submodule.")
                        .arg(
                            Arg::new("url")
                                .long("url")
                                .short('u')
                                .value_name("URL")
                                .help("URL of the submodule repository.")
                                .required(true),
                        )
                        .arg(
                            Arg::new("path")
                                .long("path")
                                .short('p')
                                .value_name("PATH")
                                .help("Path where the submodule will be added.")
                                .required(true),
                        )
                        .arg(
                            Arg::new("branch")
                                .long("branch")
                                .short('b')
                                .value_name("BRANCH")
                                .help("Branch to checkout in the submodule."),
                        )
                        .arg( // From CLI B AddSubmodulesArgs
                            Arg::new("target-org")
                                .long("target-org")
                                .value_name("ORG")
                                .help("The GitHub organization to fork repositories to.")
                                .default_value("meta-introspector"),
                        )
                        .arg( // From CLI B AddSubmodulesArgs
                            Arg::new("target-branch")
                                .long("target-branch")
                                .value_name("BRANCH")
                                .help("The branch to checkout and use for dependencies.")
                                .default_value("feature/CRQ-016-nixify"),
                        )
                        .arg( // From CLI B AddSubmodulesArgs
                            Arg::new("output-file")
                                .long("output-file")
                                .value_name("FILE")
                                .help("The output file for the generated submodule list.")
                                .default_value("submodules.txt"),
                        )
                        .arg( // From CLI B AddSubmodulesArgs
                            Arg::new("json-input-file")
                                .long("json-input-file")
                                .value_name("FILE")
                                .help("Path to a JSON input file for submodules."),
                        ),
                )
                .subcommand(
                    ClapCommand::new("status")
                        .about("Reports status of submodules.")
                        .arg(
                            Arg::new("root-dir")
                                .long("root-dir")
                                .default_value(".")
                                .help("The root directory of the project."),
                        ),
                )
                .subcommand(
                    ClapCommand::new("remove")
                        .about("Remove a Git submodule")
                        .arg(Arg::new("path")
                            .help("Path of the submodule to remove")
                            .required(true)),
                )
                .subcommand(
                    ClapCommand::new("commit-and-push")
                        .about("Commit and push changes in all submodules")
                        .arg(Arg::new("message")
                            .long("message")
                            .short('m')
                            .value_name("MESSAGE")
                            .help("Commit message for submodule changes")
                            .default_value("chore: Update submodule")),
                )
                .subcommand(
                    ClapCommand::new("patch")
                        .about("Generate [patch.crates-io] entries for submodules")
                        .arg(Arg::new("output")
                            .long("output")
                            .short('o')
                            .value_name("FILE")
                            .help("Specify the output .cargo/config.toml file")
                            .default_value("config.toml")) // Changed default to config.toml
                        .arg(Arg::new("dry-run")
                            .long("dry-run")
                            .action(ArgAction::SetTrue)
                            .help("Print generated patches to stdout without writing to a file"))
                        .arg(Arg::new("overwrite")
                            .long("overwrite")
                            .action(ArgAction::SetTrue)
                            .help("Overwrite existing [patch.crates-io] sections for submodules"))
                        .arg(Arg::new("git-url-template")
                            .long("git-url-template")
                            .value_name("TEMPLATE")
                            .help("Template string to construct the Git URL for submodules"))
                        .arg(Arg::new("branch-template")
                            .long("branch-template")
                            .value_name("TEMPLATE")
                            .help("Template string to construct the branch name for submodules"))
                        .arg(Arg::new("recursive")
                            .long("recursive")
                            .action(ArgAction::SetTrue)
                            .help("Recursively process nested submodules"))
                        .arg( // From CLI B GeneratePatchesArgs
                            Arg::new("root-dir")
                                .long("root-dir")
                                .default_value(".")
                                .help("The root directory to start scanning for Cargo.toml files."),
                        ),
                )
                .subcommand(
                    ClapCommand::new("fork-and-patch")
                        .about("Forks vendor crates, updates remotes, checks out a branch, and patches Cargo.toml dependencies.")
                        .arg(
                            Arg::new("target-org")
                                .long("target-org")
                                .short('o')
                                .value_name("ORG")
                                .help("The GitHub organization to fork repositories to.")
                                .default_value("meta-introspector"),
                        )
                        .arg(
                            Arg::new("target-branch")
                                .long("target-branch")
                                .short('b')
                                .value_name("BRANCH")
                                .help("The branch to checkout and use for dependencies.")
                                .default_value("feature/CRQ-016-nixify"),
                        )
                        .arg(
                            Arg::new("dry-run")
                                .long("dry-run")
                                .action(ArgAction::SetTrue)
                                .help("Perform a dry run without making actual changes."),
                        ),
                )
        )
        .subcommand(
            ClapCommand::new("add-workspace-submodules")
                .about("Adds all submodules as path dependencies to the root workspace.dependencies.")
        )
        .subcommand(
            ClapCommand::new("comment-submodule-workspaces")
                .about("Comments out [workspace] sections in submodule Cargo.toml files.")
        )
        .subcommand(
            ClapCommand::new("generate-workspace-deps")
                .about("Generates a comprehensive [workspace.dependencies] section for the root Cargo.toml.")
                .arg( // From CLI B GenerateWorkspacesArgs
                    Arg::new("project-root")
                        .long("project-root")
                        .default_value(".")
                        .help("The root directory of the project."),
                )
                .arg( // From CLI B GenerateWorkspacesArgs
                    Arg::new("output-dir")
                        .long("output-dir")
                        .default_value("generated_workspaces")
                        .help("The output directory for the generated workspaces."),
                )
                .arg( // From CLI B GenerateWorkspacesArgs
                    Arg::new("depgraph-dot-file")
                        .long("depgraph-dot-file")
                        .default_value("depgraph.dot")
                        .help("Path to the depgraph.dot file."),
                )
                .arg( // From CLI B GenerateWorkspacesArgs
                    Arg::new("tree-file")
                        .long("tree-file")
                        .default_value("tree.txt")
                        .help("Path to the tree.txt file."),
                ),
        )
}