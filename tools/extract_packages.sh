#!/bin/bash

# Define keywords to exclude
EXCLUDE_KEYWORDS="bench|test|example|fuzz|target|temp|docs|examples|tests"

# Find all Cargo.toml files, excluding those in directories matching exclude keywords
find . -name "Cargo.toml" | while read -r cargo_toml_path; do
    # Get the directory of the Cargo.toml file
    cargo_toml_dir=$(dirname "$cargo_toml_path")

    # Check if the directory path contains any exclude keywords
    if echo "$cargo_toml_dir" | grep -qE "$EXCLUDE_KEYWORDS"; then
        continue # Skip this file if its path contains an excluded keyword
    fi

    # Extract the package name
    package_name=$(grep -E '^name = ' "$cargo_toml_path" | head -n 1 | awk -F'=' '{print $2}' | tr -d '" ' | tr -d '\r')

    # If a package name is found and it's not empty
    if [ -n "$package_name" ]; then
        # Check if the package name itself contains any exclude keywords
        if echo "$package_name" | grep -qE "$EXCLUDE_KEYWORDS"; then
            continue # Skip this package if its name contains an excluded keyword
        fi
        echo "Package: $package_name, Path: $cargo_toml_dir"
    fi
done
