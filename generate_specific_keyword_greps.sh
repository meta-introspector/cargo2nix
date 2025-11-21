#!/usr/bin/env bash

# Create the index directory if it doesn't exist
mkdir -p index/

# List of specific keywords to search for in .rs and Cargo.toml files
keywords=(
    "generate_cargo_nix"
    "CargoNixGenerator"
    "parse_manifest"
    "Manifest"
    "Dependency"
    "nix_expression"
    "to_nix"
    "write_file"
    "FileWriter"
    "discover_crate"
    "CrateDiscovery"
    # New keywords for Nix expression generation
    "Nix"
    "build_expr"
    "to_string_nix"
    "mkDerivation"
    "lib.attrsets"
)

# Iterate through each keyword and perform a grep search
for keyword in "${keywords[@]}"; do
    # Sanitize keyword for filename (replace spaces and special chars with underscores)
    filename=$(echo "$keyword" | sed 's/[^a-zA-Z0-9]/_/g')
    output_file="index/${filename}_specific.txt"

    if [ -f "$output_file" ]; then
        echo "Skipping '$keyword' as '$output_file' already exists."
        continue
    fi

    echo "Searching for '$keyword' in .rs and Cargo.toml files and saving to $output_file"
    grep -r -E "$keyword" --include "*.rs" --include "Cargo.toml" > "$output_file"
done

echo "Specific keyword search complete. Results are in the 'index/' directory."