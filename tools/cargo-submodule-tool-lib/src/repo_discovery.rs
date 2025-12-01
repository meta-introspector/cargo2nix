use anyhow::Context; // Keep anyhow::Context for now, will refactor later if needed

use crate::analysis::cargo_toml_parser::CurrentCargoTomlParser;
use crate::analysis::walkdir_iterator::CurrentWalkDirIterator;
use real_regex_adapter_lib::CurrentRegexMatcher;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use tool_traits_lib::cargo_toml_parser::CargoTomlParser;
use tool_traits_lib::cargo_toml_processor::CargoTomlProcessor;
use tool_traits_lib::regex_adapter::{RegexCaptures, RegexMatcher};
#[cfg(feature = "tool_traits_lib_enabled")]
use tool_traits_lib::serde_adapter::{CurrentSerdeAdapter, SerdeAdapter};
use tool_traits_lib::types::RepoState;
use tool_traits_lib::walkdir_iterator::WalkDirIterator;

pub trait RepoDiscoverer {
    fn discover_repos(
        &self,
        root_dir: &Path,
    ) -> std::result::Result<(Vec<RepoState>, HashSet<String>), String>;
}

pub struct PureRustRepoDiscoverer {
    target_org: String,
    target_branch: String,
}

impl PureRustRepoDiscoverer {
    pub fn new(target_org: String, target_branch: String) -> Self {
        PureRustRepoDiscoverer {
            target_org,
            target_branch,
        }
    }
}

impl RepoDiscoverer for PureRustRepoDiscoverer {
    fn discover_repos(
        &self,
        root_dir: &Path,
    ) -> std::result::Result<(Vec<RepoState>, HashSet<String>), String> {
        let mut discovered_repos: HashSet<RepoState> = HashSet::new();
        let mut all_vendored_crate_names: HashSet<String> = HashSet::new();

        let walkdir_iterator = CurrentWalkDirIterator::new(root_dir);
        use crate::analysis::cargo_toml_processor::RealCargoTomlProcessor;
        let cargo_toml_processor = RealCargoTomlProcessor; // Instantiate the processor

        for entry_result in walkdir_iterator.into_iter() {
            let path = entry_result.map_err(|e| format!("Error walking directory: {}", e))?;

            // Filter logic (moved from filter_entry)
            if path.ends_with("target")
                || path.ends_with("tests")
                || path.ends_with("examples")
                || path.ends_with("submodules/target")
                || path.ends_with("submodules/tests")
                || path.ends_with("submodules/examples")
            {
                continue;
            }

            if path.file_name().map_or(false, |name| name == "Cargo.toml") {
                cargo_toml_processor.process_cargo_toml(
                    &path,
                    &mut discovered_repos,
                    &mut all_vendored_crate_names,
                    &self.target_org,
                    &self.target_branch,
                )?;
            }
        }

        Ok((
            discovered_repos.into_iter().collect(),
            all_vendored_crate_names,
        ))
    }
}
