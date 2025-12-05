#!/bin/bash

# Define the users to search for and the main repository directory
AUTHORS_TO_SEARCH=(
"mike dupont <mike.dupont@introspector.local>"
"mike <h@solfunmeme.com>"
)
MAIN_REPO_DIR="/mnt/data1/nix/vendor/rust/cargo2nix"
SINCE_DATE="2025-12-02 14:16:18 -0500" # Approximately 24 hours before last successful run output

# Function to print git diffs for a given path and author
# This function is now private to this script, as it will be called by print_crate_info
_print_git_diff() {
    local target_path="$1"
    local author_filter="$2"

    echo "--- Git Diff for $target_path by $author_filter (Since: $SINCE_DATE) ---"
    (
        cd "$MAIN_REPO_DIR" || { echo "ERROR: Could not change directory to $MAIN_REPO_DIR"; return 1; }
        git log -p --since="$SINCE_DATE" --author="$author_filter" -- "$target_path" || echo "No diffs found for $target_path by $author_filter in the last 24 hours."
    )
    echo ""
}

# Main function to analyze a specific target path (crate or file)
analyze_target_path() {
    local target_path="$1"

    echo "--- Analyzing Target: $target_path ---"

    # Print git diffs for the target path by specified authors
    for author in "${AUTHORS_TO_SEARCH[@]}"; do
        _print_git_diff "$target_path" "$author"
    done
}

# Main script execution
if [ -n "$1" ]; then
    # If a target path is provided as an argument, analyze only that path
    analyze_target_path "$1"
else
    # Otherwise, run the default behavior (which would be to iterate problematic crates)
    # For now, if no argument is given, it will just exit or print usage.
    echo "Usage: $0 <target_crate_or_file_path>"
    exit 1
fi