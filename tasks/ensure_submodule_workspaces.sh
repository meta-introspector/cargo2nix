#!/bin/bash
set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
PROJECT_ROOT=$(dirname "$SCRIPT_DIR") # This assumes tasks/ is directly under PROJECT_ROOT

echo "Ensuring all submodules have a [workspace] section in their Cargo.toml..."

# Iterate through all directories in the submodules folder
for submodule_dir in "$PROJECT_ROOT"/submodules/*/; do
    if [ -d "$submodule_dir" ]; then
        submodule_name=$(basename "$submodule_dir")
        cargo_toml="$submodule_dir/Cargo.toml"

        if [ -f "$cargo_toml" ]; then
            if ! grep -q "^\[workspace\]" "$cargo_toml"; then
                echo "Adding [workspace] to $submodule_name/Cargo.toml"
                echo -e "\n[workspace]" >> "$cargo_toml"
                # Add to git staging area if this script is run as part of a commit process
                # git -C "$submodule_dir" add Cargo.toml
            else
                echo "[workspace] already exists in $submodule_name/Cargo.toml"
            fi
        else
            echo "No Cargo.toml found in $submodule_name"
        fi
    fi
done

echo "Finished ensuring [workspace] sections for submodules."
