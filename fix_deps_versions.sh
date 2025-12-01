#!/bin/bash
set -x

# This script reads dependency lines (e.g., from a grep result like 'file:dep_line')
# from standard input, extracts unique file paths, and then applies a series of
# sed transformations to those files to standardize dependency declarations.
#
# Transformations:
# 1. Remove 'default-features = false' and any preceding comma.
# 2. Replace 'version = "X.Y.Z"' with 'workspace=true'.
# 3. Remove 'path = "..."' and any preceding comma.
# 4. Clean up any leftover leading commas inside curly braces.
#
# Usage: cat <input_file_with_dep_lines> | ./fix_deps_versions.sh

# Function to display usage information
usage() {
    echo "Usage: cat <input_file_with_dep_lines> | $0"
    echo "  <input_file_with_dep_lines> should contain lines in the format 'file_path:dependency_line'."
    echo "  Example: cat gix.todo | $0"
    exit 1
}

# Check if input is provided via stdin
if [ -t 0 ]; then # Check if stdin is a terminal (i.e., no pipe input)
    echo "Error: No input provided via standard input."
    usage
fi

# Create a temporary file to store unique file paths
TEMP_UNIQUE_FILES=$(mktemp)

# Read input from stdin, extract file paths, and store unique ones in the temporary file
cut -d':' -f1 | sort -u > "$TEMP_UNIQUE_FILES"

echo "Applying sed transformations to the identified files..."

# Loop through each unique file path and apply the sed commands
while IFS= read -r file_path; do
    if [ -f "$file_path" ]; then
        echo "Processing $file_path..."
        sed -i -E 's/, default-features = false//g' "$file_path"
        sed -i -E 's/default-features = false//g' "$file_path"
        sed -i -E 's/version = "[0-9^.]*"/workspace=true/g' "$file_path"
        sed -i -E 's/, path = "[^"]*"//g' "$file_path"
        sed -i -E 's/path = "[^"]*"//g' "$file_path"
        sed -i -E 's/\{\s*, /\{/g' "$file_path"
    else
        echo "Warning: File not found: $file_path. Skipping."
    fi
done < "$TEMP_UNIQUE_FILES"

# Clean up the temporary file
rm "$TEMP_UNIQUE_FILES"

echo "Finished applying transformations."
