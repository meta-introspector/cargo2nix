#!/usr/bin/env bash
# This script recursively commits and pushes changes in Git submodules,
# processing deeper submodules first.

# Ensure we are in the root of the main repository
MAIN_REPO_ROOT=$(git rev-parse --show-toplevel)
if [ -z "$MAIN_REPO_ROOT" ]; then
    echo "Error: Not in a Git repository."
    exit 1
fi
cd "$MAIN_REPO_ROOT" || exit 1

echo "Collecting and sorting submodule paths by depth..."

# Get all submodule paths, extract the path, and sort by depth (descending)
# This ensures deeper submodules are processed first
SUBMODULE_PATHS=$(git submodule status --recursive | awk '{print $2}' | sort -r -t '/' -k 1,1)

# Function to process a single submodule
process_submodule() {
    local submodule_path="$1"
    echo "--- Processing submodule: $submodule_path ---"

    # Save current directory
    local current_dir=$(pwd)

    cd "$submodule_path" || { echo "Error: Could not enter $submodule_path"; return 1; }

    # Add all changes
    git add .

    # Check if there are any staged changes to commit
    if ! git diff --cached --quiet; then
        echo "Committing staged changes in $submodule_path..."
        git commit -m "feat: CRQ-016 submodule updates" -n || { echo "Warning: Failed to commit in $submodule_path"; cd "$current_dir"; return 1; }
        echo "Commit successful in $submodule_path."

        # Get remote URL
        local remote_url=$(git remote get-url origin 2>/dev/null)
        if [[ "$remote_url" == *"github.com/meta-introspector/"* ]]; then
            echo "Remote is from github.com/meta-introspector/. Attempting push for $submodule_path."
            if git push origin HEAD:$(git rev-parse --abbrev-ref HEAD) >> "$MAIN_REPO_ROOT/submodules_recursive_log.txt" 2>&1; then
                echo "Push successful for $submodule_path."
            else
                echo "Warning: Failed push for $submodule_path. Check $MAIN_REPO_ROOT/submodules_recursive_log.txt for details."
            fi
        else
            echo "Remote is NOT from github.com/meta-introspector/. Skipping push for $submodule_path."
        fi
    else
        echo "No staged changes to commit in $submodule_path."
    fi

    # Go back to the original directory
    cd "$current_dir" || exit 1
    echo "--- Finished processing submodule: $submodule_path ---"
}

# Iterate and process each submodule
for path in $SUBMODULE_PATHS; do
    process_submodule "$path"
done

echo "All submodules processed."
