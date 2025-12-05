#!/usr/bin/env bash

# Create the index directory if it doesn't exist
mkdir -p index/

# List of keywords to search for in .rs files
keywords=(
    "fn main"
    "#[derive("
    "use crate::"
    "nix"
    "parse"
    "generate"
    "struct"
    "enum"
    "impl"
    "pub fn"
)

# Iterate through each keyword and perform a grep search
for keyword in "${keywords[@]}"; do
    # Sanitize keyword for filename (replace spaces and special chars with underscores)
    filename=$(echo "$keyword" | sed 's/[^a-zA-Z0-9]/_/g')
    echo "Searching for '$keyword' in .rs files and saving to index/${filename}.txt"
    grep -r -E "$keyword" --include "*.rs" > "index/${filename}.txt"
done

echo "Keyword search complete. Results are in the 'index/' directory."
