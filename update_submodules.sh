#!/bin/bash

# Ensure we are in the main repository root
if ! git rev-parse --is-inside-work-tree > /dev/null 2>&1 || [[ "$(basename $(pwd))" != "cargo2nix" ]]; then
    echo "Error: This script must be run from the main 'cargo2nix' repository root."
    exit 1
fi

echo "--- Updating all submodules with upstream changes ---"

git submodule status | while read -r line ; do
    path=$(echo "$line" | awk '{print $2}')

    echo "Processing submodule: $path"
    
    # Go into the submodule directory
    if [ ! -d "$path" ]; then
        echo "Error: Submodule directory $path not found. Skipping."
        continue
    fi
    (
        cd "$path" || exit 1
        
        current_branch=$(git rev-parse --abbrev-ref HEAD)
        if [ "$current_branch" != "feature/CRQ-016-nixify" ]; then
            echo "  Warning: Submodule $path is not on feature/CRQ-016-nixify. Skipping pull/merge."
            echo "  Please ensure all submodules are on feature/CRQ-016-nixify before running this script."
            exit 1 # Exit the subshell, main script continues
        fi

        echo "  Fetching upstream changes..."
        git fetch origin

        # Try merging from 'origin/main' first, then 'origin/master'
        merge_successful=false
        for upstream_branch in "main" "master" "trunk"; do
            if git show-ref --verify --quiet "refs/remotes/origin/$upstream_branch"; then
                echo "  Attempting to merge origin/$upstream_branch into $current_branch..."
                if git merge "origin/$upstream_branch"; then
                    echo "  Successfully merged origin/$upstream_branch."
                    merge_successful=true
                    break
                else
                    echo "  Merge from origin/$upstream_branch failed for $path. Please resolve conflicts manually."
                    echo "  To resolve: cd $path && git status && git mergetool && git add . && git commit"
                    exit 1 # Exit the subshell, main script continues, but this submodule is left with conflicts
                fi
            fi
        done
        
        if [ "$merge_successful" = false ]; then
            echo "  Could not find a suitable upstream branch (main or master) to merge from for $path. Skipping."
            exit 1
        fi
    )
    if [ $? -ne 0 ]; then
        echo "Error: Failed to process submodule $path. Check its output above."
        # This error is from the subshell exiting with non-zero, indicating a merge conflict or other issue.
        # We need to stop the main script here to allow user to resolve.
        echo "Please resolve the issue in submodule $path and re-run the script."
        exit 1
    fi
    echo ""
done

echo "--- Finished updating all submodules ---"
