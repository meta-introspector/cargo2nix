#!/usr/bin/env bash

# Define modules and their corresponding feature flags
declare -A modules_to_patch=(
    ["anyhow"]="anyhow_enabled"
    ["serde"]="serde_enabled"
    ["serde_json"]="serde_json_enabled"
    ["toml_edit"]="toml_edit_enabled"
    ["git_wrapper_lib"]="git_wrapper_lib_enabled"
    ["cargo_submodule_tool_lib"]="cargo_submodule_tool_lib_enabled"
    ["clap"]="clap_enabled"
    ["pathdiff"]="pathdiff_enabled"
    ["cargo_metadata"]="cargo_metadata_enabled"
    ["walkdir"]="walkdir_enabled"
    ["syn"]="syn_enabled"
    ["cargo_repo_sync_lib"]="cargo_repo_sync_lib_enabled"
    ["git2"]="git2_enabled"
    ["sha1"]="sha1_enabled"
    ["hex"]="hex_enabled"
    ["tool_traits_lib"]="tool_traits_lib_enabled"
)

# Directory for temporary files and logs
TEMP_DIR="temp_feature_builds"
LOG_DIR="feature_build_logs"
mkdir -p "$TEMP_DIR" "$LOG_DIR"

# Clean previous logs
rm -f "$LOG_DIR"/*.log

echo "Starting diagnostic build for each feature..."

for module in "${!modules_to_patch[@]}"; do
    feature="${modules_to_patch[$module]}"
    LOG_FILE="$LOG_DIR/${feature}.log"
    echo "--- Diagnosing feature: $feature ---" | tee -a "$LOG_FILE"

    # Create a temporary copy of the entire cargo-submodule-tool-lib src
    TEMP_SRC_DIR="$TEMP_DIR/cargo-submodule-tool-lib-src-$feature"
    rm -rf "$TEMP_SRC_DIR"
    cp -r cargo-submodule-tool-lib/src "$TEMP_SRC_DIR"

    # Apply #[cfg(feature = "...")] for use statements for this specific feature
    find "$TEMP_SRC_DIR" -name "*.rs" -print0 | xargs -0 -I {} bash -c '
        file="$1"
        module_name="$2"
        feature_name="$3"
        # Use a different sed delimiter to avoid issues with / in paths
        # Only patch if the line is not already cfg-ed
        grep -q -E "use ($module_name|${module_name//_/-})" "$file" && ! grep -q -P "^\s*#\[cfg\(feature = \"$feature_name\"\)\]\s*\n\s*use \($module_name\|${module_name//_/-}\)\b" "$file" && \
        sed -i "\#use \($module_name\|${module_name//_/-}\)#i\#\[cfg\(feature = \"$feature_name\"\)\]" "$file"
    ' _ {} "$module" "$feature"

    # Apply #[cfg(feature = "serde_enabled")] for serde derives if this is the serde feature
    if [ "$feature" == "serde_enabled" ]; then
        find "$TEMP_SRC_DIR" -name "*.rs" -print0 | xargs -0 -I {} bash -c '
            file="$1"
            # Only patch if the line is not already cfg-ed
            grep -q -E "#\[derive\((.*,)?serde::(Serialize|Deserialize)(,.*)?\)\]" "$file" && ! grep -q -P "^\s*#\[cfg\(feature = \"serde_enabled\"\)\]\s*\n\s*#\[derive\((.*,)?serde::(Serialize|Deserialize)(,.*)?\)\]" "$file" && \
            sed -i "\#\#\[derive\((.*,)?serde::(Serialize|Deserialize)(,.*)?\)\]#i\#\[cfg\(feature = \"serde_enabled\"\)\]" "$file"
        ' _ {}
    fi

    echo "Compiling cargo-submodule-tool-lib with feature '$feature' enabled..." | tee -a "$LOG_FILE"
    rustc --crate-name cargo_submodule_tool_lib \
          "$TEMP_SRC_DIR"/lib.rs \
          --edition=2021 \
          --crate-type lib \
          --emit=dep-info,metadata,link \
          -C embed-bitcode=no \
          -C debuginfo=2 \
          --check-cfg 'cfg(docsrs,test)' \
          --cfg "feature=\"$feature\"" \
          --out-dir target/debug/deps \
          -L dependency=target/debug/deps \
          2>&1 | tee -a "$LOG_FILE"

    echo "--- Finished diagnosing feature: $feature ---" | tee -a "$LOG_FILE"
    echo "" | tee -a "$LOG_FILE"
done

echo "Diagnostic complete. Check logs in $LOG_DIR."