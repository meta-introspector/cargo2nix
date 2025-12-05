#!/bin/bash

# Ensure we are in the main repository root
# (This script is intended to be run from the main repository root)
if ! git rev-parse --is-inside-work-tree > /dev/null 2>&1 || [[ "$(basename $(pwd))" != "cargo2nix" ]]; then
    echo "Error: This script must be run from the main 'cargo2nix' repository root."
    exit 1
fi

echo "--- Checking out feature/CRQ-016-nixify in all submodules ---"

git submodule status | while read -r line ; do
    sha=$(echo "$line" | awk '{print $1}')
    path=$(echo "$line" | awk '{print $2}')
    description=$(echo "$line" | cut -d'(' -f2- | cut -d')' -f1)

    echo "Processing submodule: $path"
    
    # Check if the submodule is already on the target branch
    # This is a heuristic based on the description from git submodule status
    if [[ "$description" == "heads/feature/CRQ-016-nixify" ]]; then
        echo "  Already on feature/CRQ-016-nixify. Skipping. ${path}"
        continue
    fi

    # Go into the submodule directory
    if [ ! -d "$path" ]; then
        echo "Error: Submodule directory $path not found. Skipping."
        continue
    fi
    (
        cd "$path" || exit 1
        git status
	git reflog
        # Check if the branch exists
        if git show-ref --verify --quiet refs/heads/feature/CRQ-016-nixify; then
            echo "  Branch 'feature/CRQ-016-nixify' exists. Checking out."
            git checkout feature/CRQ-016-nixify
            if [ $? -ne 0 ]; then
                echo "Error: Failed to checkout branch in $path."
            fi
        else
            echo "  Branch 'feature/CRQ-016-nixify' does not exist. Creating from HEAD."
            git checkout -b feature/CRQ-016-nixify
            if [ $? -ne 0 ]; then
                echo "Error: Failed to create and checkout branch in $path."
            fi
        fi
    )
    if [ $? -ne 0 ]; then
        echo "Error: Failed to process submodule $path. Check its output above."
    fi
    echo "done"
 
done

echo "--- Finished processing all submodules ---"
