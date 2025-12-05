#!/bin/bash

MAIN_REPO_DIR="/mnt/data1/nix/vendor/rust/cargo2nix"
ERROR_LOG_FILE="build_logs/cargo2nix_errors.log"
CRATE_SCOPE="$1" # New: Optional argument to scope to a single crate

if [ -n "$CRATE_SCOPE" ]; then
    echo "--- Analyzing Failed Crates (Scoped to: $CRATE_SCOPE) ---"
else
    echo "--- Analyzing All Failed Crates from $ERROR_LOG_FILE ---"
fi
echo ""

if [ ! -f "$ERROR_LOG_FILE" ]; then
    echo "Error: $ERROR_LOG_FILE not found. Please run 'make main-report' first."
    exit 1
fi

# Ensure build_logs directory exists for standalone build logs
mkdir -p build_logs

# Determine problematic crate root paths from error log
PROBLEMATIC_CRATES=()

if [ -n "$CRATE_SCOPE" ]; then
    # If a scope is provided, only process that crate
    PROBLEMATIC_CRATES=("$CRATE_SCOPE")
else
    # Original logic to identify all problematic crates
    while IFS= read -r line; do
        CRATE_ROOT=""
        # Check if the line matches any of the known problematic paths
        if [[ "$line" =~ submodules/cargo/src/cargo/ ]]; then
            CRATE_ROOT="submodules/cargo"
        elif [[ "$line" =~ submodules/clap/clap_complete/ ]]; then
            CRATE_ROOT="submodules/clap/clap_complete"
        elif [[ "$line" =~ submodules/git2-rs/ ]]; then
            CRATE_ROOT="submodules/git2-rs"
        elif [[ "$line" =~ submodules/rusqlite/ ]]; then
            CRATE_ROOT="submodules/rusqlite"
        elif [[ "$line" =~ submodules/serde/serde_core/ ]]; then
            CRATE_ROOT="submodules/serde/serde_core"
        elif [[ "$line" =~ src/index.crates.io-[^/]+/either-1.13.0/ ]]; then
            CRATE_ROOT=$(echo "$line" | grep -oP 'src/index.crates.io-[^/]+/either-1.13.0')
        fi

        if [ -n "$CRATE_ROOT" ] && [[ ! " ${PROBLEMATIC_CRATES[*]} " == *" ${CRATE_ROOT} "* ]]; then
            PROBLEMATIC_CRATES+=("$CRATE_ROOT")
        fi
    done < "$ERROR_LOG_FILE"

    # Add Cargo.toml and Cargo.lock to problematic crates if not already present, as they are key sources of dependency config issues
    if [[ ! " ${PROBLEMATIC_CRATES[*]} " == *" Cargo.toml "* ]]; then
        PROBLEMATIC_CRATES+=("Cargo.toml")
    fi
    if [[ ! " ${PROBLEMATIC_CRATES[*]} " == *" Cargo.lock "* ]]; then
        PROBLEMATIC_CRATES+=("Cargo.lock")
    fi
fi # End of CRATE_SCOPE conditional

if [ ${#PROBLEMATIC_CRATES[@]} -eq 0 ]; then
    echo "No problematic crates identified from error log or provided scope. Exiting."
    exit 0
else
    echo "Identified problematic crates and relevant files: ${PROBLEMATIC_CRATES[*]}"
    echo ""
fi

# Loop through each problematic crate/file
for target_path in "${PROBLEMATIC_CRATES[@]}"; do
    echo "=================================================="
    echo "=== Processing Target: $target_path ==="
    echo "=================================================="
    echo ""

    CRATE_CARGO_TOML_PATH="$MAIN_REPO_DIR/$target_path/Cargo.toml"

    # 1. Attempt standalone build if it's a directory with Cargo.toml
    if [ -d "$MAIN_REPO_DIR/$target_path" ] && [ -f "$CRATE_CARGO_TOML_PATH" ]; then
        echo "--- Attempting Standalone Build for $target_path ---"
        BUILD_LOG="build_logs/$(basename "$target_path")_standalone_build_raw.log"
        
        # Navigate to the crate directory and build
        (
            cd "$MAIN_REPO_DIR/$target_path" || { echo "ERROR: Could not change directory to $MAIN_REPO_DIR/$target_path"; exit 1; }
            # Use Nix shell for consistent build environment
            nix develop --command bash -c "cargo build --message-format=json 2>&1" > "$MAIN_REPO_DIR/$BUILD_LOG" 2>&1 || true
        )

        echo "--- Standalone Build Errors for $target_path ---"
        FILTERED_ERRORS=$(grep '"level":"error"' "$MAIN_REPO_DIR/$BUILD_LOG" |
        grep '"reason":"compiler-message"' |
        sed 's/.*"message":{[^}]*"rendered":"\([^`]*\)".*/\1/' |
        sed 's/\\n/\n/g' |
        sed 's/\\t/\t/g' |
        sed 's/\\"/"/g' |
        sed 's/\\r//g' |
        sed 's/\\//g')

        echo "$FILTERED_ERRORS" > "$MAIN_REPO_DIR/build_logs/$(basename "$target_path")_standalone_errors.log"

        if [ -s "$MAIN_REPO_DIR/build_logs/$(basename "$target_path")_standalone_errors.log" ]; then
            cat "$MAIN_REPO_DIR/build_logs/$(basename "$target_path")_standalone_errors.log"
        else
            echo "No compiler errors found in standalone build for $target_path."
        fi
        echo ""
    elif [ "$target_path" == "Cargo.toml" ] || [ "$target_path" == "Cargo.lock" ]; then
        echo "--- Skipping Standalone Build for $target_path (not a buildable crate directory) ---"
        echo ""
    else
        echo "--- Cannot perform standalone build for $target_path (not a directory with Cargo.toml) ---"
        echo ""
    fi

    # 2. Run git info for the target path
    echo "--- Git Information for $target_path (from get_git_info.sh) ---"
    "$MAIN_REPO_DIR/get_git_info.sh" "$target_path"
    echo ""
done

echo "--- Analysis Complete ---"