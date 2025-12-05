#!/usr/bin/env bash
# This script is meant to be run via `git submodule foreach` or directly.
# It pulls the latest changes and rebases the current submodule.

echo "--- Rebasing submodule: $(pwd) ---"

# Pull latest changes and rebase
echo "Pulling latest changes with rebase..."
git pull --rebase origin $(git rev-parse --abbrev-ref HEAD)

echo "--- Finished rebasing submodule: $(pwd) ---"
