#!/usr/bin/env bash

# This script iterates through top-level Rust projects,
# identifies all optional features, and builds them with those features enabled.

# Function to extract optional features from a Cargo.toml
get_optional_features() {
    local cargo_toml_path="$1"
    local features=""

    # Read the Cargo.toml content
    content=$(cat "$cargo_toml_path")

    # Flag to indicate if we are in the [features] section
    in_features_section=0

    while IFS= read -r line; do
        # Check for [features] section start
        if [[ "$line" =~ ^\[features\] ]]; then
            in_features_section=1
            continue
        fi

        # Check for end of [features] section (start of another section)
        if [[ "$in_features_section" -eq 1 && "$line" =~ ^\[[a-zA-Z0-9._-]+\] ]]; then
            in_features_section=0
            break
        fi

        # If in [features] section and line defines a feature
        if [[ "$in_features_section" -eq 1 && "$line" =~ ^[[:space:]]*([a-zA-Z0-9_-]+)[[:space:]]*= ]]
        then
            feature_name="${BASH_REMATCH[1]}"
            # Exclude 'default' feature
            if [[ "$feature_name" != "default" ]]; then
                features+=" $feature_name"
            fi
        fi
    done <<< "$content"

    echo "$features"
}

# List of top-level project directories (same as identified before)
base_path="/data/data/com.termux.nix/files/home/experiments/pick-up-nix/vendor/nix/cargo2nix/tools/"
project_directories=(
    "cargo-submodule-tool-lib"
    "workspaces"
    "tool-traits-lib"
    "real-walkdir-adapter-lib"
    "real-regex-adapter-lib"
    "real-toml-adapter-lib"
    "rust-bootstrap-nix"
    "cargo-workspace-from-tree"
    "cargo-edit-lib"
    "cargo-edit-tool"
    "cargo-toml-editor-lib"
    "cargo-vendormod"
    "dep2submodule"
    "feature-permutation-builder"
    "git-wrapper-lib"
    "nix-generator-lib"
    "repo_manager"
    "secret-manager-rs"
    "cargo-feature-adapter"
    "syn-adapter-lib"
    "cargo-submodule-tool"
    "generate_workspace_deps"
    "cargo-repo-sync-lib"
)

all_build_results=""
error_summary=""
build_success_count=0
build_failure_count=0

for project_dir_relative in "${project_directories[@]}"; do
    project_path="${base_path}${project_dir_relative}"
    cargo_toml_path="${project_path}/Cargo.toml"

    if [ -f "$cargo_toml_path" ]; then
        echo "--- Processing project: $project_dir_relative ---"
        optional_features=$(get_optional_features "$cargo_toml_path")
        
        build_command="cargo build"
        if grep -q "\[workspace\]" "$cargo_toml_path"; then
            build_command+=" --workspace"
        fi

        if [ -n "$optional_features" ]; then
            build_command+=" --features \"$optional_features\""
        fi

        echo "Running: $build_command in $project_path"
        
        # Capture output
        # Using a temporary file to capture output more reliably
        temp_output_file=$(mktemp)
        (cd "$project_path" && $build_command) &> "$temp_output_file"
        build_status=$?
        build_output=$(cat "$temp_output_file")
        rm "$temp_output_file"

        if [ $build_status -eq 0 ]; then
            echo "Build successful for $project_dir_relative"
            build_success_count=$((build_success_count + 1))
        else
            echo "Build FAILED for $project_dir_relative"
            build_failure_count=$((build_failure_count + 1))
            error_summary+="### Build Failed for $project_dir_relative\n"
            error_summary+="
```\n$build_output\n```\n\n"
            echo "--- Raw output for $project_dir_relative (FAILED) ---" # Debugging line
            echo "$build_output" # Debugging line
            echo "-----------------------------------------------------" # Debugging line
        fi
        all_build_results+="\n--- Output for $project_dir_relative ---\n$build_output\n"
    else
        echo "Skipping $project_dir_relative: Cargo.toml not found at $cargo_toml_path"
    fi
done

echo "$all_build_results" > build_errors_raw.log
echo "## Build Summary\n" > build_errors_report.md
echo "Total projects built: $((build_success_count + build_failure_count))\n" >> build_errors_report.md
echo "Successful builds: $build_success_count\n" >> build_errors_report.md
echo "Failed builds: $build_failure_count\n" >> build_errors_report.md
echo "$error_summary" >> build_errors_report.md

echo "Raw build output saved to build_errors_raw.log"
echo "Build report saved to build_errors_report.md"