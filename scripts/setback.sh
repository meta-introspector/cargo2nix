#!/usr/bin/env bash

# This script is intended to be run by 'git submodule foreach'.
# It collects the reflog for the current submodule and reports its status
# relative to the 'feature/CRQ-016-nixify' branch.

# Define the target branch for CRQ-016
CRQ_BRANCH="feature/CRQ-016-nixify"

echo "----------------------------------------------------"
# $path variable is provided by git submodule foreach
echo "Processing submodule: $path (current directory: $PWD)"

# Check if the current directory is a valid Git repository and initialized
# This check is more robust for submodules than checking a fixed path for existence.
if ! git rev-parse --is-inside-work-tree &>/dev/null; then
    echo "Current directory is not a Git work tree. Skipping."
    exit 0
fi

echo "Reflog for $path:"
git reflog --date=relative
echo ""

CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)

if git rev-parse --verify "$CRQ_BRANCH" &>/dev/null; then
    echo "Branch '$CRQ_BRANCH' exists in $path."
    
    if [ "$CURRENT_BRANCH" == "$CRQ_BRANCH" ]; then
        echo "Current branch is '$CRQ_BRANCH'."
    else
        echo "Current branch is '$CURRENT_BRANCH', NOT '$CRQ_BRANCH'."
    fi

    # Compare HEADs to see if it has moved away
    CRQ_HEAD=$(git rev-parse "$CRQ_BRANCH")
    CURRENT_HEAD=$(git rev-parse HEAD)

    if [ "$CRQ_HEAD" == "$CURRENT_HEAD" ]; then
        echo "Current HEAD is at the same commit as '$CRQ_BRANCH'."
    else
        echo "Current HEAD ($CURRENT_HEAD) has moved away from '$CRQ_BRANCH' ($CRQ_HEAD)."
        # Check for divergence
        if git merge-base --is-ancestor "$CRQ_HEAD" "$CURRENT_HEAD"; then
            echo "Current HEAD is a descendant of '$CRQ_BRANCH'."
        elif git merge-base --is-ancestor "$CURRENT_HEAD" "$CRQ_HEAD"; then
            echo "'$CRQ_BRANCH' is a descendant of Current HEAD."
        else
            echo "Current HEAD and '$CRQ_BRANCH' have diverged."
        fi
    fi
else
    echo "Branch '$CRQ_BRANCH' not found in $path."
fi
echo ""