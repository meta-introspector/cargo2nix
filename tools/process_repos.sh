#!/usr/bin/env bash

# Exit immediately if a command exits with a non-zero status.
set -e

# Define the target organization and branch
TARGET_ORG="meta-introspector"
TARGET_BRANCH="feature/CRQ-016-nixify"
REPOS_FILE="github_repos.txt"
SUBMODULES_DIR="submodules"

# Parse arguments
DRY_RUN=false
while [[ "$#" -gt 0 ]]; do
    case "$1" in
        --dry-run) DRY_RUN=true ;;
        *) echo "Unknown parameter passed: $1"; exit 1 ;;
    esac
    shift
done

if [ "$DRY_RUN" = true ]; then
    echo "--- DRY RUN MODE ACTIVE ---"
fi

# Create the submodules directory if it doesn't exist
if [ "$DRY_RUN" = true ]; then
    echo "[DRY RUN] Would create directory: $SUBMODULES_DIR"
else
    mkdir -p "$SUBMODULES_DIR"
fi

# Read each repository URL from the file
while IFS= read -r REPO_URL; do
    echo "--- Processing $REPO_URL ---"

    # Extract owner and repository name
    # Example: https://github.com/owner/repo.git -> owner, repo
    # Handle cases like https://github.com/owner/repo/path/to/subproject
    # We want 'owner' and 'repo' (the top-level repository name)
    # First, remove "https://github.com/" prefix
    GITHUB_PATH=$(echo "$REPO_URL" | sed 's/https:\/\/github.com\///')
    # Then, split by / and take the first two parts
    OWNER=$(echo "$GITHUB_PATH" | cut -d'/' -f1)
    REPO_NAME=$(echo "$GITHUB_PATH" | cut -d'/' -f2 | sed 's/\.git$//') # Remove .git suffix if present

    if [ -z "$OWNER" ] || [ -z "$REPO_NAME" ]; then
        echo "Could not extract owner or repository name from $REPO_URL. Skipping."
        continue
    fi

    SUBMODULE_PATH="$SUBMODULES_DIR/$REPO_NAME"

    # Construct the base clone URL
    CLONE_URL="https://github.com/$OWNER/$REPO_NAME.git"

    # Check if the submodule directory already exists
    if [ -d "$SUBMODULE_PATH" ]; then
        echo "Submodule directory $SUBMODULE_PATH already exists. Skipping clone."
    else
        if [ "$DRY_RUN" = true ]; then
            echo "[DRY RUN] Would clone $CLONE_URL into $SUBMODULE_PATH..."
        else
            echo "Cloning $CLONE_URL into $SUBMODULE_PATH..."
            git clone "$CLONE_URL" "$SUBMODULE_PATH"
        fi
    fi

    # Navigate into the submodule directory
    if [ "$DRY_RUN" = true ]; then
        echo "[DRY RUN] Would navigate into $SUBMODULE_PATH"
        # In dry-run, we can't actually pushd, so we simulate the context
        # For subsequent commands, we'll just print them as if we were in that dir.
    else
        pushd "$SUBMODULE_PATH" > /dev/null
    fi

    # Rename 'origin' to 'upstream' if it exists and 'upstream' does not already exist
    if git remote get-url origin &> /dev/null; then
        if ! git remote get-url upstream &> /dev/null; then
            if [ "$DRY_RUN" = true ]; then
                echo "[DRY RUN] Would rename remote 'origin' to 'upstream' in $REPO_NAME..."
            else
                echo "Renaming remote 'origin' to 'upstream'..."
                git remote rename origin upstream
            fi
        else
            echo "Remote 'upstream' already exists. Skipping renaming 'origin'."
        fi
    fi

    # Add new 'origin' pointing to meta-introspector fork
    NEW_ORIGIN_URL="https://github.com/$TARGET_ORG/$REPO_NAME.git"
    if ! git remote get-url origin &> /dev/null; then
        if [ "$DRY_RUN" = true ]; then
            echo "[DRY RUN] Would add new remote 'origin' as $NEW_ORIGIN_URL in $REPO_NAME..."
        else
            echo "Adding new remote 'origin' as $NEW_ORIGIN_URL..."
            git remote add origin "$NEW_ORIGIN_URL"
        fi
    else
        echo "Remote 'origin' already exists. Skipping adding new origin."
    fi

    # Fork using gh repo fork
    echo "Attempting to fork $OWNER/$REPO_NAME to $TARGET_ORG using 'gh' CLI..."
    if [ "$DRY_RUN" = true ]; then
        echo "[DRY RUN] Would execute: gh repo fork $OWNER/$REPO_NAME --org $TARGET_ORG --remote --fork-name $REPO_NAME"
    else
        gh repo fork "$OWNER/$REPO_NAME" --org "$TARGET_ORG" --remote --fork-name "$REPO_NAME" || {
            echo "Warning: Failed to fork $OWNER/$REPO_NAME. It might already be forked or 'gh' CLI is not configured. Continuing..."
        }
    fi

    # Checkout the target branch
    echo "Checking out branch $TARGET_BRANCH..."
    if [ "$DRY_RUN" = true ]; then
        echo "[DRY RUN] Would checkout branch $TARGET_BRANCH in $REPO_NAME..."
    else
        if git checkout "$TARGET_BRANCH" &> /dev/null; then
            echo "Checked out existing branch $TARGET_BRANCH."
        else
            echo "Branch $TARGET_BRANCH does not exist, creating and checking out..."
            git checkout -b "$TARGET_BRANCH"
        fi
    fi

    # Navigate back to the parent directory
    if [ "$DRY_RUN" = true ]; then
        echo "[DRY RUN] Would navigate back from $SUBMODULE_PATH"
    else
        popd > /dev/null
    fi

    echo "--- Finished processing $REPO_URL ---"
    echo ""

done < "$REPOS_FILE"

echo "All repositories processed."