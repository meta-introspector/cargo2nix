**Overall Goal:** Refactor the Git-related logic to consolidate it within the `git-wrapper-lib` crate, making `git2` an optional feature and `DummyGitExecutor` the default. This also involves ensuring declarations are in separate, smaller files. Additionally, refactor `cargo-submodule-tool-lib/src/cli/args.rs` into a lattice of modules. The overarching goal is to create super-fast, minimal-dependency, pure-functional adapters for Nix, Cargo/TOML, Git, and Syn, with dry-run and Serde modes, all controllable via a lattice of features for reproducible builds with Nix store/Git object inputs.

**Key Knowledge Gained/Confirmed:**
*   The project uses a multi-crate Rust workspace.
*   The refactoring involves moving traits, structs, and implementations related to Git and GitHub operations from `cargo-submodule-tool-lib` to `git-wrapper-lib`.
*   `git-wrapper-lib` is now the primary owner of Git execution and repository operation logic.
*   The `git2` crate functionality in `git-wrapper-lib` should be optional.
*   A `DummyGitExecutor` should be the default implementation if `git2` is not enabled.
*   The architecture prefers small, single-declaration files (`pure_rust_git_executor.rs`, `system_git_executor.rs`, etc.) rather than a large `git_implementations.rs`.
*   The `Execv` trait should be used consistently for external command execution.
*   `RollupLock`, `SubmoduleStat`, `SubmoduleInfo`, `FileMetadata`, `CapturedCommand`, `MergedCrateInfo` are central types that now reside in `git-wrapper-lib::git_types`.
*   `cargo-submodule-tool-lib` will depend on `git-wrapper-lib`.
*   `cargo-submodule-tool-lib/src/cli/args.rs` needs to be split into individual files for each argument struct.

**Completed Tasks:**
*   **Git Adapters:**
    *   Defined `GitAdapter` trait.
    *   Implemented `MockGitAdapter`, `ShellGitAdapter`, and `LibGitAdapter` (using `git2` crate).
    *   Created `SystemExecv` for shell command execution.
    *   Updated `git-wrapper-lib/src/lib.rs` to expose new modules.
    *   Updated `git-wrapper-lib/src/git_types.rs` to add `#[derive(Serialize, Deserialize)]` to `SubmoduleInfo`.
    *   Fixed `SubmoduleStat` field names in `git-wrapper-lib/src/git_adapters.rs`.
    *   Commented out `create_snapshot` calls in `git-wrapper-lib/src/pure_rust_git_executor.rs` and `git-wrapper-lib/src/system_git_executor.rs`.
    *   Fixed `IndexEntry::id()` method call to `id` field access in `git-wrapper-lib/src/pure_rust_git_executor.rs`.
*   **Cargo/TOML Adapters:**
    *   Implemented `CargoMetadataProvider` trait with `MockCargoMetadataProvider` and `RealCargoMetadataProvider` (lib mode using `cargo_metadata`).
    *   Updated `cargo-edit-tool/src/cargo_config_generator.rs` to use `&dyn GitAdapter` and `&dyn CargoMetadataProvider`.
    *   Added `#[derive(Serialize, Deserialize)]` to `PatchEntry` and `SubmoduleWorkspaceInfo` in `cargo-edit-tool/src/cargo_config_generator.rs`.
*   **Nix Adapters:**
    *   Defined `NixAdapter` trait and `CrateInfo` struct.
    *   Implemented `MockNixAdapter` and `ShellNixAdapter`.
    *   Updated `nix-generator-lib/src/lib.rs` to expose `nix_adapters` module.
    *   Added `serde` as a dependency to `nix-generator-lib/Cargo.toml`.
    *   Added `#[derive(Serialize, Deserialize)]` to `CrateInfo` in `nix-generator-lib/src/nix_adapters.rs`.
    *   Modified `nix-generator-lib/src/cli/commands/generate_nix.rs` to accept `&dyn NixAdapter` and use its methods.
*   **Syn Adapters:**
    *   Created `syn-adapter-lib` crate.
    *   Defined `SynAdapter` trait.
    *   Implemented `MockSynAdapter` and `LibSynAdapter` (using `syn` crate).
    *   Added `syn`, `quote`, `proc-macro2` as optional dependencies with `syn-parsing` feature to `syn-adapter-lib/Cargo.toml`.
