#!/bin/bash

# This script finds all Cargo.toml files, extracts GitHub repository URLs,
# and generates 'gh repo fork' commands for unique repositories.
#
# Usage: ./generate_gh_fork_commands.sh [--target-org <org>] [--target-branch <branch>]
#
# The generated commands will be printed to stdout. Review them carefully
# before executing.

TARGET_ORG="meta-introspector"
TARGET_BRANCH="feature/CRQ-016-nixify"

# Parse command-line arguments
while [[ "$#" -gt 0 ]]; do
    case "$1" in
        --target-org)
            TARGET_ORG="$2"
            shift
            ;;
        --target-branch)
            TARGET_BRANCH="$2"
            shift
            ;;
        *)
            echo "Unknown parameter passed: $1"
            exit 1
            ;;
    esac
    shift
done

echo "# Generated gh repo fork commands"
echo "# Review these commands carefully before executing them."
echo "# Target Organization: ${TARGET_ORG}"
echo "# Target Branch: ${TARGET_BRANCH}"
echo

# Find all Cargo.toml files, extract repository URLs, filter for GitHub,
# and get unique URLs.
# Exclude Cargo.toml files in 'target', 'tests', 'examples' directories.
find . -type f -name "Cargo.toml" \
    -not -path "./target/*" \
    -not -path "./submodules/*/target/*" \
    -not -path "./submodules/*/tests/*" \
    -not -path "./submodules/*/examples/*" \
    -not -path "./tests/*" \
    -not -path "./examples/*" \
    -print0 | xargs -0 grep -E 'repository\s*=' | \
    grep -E 'github\.com' | \
    sed -n 's/.*repository\s*=\s*\"\([^\"]*\)\".*/\1/p' | \
    sort -u | \
    while read -r repo_url; do
        # Extract owner and repo name from the URL
        if [[ "$repo_url" =~ github\.com/([^/]+)/([^/.]+)(\.git)? ]]; then
            OWNER="${BASH_REMATCH[1]}"
            REPO_NAME="${BASH_REMATCH[2]}"

            if [[ -n "$OWNER" && -n "$REPO_NAME" ]]; then
                echo "# Processing: ${repo_url}"
                echo "gh repo fork \"${OWNER}/${REPO_NAME}\" --org \"${TARGET_ORG}\" --remote --fork-name \"${REPO_NAME}\""
                echo "git clone \"https://github.com/${TARGET_ORG}/${REPO_NAME}.git\" \"submodules/${REPO_NAME}\""
                echo "cd \"submodules/${REPO_NAME}\""
                echo "git remote rename origin upstream"
                echo "git remote add origin \"https://github.com/${TARGET_ORG}/${REPO_NAME}.git\""
                echo "git checkout -b \"${TARGET_BRANCH}\" || git checkout \"${TARGET_BRANCH}\""
                echo "cd -"
                echo
            else
                echo "# WARNING: Could not extract owner or repository name from: ${repo_url}. Skipping."
            fi
        else
            echo "# WARNING: Repository URL does not match expected GitHub format: ${repo_url}. Skipping."
        fi
    done
