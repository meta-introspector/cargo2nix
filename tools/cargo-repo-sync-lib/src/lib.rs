pub mod cargo_config_generator;
pub mod repo_state_types;
pub mod executors;
pub mod traits;
pub mod fs_cache;
pub mod fs_writer;
pub mod repo_discovery;
pub mod cargo_toml_patcher;
pub mod plan_manager;
pub use crate::plan_manager::*;
pub mod submodule_manager;
pub use crate::submodule_manager::*;
pub mod workspace_generator;
pub use crate::workspace_generator::*;
pub mod git_operations;

// New modules for split-out declarations
pub mod repo_sync_lib;
pub mod analysis;
pub mod workspace_deps_generator;

// Re-export everything from the new modules
pub use repo_sync_lib::repo_action::RepoAction;
pub use repo_sync_lib::submodule_stat::SubmoduleStat;
pub use repo_sync_lib::rollup_lock::RollupLock;
pub use repo_sync_lib::submodule_stat_provider::SubmoduleStatProvider;
pub use repo_sync_lib::real_submodule_stat_provider::RealSubmoduleStatProvider;
pub use repo_sync_lib::cached_submodule_stat_provider::CachedSubmoduleStatProvider;
pub use repo_sync_lib::execute_actions_plan::execute_actions_plan;
pub use repo_sync_lib::update_cargo_config::update_cargo_config;
pub use repo_sync_lib::repo_sync_config::RepoSyncConfig;
pub use repo_sync_lib::run_submodule_status::run_submodule_status;