*   **Mode Selection & Integration:**
    *   Created `cargo-edit-tool/src/adapters_factory.rs` with `Mode` enum and `get_adapters` factory function.
    *   Added `clap` as a dependency to `cargo-edit-tool/Cargo.toml`.
    *   Added `git-wrapper-lib`, `nix-generator-lib`, and `syn-adapter-lib` as dependencies to `cargo-edit-tool/Cargo.toml`.
    *   Updated `cargo-edit-tool/src/adapters_factory.rs` to include `NixAdapter` and `SynAdapter` in the factory function.
    *   Updated `cargo-edit-tool/src/main.rs` to parse command-line arguments for mode, use the adapter factory, and include a `GenerateNix` subcommand.
    *   Simplified `cargo_repo_sync_lib_test/src/main.rs`.
    *   Updated `cargo_repo_sync_lib_test/Makefile` to build and run `cargo-edit-tool`.
*   **Lattice of Features:**
    *   Defined `nix_generation` feature in `cargo-edit-tool/Cargo.toml`.

**File System State Changes (Summary):**

*   **Deleted from `git-wrapper-lib/src/`:**
    *   `git_executor.rs` (duplicate trait definition)
    *   `git_implementations.rs` (large consolidated file)
    *   `system_git_executor.rs` (duplicate implementation)
*   **Created in `git-wrapper-lib/src/`:**
    *   `pure_rust_git_executor.rs`
    *   `system_git_executor.rs`
    *   `dummy_git_executor.rs`
    *   `system_gh_executor.rs`
    *   `real_git_repository_operations.rs`
    *   `executors/system_execv.rs`
    *   `executors/mod.rs`
    *   `git_adapters.rs`
*   **Modified in `git-wrapper-lib/src/`:**
    *   `git_traits.rs`: Now defines `GitExecutor`, `GhExecutor`, `GitRepositoryOperations`, `Execv`. Imports types from `git_types`.
    *   `git_types.rs`: Added `use anyhow::{Context, Result};` and ensures all relevant types (`RollupLock`, `SubmoduleStat`, `SubmoduleInfo`, `FileMetadata`, `CapturedCommand`, `MergedCrateInfo`) are defined here. Added `#[derive(Serialize, Deserialize)]` to `SubmoduleInfo`.
    *   `lib.rs`: Updated to declare all new, smaller modules and remove the old `git_implementations` module. Added `pub mod executors;` and `pub mod git_adapters;`.
    *   `pure_rust_git_executor.rs`: Commented out `create_snapshot` calls. Fixed `IndexEntry::id()` method call to `id` field access.
    *   `system_git_executor.rs`: Commented out `create_snapshot` calls.
    *   `git_adapters.rs`: Implemented `MockGitAdapter`, `ShellGitAdapter`, and `LibGitAdapter`.
*   **Modified in `git-wrapper-lib/Cargo.toml`:** Added `serde` and `serde_json` dependencies.
*   **Deleted from `cargo-submodule-tool-lib/src/`:**
    *   `executors.rs`
    *   `executors/git_executor.rs`
    *   `executors/system_git_executor.rs`
    *   `repo_sync_lib/` (directory)
    *   `execv.rs`
    *   `traits/execv.rs`
    *   `repo_state_types.rs`
    *   `git_operations.rs`
    *   `execute-actions-plan.rs` (duplicate)
    *   `update-cargo-config.rs` (duplicate)
*   **Created in `cargo-submodule-tool-lib/src/cli/args/`:**
    *   `add_submodules.rs`
    *   `submodule_status.rs`
    *   `generate_nix.rs`
    *   `generate_patches.rs`
    *   `analyze.rs`
    *   `update_cargo_toml.rs`
    *   `process_tt_txt.rs`
    *   `collect_repo_state.rs`
    *   `generate_workspaces.rs`
