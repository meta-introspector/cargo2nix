#!/usr/bin/env bash
set -euo pipefail

ABSOLUTE_CONFIG_FILE="$1"
FILTERED_CONFIG_PATHS_FILE="$2"
CARGO_CONFIG_ROOT="$3" # Added for explicit overwrite

echo "--- Overwriting all filtered config.toml files ---"

ABSOLUTE_CONFIG_CONTENT=$(cat "$ABSOLUTE_CONFIG_FILE")

while IFS= read -r file_path; do
    echo "Attempting to overwrite: $file_path"
    echo "$ABSOLUTE_CONFIG_CONTENT" > "$file_path"
    echo "Finished overwriting: $file_path"
    # Add a cat command to immediately verify the content after writing
    echo "Content of $file_path after overwrite:"
    cat "$file_path"
    echo "--- End content of $file_path ---"
done < "$FILTERED_CONFIG_PATHS_FILE"

# Explicitly overwrite the root config.toml with absolute paths
echo "--- Overwriting root $CARGO_CONFIG_ROOT with absolute paths ---"
echo "$ABSOLUTE_CONFIG_CONTENT" > "$CARGO_CONFIG_ROOT"

echo "--- All Cargo configurations updated ---"