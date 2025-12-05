// crates/tool-refactorings/src/cargo_toml_patcher_config.rs

use std::collections::HashMap;

/// Trait for providing configuration for `Cargo.toml` patching operations.
pub trait CargoTomlPatcherConfig {
    /// Returns a map of external dependency names to their corresponding feature flags.
    fn get_external_deps(&self) -> &HashMap<String, String>;
    /// Returns a map of local dependency names to their corresponding feature flags.
    fn get_local_deps(&self) -> &HashMap<String, String>;
    /// Provides the dependency string for a given external module (e.g., "{ workspace = true, optional = true }").
    fn get_external_dep_string(&self, module_name: &str) -> String;
    /// Provides the dependency string for a given local module (e.g., "{ path = \"../crate\", optional = true }").
    fn get_local_dep_string(&self, module_name: &str) -> String;
}

/// Dummy implementation of `CargoTomlPatcherConfig` with hardcoded values.
pub struct DefaultCargoTomlPatcherConfig;

impl CargoTomlPatcherConfig for DefaultCargoTomlPatcherConfig {
    fn get_external_deps(&self) -> &HashMap<String, String> {
        static EXTERNAL_DEPS: once_cell::sync::Lazy<HashMap<String, String>> = 
            once_cell::sync::Lazy::new(|| {
                let mut m = HashMap::new();
                m.insert("anyhow".to_string(), "anyhow_enabled".to_string());
                m.insert("serde".to_string(), "serde_enabled".to_string());
                m.insert("serde_json".to_string(), "serde_json_enabled".to_string());
                m.insert("toml_edit".to_string(), "toml_edit_enabled".to_string());
                m.insert("clap".to_string(), "clap_enabled".to_string());
                m.insert("pathdiff".to_string(), "pathdiff_enabled".to_string());
                m.insert("cargo_metadata".to_string(), "cargo_metadata_enabled".to_string());
                m.insert("walkdir".to_string(), "walkdir_enabled".to_string());
                m.insert("syn".to_string(), "syn_enabled".to_string());
                m.insert("git2".to_string(), "git2_enabled".to_string());
                m.insert("sha1".to_string(), "sha1_enabled".to_string());
                m.insert("hex".to_string(), "hex_enabled".to_string());
                m.insert("md5".to_string(), "md5_enabled".to_string());
                m.insert("sha2".to_string(), "sha2_enabled".to_string());
                m.insert("lazy_static".to_string(), "lazy_static_enabled".to_string());
                m.insert("lru".to_string(), "lru_enabled".to_string());
                m.insert("url".to_string(), "url_enabled".to_string());
                m.insert("toml".to_string(), "toml_enabled".to_string());
                m
            });
        &EXTERNAL_DEPS
    }

    fn get_local_deps(&self) -> &HashMap<String, String> {
        static LOCAL_DEPS: once_cell::sync::Lazy<HashMap<String, String>> = 
            once_cell::sync::Lazy::new(|| {
                let mut m = HashMap::new();
                m.insert("cargo-edit-lib".to_string(), "cargo_edit_lib_enabled".to_string());
                m.insert("cargo-repo-sync-lib".to_string(), "cargo_repo_sync_lib_enabled".to_string());
                m.insert("cargo-submodule-tool-lib".to_string(), "cargo_submodule_tool_lib_enabled".to_string());
                m.insert("cargo-toml-editor-lib".to_string(), "cargo_toml_editor_lib_enabled".to_string());
                m.insert("git-wrapper-lib".to_string(), "git_wrapper_lib_enabled".to_string());
                m.insert("nix-generator-lib".to_string(), "nix_generator_lib_enabled".to_string());
                m.insert("real-regex-adapter-lib".to_string(), "real_regex_adapter_lib_enabled".to_string());
                m.insert("real-toml-adapter-lib".to_string(), "real_toml_adapter_lib_enabled".to_string());
                m.insert("real-walkdir-adapter-lib".to_string(), "real_walkdir_adapter_lib_enabled".to_string());
                m.insert("syn-adapter-lib".to_string(), "syn_adapter_lib_enabled".to_string());
                m.insert("tool-traits-lib".to_string(), "tool_traits_lib_enabled".to_string());
                m.insert("cargo2nix".to_string(), "cargo2nix_enabled".to_string());
                m
            });
        &LOCAL_DEPS
    }

    fn get_external_dep_string(&self, module_name: &str) -> String {
        match module_name {
            "anyhow" | "serde" | "serde_json" | "clap" | "pathdiff" | "cargo_metadata" | "walkdir" | "syn" | "git2" | "hex" | "md5" | "sha2" | "lazy_static" | "lru" | "url" | "toml" => {
                "{ workspace = true, optional = true }".to_string()
            }
            _ => "{ version = \"*\", optional = true }".to_string(),
        }
    }

    fn get_local_dep_string(&self, module_name: &str) -> String {
        match module_name {
            "cargo2nix" => "{ path = \"../../crates/cargo2nix\", optional = true }".to_string(),
            _ => format!("{{ path = \"../{}\", optional = true }}", module_name),
        }
    }
}
