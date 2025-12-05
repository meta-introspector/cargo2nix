# Abstract Git Operations

## Goal
Refactor the `cargo-submodule-tool-lib` crate to abstract all Git operations behind the `GitExecutor` trait. This will allow for flexible switching between `git2` (pure Rust) and `SystemGitExecutor` (system `git` command) implementations, and improve testability.

## Current Status
- The `GitExecutor` trait has been defined with methods for `submodule_add`, `checkout_branch`, `status`, `list_submodules`, `clone`, and `get_file_git_info`.
- Implementations for `SystemGitExecutor` and `PureRustGitExecutor` exist for these methods.
- `src/git_operations.rs` has been updated to use `GitExecutor`.
- `src/submodule_manager.rs` has been updated to use `GitExecutor` for `is_submodule`, `clone_repository`, `checkout_branch`, and `manage_remotes` (partially).
- `src/analysis/repo_state_collector.rs` has been updated to use `GitExecutor`.
- `src/cli/commands/collect_repo_state.rs` has been updated to initialize and pass `GitExecutor`.
- `src/cli/commands/add_submodules.rs` has been updated to initialize and pass `GitExecutor`.
- `src/fs_cache.rs` needs to be updated to use `GitExecutor` for git-related file metadata.

## Remaining Tasks

1.  **Update `src/fs_cache.rs`:**
    *   Modify `RealFileSystemStat` to use `GitExecutor` for `is_git_tracked` and `git_object_hash` in `get_metadata`.

2.  **Refactor remaining files using `git2` or direct `git` commands:**
    *   `/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/tools/cargo-submodule-tool-lib/src/cli/commands/submodule_status.rs`
    *   `/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/tools/cargo-submodule-tool-lib/src/cli/commands/generate_nix.rs`
    *   `/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/tools/cargo-submodule-tool-lib/src/cli/commands/generate_patches.rs`
    *   `/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/tools/cargo-submodule-tool-lib/src/cli/commands/analyze.rs`
    *   `/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/tools/cargo-submodule-tool-lib/src/cli/commands/update_cargo_toml.rs`
    *   `/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/tools/cargo-submodule-tool-lib/src/repo_sync_lib/git_snapshot.rs`
    *   `/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/tools/cargo-submodule-tool-lib/src/repo_sync_lib/run_submodule_status.rs`
    *   `/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/tools/cargo-submodule-tool-lib/src/cargo_config_generator.rs`
    *   `/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/tools/cargo-submodule-tool-lib/src/plan_manager.rs`
    *   `/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/tools/cargo-submodule-tool-lib/src/workspace_deps_generator.rs`

3.  **Review and update `commit_and_push_submodule` in `src/submodule_manager.rs`:**
    *   Implement the `#[cfg(not(feature = "git2"))]` branch using `SystemGitExecutor` for status, add, commit, and push operations. This will require adding corresponding methods to `GitExecutor`.

## Next Steps
Continue with the refactoring of `src/fs_cache.rs`.
