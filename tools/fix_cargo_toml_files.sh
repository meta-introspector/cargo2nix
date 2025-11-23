#!/usr/bin/env bash

# Define external modules and their corresponding feature flags
declare -A external_deps=(
    ["anyhow"]="anyhow_enabled"
    ["serde"]="serde_enabled"
    ["serde_json"]="serde_json_enabled"
    ["toml_edit"]="toml_edit_enabled"
    ["clap"]="clap_enabled"
    ["pathdiff"]="pathdiff_enabled"
    ["cargo_metadata"]="cargo_metadata_enabled"
    ["walkdir"]="walkdir_enabled"
    ["syn"]="syn_enabled"
    ["git2"]="git2_enabled"
    ["sha1"]="sha1_enabled"
    ["hex"]="hex_enabled"
    ["md5"]="md5_enabled"
    ["sha2"]="sha2_enabled"
    ["lazy_static"]="lazy_static_enabled"
    ["lru"]="lru_enabled"
    ["url"]="url_enabled"
    ["toml"]="toml_enabled"
)

# Define local modules and their corresponding feature flags
declare -A local_deps=(
    ["cargo-edit-lib"]="cargo_edit_lib_enabled"
    ["cargo-repo-sync-lib"]="cargo_repo_sync_lib_enabled"
    ["cargo-submodule-tool-lib"]="cargo_submodule_tool_lib_enabled"
    ["cargo-toml-editor-lib"]="cargo_toml_editor_lib_enabled"
    ["git-wrapper-lib"]="git_wrapper_lib_enabled"
    ["nix-generator-lib"]="nix_generator_lib_enabled"
    ["real-regex-adapter-lib"]="real_regex_adapter_lib_enabled"
    ["real-toml-adapter-lib"]="real_toml_adapter_lib_enabled"
    ["real-walkdir-adapter-lib"]="real_walkdir_adapter_lib_enabled"
    ["syn-adapter-lib"]="syn_adapter_lib_enabled"
    ["tool-traits-lib"]="tool_traits_lib_enabled"
    ["cargo2nix"]="cargo2nix_enabled" # This is in ../../crates/cargo2nix
)

echo "Standardizing cargo_metadata version to 1.0..."
find . -name "Cargo.toml" -print0 | xargs -0 sed -i 's/cargo_metadata = "0.18"/cargo_metadata = { workspace = true, optional = true }/g'
find . -name "Cargo.toml" -print0 | xargs -0 sed -i 's/cargo_metadata = "0.18.1"/cargo_metadata = { workspace = true, optional = true }/g'
find . -name "Cargo.toml" -print0 | xargs -0 sed -i 's/cargo_metadata = { version = "1.0", optional = true }/cargo_metadata = { workspace = true, optional = true }/g'


echo "Patching Cargo.toml files with dependencies and features..."

# Function to add dependency and feature to Cargo.toml
add_dependency_and_feature() {
    local cargo_toml_file="$1"
    local module_name="$2"
    local feature_name="$3"
    local dep_string="$4" # The full dependency string, e.g., '{ workspace = true, optional = true }' or '{ path = "../crate", optional = true }'

    echo "Processing Cargo.toml: $cargo_toml_file for module: $module_name, feature: $feature_name"

    # Ensure [dependencies] section exists
    if ! grep -q "^\\[dependencies\\]" "$cargo_toml_file"; then
        echo "Adding [dependencies] section to $cargo_toml_file"
        echo -e "\n[dependencies]\n" >> "$cargo_toml_file"
    fi

    # Add dependency if it doesn't exist
    if ! grep -q "^$module_name = " "$cargo_toml_file"; then
        echo "Adding dependency $module_name = $dep_string to $cargo_toml_file"
        # Insert after [dependencies] or the last dependency if [dependencies] is not empty
        sed -i "/^\\[dependencies\\]/a $module_name = $dep_string" "$cargo_toml_file"
    fi

    # Ensure [features] section exists
    if ! grep -q "^\\[features\\]" "$cargo_toml_file"; then
        echo "Adding [features] section to $cargo_toml_file"
        echo -e "\n[features]\n" >> "$cargo_toml_file"
    fi

    # Add feature if it doesn't exist
    if ! grep -q "^$feature_name = " "$cargo_toml_file"; then
        echo "Adding feature $feature_name = [\"dep:$module_name\"] to $cargo_toml_file"
        # Insert after [features] or the last feature if [features] is not empty
        sed -i "/^\\[features\\\]/a $feature_name = [\"dep:$module_name\"]" "$cargo_toml_file"
    fi
}

# Find all Cargo.toml files and apply the patching logic
find . -name "Cargo.toml" -print0 | while IFS= read -r -d $''$ '\0' cargo_toml_file; do
    # Process external dependencies
    for module in "${!external_deps[@]}"; do
        feature="${external_deps[$module]}"
        # Use workspace = true for common external dependencies, otherwise version = "*"
        if [[ "$module" == "anyhow" || "$module" == "serde" || "$module" == "serde_json" || "$module" == "clap" || "$module" == "pathdiff" || "$module" == "cargo_metadata" || "$module" == "walkdir" || "$module" == "syn" || "$module" == "git2" || "$module" == "hex" || "$module" == "md5" || "$module" == "sha2" || "$module" == "lazy_static" || "$module" == "lru" || "$module" == "url" || "$module" == "toml" ]]; then
            dep_string="{ workspace = true, optional = true }"
        else
            dep_string="{ version = \"*\", optional = true }"
        fi
        add_dependency_and_feature "$cargo_toml_file" "$module" "$feature" "$dep_string"
    done

    # Process local dependencies
    for module in "${!local_deps[@]}"; do
        feature="${local_deps[$module]}"
        if [[ "$module" == "cargo2nix" ]]; then
            dep_string="{ path = \"../../crates/cargo2nix\", optional = true }"
        else
            dep_string="{ path = \"../${module}\", optional = true }"
        fi
        add_dependency_and_feature "$cargo_toml_file" "$module" "$feature" "$dep_string"
    done
done
