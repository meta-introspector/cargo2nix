#!/usr/bin/env bash

# Define local modules
local_modules=(
    "cargo-edit-lib"
    "cargo-repo-sync-lib"
    "cargo-submodule-tool-lib"
    "cargo-toml-editor-lib"
    "git-wrapper-lib"
    "nix-generator-lib"
    "real-regex-adapter-lib"
    "real-toml-adapter-lib"
    "real-walkdir-adapter-lib"
    "syn-adapter-lib"
    "tool-traits-lib"
    "cargo2nix" # This is in ../../crates/cargo2nix
)

echo "Fixing local dependencies in Cargo.toml files..."

# Iterate over each local module
for module in "${local_modules[@]}"; do
    # Replace version-based dependency with workspace-based dependency
    find . -name "Cargo.toml" -print0 | xargs -0 sed -i "s|${module} = { version = \"\*\", optional = true }|${module} = { workspace = true, optional = true }|g"
    
    # Replace incorrect underscored feature dependency with correct hyphenated name
    find . -name "Cargo.toml" -print0 | xargs -0 sed -i "s|dep:${module//-/_}|dep:${module}|g"
done

echo "Local dependency fixes applied."
