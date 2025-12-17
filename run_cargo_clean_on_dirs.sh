#!/bin/bash

# This script reads a list of directories from /mnt/data1/nix/clean.txt
# and runs 'cargo clean' in each of them.
# It assumes the paths in clean.txt are relative to /mnt/data1/nix/

# Define the base directory for relative paths in clean.txt
BASE_DIR="/mnt/data1/nix"
INPUT_FILE="${BASE_DIR}/clean.txt"

# Check if the input file exists
if [ ! -f "$INPUT_FILE" ]; then
    echo "Error: Input file '$INPUT_FILE' not found."
    exit 1
fi

echo "Starting cargo clean process for directories listed in $INPUT_FILE"

# Read each directory from the input file
while IFS= read -r relative_dir; do
    # Skip empty lines
    if [ -z "$relative_dir" ]; then
        continue
    fi

    # Construct the full absolute path to the directory
    # Remove leading './' if present in relative_dir to avoid double slashes and ensure clean path
    cleaned_relative_dir=$(echo "$relative_dir" | sed 's/^\.\///')
    TARGET_DIR="${BASE_DIR}/${cleaned_relative_dir}"

    # Check if the directory exists
    if [ -d "$TARGET_DIR" ]; then
        echo "Entering directory: $TARGET_DIR"
        # Fix SC2181: Check exit code directly
        if (cd "$TARGET_DIR" && cargo clean); then
            echo "Successfully ran 'cargo clean' in $TARGET_DIR"
        else
            echo "Error: 'cargo clean' failed in $TARGET_DIR"
        fi
    else
        echo "Warning: Directory '$TARGET_DIR' not found. Skipping."
    fi
    echo "----------------------------------------"
done < "$INPUT_FILE"

echo "Cargo clean process completed."