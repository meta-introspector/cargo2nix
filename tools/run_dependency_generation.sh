#!/usr/bin/env bash

# Define temporary file names
PACKAGE_NAMES_WITH_CONTEXT="package_names_with_context.txt"
PACKAGE_NAME_LINES="package_name_lines.txt"
EXTRACTED_PACKAGES="extracted_packages.txt"
PACKAGE_AND_DIR="package_and_dir.txt"
FILTERED_PACKAGES="filtered_packages.txt"
PYTHON_SCRIPT="generate_deps_from_names_txt.py"

# Define exclusion keywords
EXCLUDE_KEYWORDS="bench|test|example|fuzz|target|temp|docs|examples|tests"

# Step 1: Find package names and context
echo "Step 1: Finding package names and context..."
grep -r -B3 "^name =" --include "Cargo.toml" > "$PACKAGE_NAMES_WITH_CONTEXT"

# Step 2: Extract lines containing package names
echo "Step 2: Extracting lines containing package names..."
grep -E 'name = ' "$PACKAGE_NAMES_WITH_CONTEXT" > "$PACKAGE_NAME_LINES"

# Step 3: Extract file paths and package names
echo "Step 3: Extracting file paths and package names..."
awk -F':name = "' '{print $1 "," $2}' "$PACKAGE_NAME_LINES" | awk -F'"' '{print $1 "," $2}' > "$EXTRACTED_PACKAGES"

# Step 4: Extract package names and their directory paths
echo "Step 4: Extracting package names and their directory paths..."
awk -F',' '{print $2 "," (gensub(/\/[^/]*$/, "", "g", $1))}' "$EXTRACTED_PACKAGES" > "$PACKAGE_AND_DIR"

# Step 5: Filter package names and paths based on exclusion keywords
echo "Step 5: Filtering package names and paths..."
grep -vE "$EXCLUDE_KEYWORDS" "$PACKAGE_AND_DIR" > "$FILTERED_PACKAGES"

# Step 6: Run the Python script with the filtered package list
echo "Step 6: Generating dependencies using Python script..."
python "$PYTHON_SCRIPT" "$FILTERED_PACKAGES"

echo "Process completed."