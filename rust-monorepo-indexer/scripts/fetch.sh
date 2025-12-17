#!/bin/bash
set -e

# This script clones top Rust monorepos for indexing.
# The list of repos can be extended.

REPOS=(
    "https://github.com/rust-lang/rust"
    "https://github.com/tokio-rs/tokio"
    "https://github.com/servo/servo"
    "https://github.com/hyperium/hyper"
    "https://github.com/clap-rs/clap"
)

TARGET_DIR="cloned_repos"
mkdir -p "$TARGET_DIR"

for repo in "${REPOS[@]}"; do
    repo_name=$(basename "$repo" .git)
    echo "Cloning $repo_name..."
    if [ -d "$TARGET_DIR/$repo_name" ]; then
        echo "  ...already exists. Skipping."
    else
        git clone --depth 1 "$repo" "$TARGET_DIR/$repo_name"
    fi
done

echo "Done."