*   **Modified in `cargo-submodule-tool-lib/`:**
    *   `Cargo.toml`: Removed `sha1`, `hex`, and `system_git` features, updated `git-wrapper-lib` dependency to be a path dependency, removed `dummy-git` feature.
    *   `src/lib.rs`: Removed module declarations for moved components.
    *   `src/git_operations.rs`: Imports updated to use `git_wrapper_lib::git_traits::GitExecutor` and `git_wrapper_lib::git_types::SubmoduleInfo`.
    *   `src/fs_cache.rs`: Imports updated to use `git_wrapper_lib::git_types::{FileMetadata, RollupLock}` and `git_wrapper_lib::git_traits::GitExecutor`.
    *   `src/cli/commands/collect_repo_state.rs`: Imports updated to use types and executors from `git_wrapper_lib`.
    *   `src/cli/commands/add_submodules.rs`: Imports updated to use types and executors from `git_wrapper_lib`.
    *   `src/cli/commands/analyze.rs`: Imports updated to use types and executors from `git_wrapper_lib`.
    *   `src/cli/commands/generate_patches.rs`: Imports updated to use types and executors from `git_wrapper_lib`.
    *   `src/cli/commands/submodule_status.rs`: Imports updated to use types and executors from `git_wrapper_lib`.
    *   `src/cli/run_commands.rs`: Uncommented `pub use` statements for updated commands.
    *   `src/bin/add_submodules.rs`: Imports updated to use types and executors from `git_wrapper_lib`.
    *   `src/repo_discovery.rs`: Updated to use `git_wrapper_lib::git_types::RepoInfo`.
    *   `src/plan_manager.rs`: Refactored to use `Execv` trait for git commands and updated imports.
    *   `src/execute_actions_plan.rs`: Refactored to use `Execv` trait for git commands and updated imports.
    *   `src/fs_writer.rs`: Updated to use `git_wrapper_lib::git_types::RollupLock`.
    *   `src/analysis/dep_graph_processor.rs`: Updated to use `git_wrapper_lib::git_types::MergedCrateInfo`.
    *   `src/analysis/layer0_analyzer.rs`: Updated to use `git_wrapper_lib::git_types::MergedCrateInfo`.
    *   `src/workspace_generator.rs`: Refactored to use `Execv` trait for cargo commands.
    *   `src/main.rs`: Re-added `CollectRepoState` command and updated imports.
    *   `src/cli/commands/mod.rs`: Re-added `collect_repo_state` module declaration.
    *   `src/cli/args.rs`: Refactored to declare `pub mod` for each individual argument file.
*   **Created `nix-generator-lib/src/nix_adapters.rs`:** Defined `NixAdapter` trait and `CrateInfo` struct, implemented `MockNixAdapter` and `ShellNixAdapter`.
*   **Modified `nix-generator-lib/src/lib.rs`:** Exposed `nix_adapters` module.
*   **Modified `nix-generator-lib/Cargo.toml`:** Added `serde` as a dependency.
*   **Modified `nix-generator-lib/src/nix_adapters.rs`:** Added `#[derive(Serialize, Deserialize)]` to `CrateInfo`.
*   **Modified `nix-generator-lib/src/cli/commands/generate_nix.rs`:** Updated to use `NixAdapter`.
*   **Created `syn-adapter-lib` crate:** Defined `SynAdapter` trait, implemented `MockSynAdapter` and `LibSynAdapter`. Added `syn`, `quote`, `proc-macro2` as optional dependencies with `syn-parsing` feature.
*   **Modified `cargo-edit-tool/Cargo.toml`:** Added `clap`, `nix-generator-lib`, and `syn-adapter-lib` as dependencies. Added `nix_generation` feature.
*   **Modified `cargo-edit-tool/src/adapters_factory.rs`:** Updated to include `NixAdapter` and `SynAdapter` in the factory function.
*   **Modified `cargo-edit-tool/src/main.rs`:** Updated to parse command-line arguments for mode, use the adapter factory, and include `GenerateNix` subcommand.
*   **Modified `cargo_repo_sync_lib_test/src/main.rs`:** Simplified to be a minimal test harness.
*   **Modified `cargo_repo_sync_lib_test/Makefile`:** Updated to build and run `cargo-edit-tool`.

**Warnings to Address:**
*   **Unused Imports:** Many warnings about unused imports in `git-wrapper-lib/src/submodule_manager.rs`, `git-wrapper-lib/src/pure_rust_git_executor.rs`, `git-wrapper-lib/src/system_git_executor.rs`, `git-wrapper-lib/src/git_types.rs`, `git-wrapper-lib/src/git_adapters.rs`.
*   **Unexpected `cfg` conditions:** Warnings from submodules (`atomic-maybe-uninit`, `dashmap`, `indexmap`, `im-rc`, `filetime`, `nix`, `shell-escape`, `encoding_rs`, `wasm-bindgen`) about unexpected `cfg` conditions. These are likely due to the workspace setup and may not be directly fixable without modifying the submodules themselves.
*   **Deprecated Methods:** Warnings from submodules (`pasetors`, `indexmap`, `nix`, `tera`, `lazy-static`) about deprecated methods.
*   **Resolver/Edition Warnings:** Warnings about `resolver` and `edition` in `Cargo.toml` files from submodules or workspace configuration.

**Current Plan (Next Steps):**
1.  **Address Unused Imports:** Go through the files with unused import warnings and remove the unused `use` statements.
2.  **Re-evaluate `create_snapshot`:** Determine the intended purpose of `create_snapshot` and how it fits into the new adapter architecture. If it's still needed, it should be refactored to use the new adapter traits.
3.  **Consider other warnings:** Evaluate if other warnings (deprecated methods, unexpected cfgs) need to be addressed, or if they are acceptable given they originate from submodules.