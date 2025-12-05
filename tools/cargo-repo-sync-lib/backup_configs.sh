#!/usr/bin/env bash
set -euo pipefail

FILTERED_CONFIG_PATHS_FILE="$1"
CARGO_CONFIG_ROOT="$2"

echo "--- Backing up existing config.toml files ---"

# Backup files from the filtered list
while IFS= read -r file_path; do
    if [ -f "$file_path" ]; then
        cp "$file_path" "$file_path.bak"
        echo "Backed up $file_path to $file_path.bak"
    fi
done < "$FILTERED_CONFIG_PATHS_FILE"

# Backup the root config.toml if it exists
if [ -f "$CARGO_CONFIG_ROOT" ]; then
    cp "$CARGO_CONFIG_ROOT" "$CARGO_CONFIG_ROOT.bak"
    echo "Backed up $CARGO_CONFIG_ROOT to $CARGO_CONFIG_ROOT.bak"
fi

echo "--- Backup complete ---"
