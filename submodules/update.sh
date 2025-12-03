#!/usr/bin/env bash
# This script is meant to be run via `git submodule foreach` or directly.
# It updates the current submodule's Git state, performs optional dry-runs,
# and pushes changes if the remote URL matches a specific pattern.

#git submodule foreach ~/nix/vendor/rust/cargo2nix/submodules/update.sh
#pwd
#set -x # Uncomment for debugging

DRY_RUN=false
CHANGES_REPORT="" # Initialize an empty string to capture changes

# Parse arguments
for arg in "$@"; do
    case $arg in
        --dry-run)
        DRY_RUN=true
        shift # Remove --dry-run from processing
        ;;
        *)
        # Handle other arguments if any, or error out
        ;;
    esac
done

# Suppress verbose output if in dry run mode, unless it's the diff itself
if [ "$DRY_RUN" = false ]; then
    echo "--- Processing submodule: $(pwd) ---"
fi

# Stage relevant files for potential commit or diff
if [ -f Cargo.lock ]; then
    git add Cargo.lock
fi
if [ -f flake.nix ]; then
    git add flake.nix
fi
if [ -f flake.lock ]; then
    git add flake.lock
fi
if [ -f Cargo.toml ]; then
    git add Cargo.toml
else
    : # No-op if Cargo.toml not found.
fi

# Dry run logic
if [ "$DRY_RUN" = true ]; then
    echo "--- DRY RUN MODE for $(pwd) ---" # Re-introducing this header with path
    # Capture staged changes
    CHANGES_REPORT=$(git diff --cached)
    if [ -n "$CHANGES_REPORT" ]; then # Check if CHANGES_REPORT is not empty
        echo "The following changes would be committed:" # Re-introducing this header
        echo "$CHANGES_REPORT"
    else
        echo "No changes to commit in dry run." # Re-introducing this message
    fi
    echo "--- END DRY RUN MODE for $(pwd) ---" # Re-introducing this footer
    exit 0 # Exit after dry run report
fi

# --- Actual script logic (only executed if not dry run) ---

    # Stage all changes in the current submodule
    git add .

    # Pull latest changes and rebase
    echo "Pulling latest changes with rebase..."
    git pull --rebase origin $(git rev-parse --abbrev-ref HEAD)

    # Check for any staged changes to commit after adds and potential rebase
    if ! git diff --cached --quiet; then
        echo "Committing staged changes..."
        git commit -m 'feat: Add/update Nix flake for submodule (CRQ-016)' -n
    else
        echo "No staged changes to commit after pull."
    fi

    REMOTE_URL=$(git remote get-url origin)
    echo "The remote URL for this submodule is: $REMOTE_URL"

    if [[ "$REMOTE_URL" == *"github.com/meta-introspector/"* ]]; then
        echo "Remote is from github.com/meta-introspector/. Attempting push."
        if git push origin HEAD:$(git rev-parse --abbrev-ref HEAD) >> /mnt/data1/nix/vendor/rust/cargo2nix/submodules/log.txt; then        echo "Push successful for $REMOTE_URL"
    else
        echo "Failed push for $REMOTE_URL. Check /mnt/data1/nix/vendor/rust/cargo2nix/submodules/log.txt for details."
    fi
else
    echo "Remote is NOT from github.com/meta-introspector/. Skipping push."
fi

echo "--- Finished processing submodule: $(pwd) ---"
