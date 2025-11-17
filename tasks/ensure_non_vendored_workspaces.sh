#!/usr/bin/env bash
set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
PROJECT_ROOT=$(dirname "$SCRIPT_DIR") # This assumes tasks/ is directly under PROJECT_ROOT
NON_VENDORED_MODULES_FILE="$PROJECT_ROOT/.cargo/non_vendored_modules.txt"

DRY_RUN=false
if [[ "$1" == "--dry-run" ]]; then
    DRY_RUN=true
    echo "Running in DRY-RUN mode. No changes will be written to files."
fi

echo "Ensuring non-vendored modules have a [workspace] section in their Cargo.toml..."

if [ ! -f "$NON_VENDORED_MODULES_FILE" ]; then
    echo "Error: Non-vendored modules list not found at $NON_VENDORED_MODULES_FILE"
    echo "Please run 'make -B non-vendored -f Makefile.cargoconfig' first to generate it."
    exit 1
fi

# Read the list of non-vendored modules
while IFS= read -r line; do
    module_name=$(echo "$line" | awk '{print $1}') # Extract module name from "name count" line

    # Determine the potential path to Cargo.toml
    # Prioritize submodules/, then vendor/
    cargo_toml=""
    if [ -d "$PROJECT_ROOT/submodules/$module_name" ]; then
        cargo_toml="$PROJECT_ROOT/submodules/$module_name/Cargo.toml"
    elif [ -d "$PROJECT_ROOT/vendor/$module_name" ]; then
        cargo_toml="$PROJECT_ROOT/vendor/$module_name/Cargo.toml"
    fi

    if [ -f "$cargo_toml" ]; then
        if ! grep -q "^\[workspace\]" "$cargo_toml"; then
            echo "ACTION: Would add [workspace] to $module_name/Cargo.toml ($cargo_toml)"
            if [ "$DRY_RUN" = false ]; then
                echo -e "\n[workspace]" >> "$cargo_toml"
                # Optionally, add to git staging area if this script is run as part of a commit process
                # git -C "$(dirname "$cargo_toml")" add Cargo.toml
            fi
        else
            echo "INFO: [workspace] already exists in $module_name/Cargo.toml ($cargo_toml)"
        fi
    else
        echo "WARNING: Cargo.toml not found for non-vendored module $module_name at expected paths."
    fi
done < "$NON_VENDORED_MODULES_FILE"

echo "Finished ensuring [workspace] sections for non-vendored modules."