#!/usr/bin/env bash

# This script finds all Cargo.toml files, extracts GitHub repository URLs,
# and outputs a JSON array of repository information.
#
# Usage: ./generate_repolist.sh [--target-org <org>] [--target-branch <branch>]
#
# The generated JSON will be printed to stdout.

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

echo "["

FIRST_ENTRY=true

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
        # Clean up repo_url to remove /tree/branch/path segments
        if [[ "$repo_url" =~ ^(https?://github\.com/[^/]+/[^/.]+)(/tree/[^/]+/.+)?(\.git)?$ ]]; then
            repo_url_cleaned="${BASH_REMATCH[1]}${BASH_REMATCH[3]}"
        else
            repo_url_cleaned="$repo_url"
        fi

        # Extract owner and repo name from the cleaned URL
        if [[ "$repo_url_cleaned" =~ github\.com/([^/]+)/([^/.]+)(\.git)? ]]; then
            OWNER="${BASH_REMATCH[1]}"
            REPO_NAME="${BASH_REMATCH[2]}"

            if [[ -n "$OWNER" && -n "$REPO_NAME" ]]; then
                if [ "$FIRST_ENTRY" = false ]; then
                    echo ","
                fi
                FIRST_ENTRY=false
                echo "  {"
                echo "    \"repo_url\": \"$repo_url_cleaned\","
                echo "    \"owner\": \"$OWNER\","
                echo "    \"repo_name\": \"$REPO_NAME\","
                echo "    \"target_org\": \"$TARGET_ORG\","
                echo "    \"target_branch\": \"$TARGET_BRANCH\""
                echo "  }"
            else
                echo "# WARNING: Could not extract owner or repository name from: ${repo_url_cleaned}. Skipping." >&2
            fi
        else
            echo "# WARNING: Repository URL does not match expected GitHub format: ${repo_url_cleaned}. Skipping." >&2
        fi
    done

echo "]"
