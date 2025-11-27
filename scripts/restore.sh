#!/bin/bash

# This script is intended to be run by 'git submodule foreach'

SUBMODULE_PATH=$(pwd)
SUBMODULE_NAME=$(basename "$SUBMODULE_PATH")

echo "--- Processing submodule: $SUBMODULE_NAME ---"

echo "Resetting $SUBMODULE_NAME to HEAD and cleaning working directory..."
# Discard all local changes and reset to the HEAD of the current branch
if ! git reset --hard HEAD; then
    echo "Error: Failed to hard reset $SUBMODULE_NAME. Skipping."
    exit 1 # Exit this script
fi

# Remove all untracked files and directories, including ignored ones
if ! git clean -fdx; then
    echo "Error: Failed to clean untracked files in $SUBMODULE_NAME. Skipping."
    exit 1 # Exit this script
fi

echo "Successfully restored $SUBMODULE_NAME."