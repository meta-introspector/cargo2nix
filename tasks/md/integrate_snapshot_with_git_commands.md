name = "Integrate Snapshot with Git Commands"
description = "Integrate `create_snapshot` calls after state-modifying Git commands (`submodule_add`, `checkout_branch`) in `GitExecutor` implementations and update all call sites to ensure immediate cache invalidation."
status = "in-progress"
steps = [
    "Modify `GitExecutor` trait to accept `Arc<Mutex<RollupLock>>` and `PathBuf` for `root_dir`.",
    "Update `SystemGitExecutor` and `PureRustGitExecutor` to store `rollup_lock` and `root_dir`.",
    "Call `create_snapshot` within `submodule_add` and `checkout_branch` implementations of both executors.",
    "Update call sites in `src/repo_sync_lib/run_submodule_status.rs` to pass `rollup_lock` and `root_dir`.",
    "Update call sites in `src/repo_sync_lib/execute_actions_plan.rs` to pass `rollup_lock` and `root_dir`.",
    "Update `tools/cargo-vendormod/src/main.rs` to pass `rollup_lock` and `root_dir` to its `execute_actions_plan` function and call `create_snapshot` after relevant commands.",
    "Update `run_generate_nix_command` in `src/cli/run_commands.rs` to correctly initialize `git_executor` with `rollup_lock` and `root_dir`."
]