#!/usr/bin/env bash

LOG_FILE="build.log"
NUM_ERRORS=20 # Default number of errors to display

# --- Load .envrc if it exists ---
if [ -f ".envrc" ]; then
    echo "Loading .envrc..."
    direnv allow . > /dev/null 2>&1
    eval "$(direnv export bash)"
    echo ".envrc loaded."
else
    echo "No .envrc found. Proceeding without loading environment variables from .envrc."
fi

# --- Git branch check ---
CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
REQUIRED_BRANCH="feature/CRQ-016-nixify-workflow"
if [ "$CURRENT_BRANCH" != "$REQUIRED_BRANCH" ]; then
    echo "WARNING: You are not on the '$REQUIRED_BRANCH' branch. Current branch is '$CURRENT_BRANCH'."
    echo "Please consider switching to the correct branch: git checkout $REQUIRED_BRANCH"
    # Decide whether to exit or continue. For now, we'll continue with a warning.
    # exit 1
fi

echo "Running cargo build with Nix environment and logging output to $LOG_FILE..."
# Ensure sccache is configured within the Nix environment
mkdir -p "/tmp/sc"
export SCCACHE_DIR="/tmp/sc"
export RUSTC_BOOTSTRAP=1 # Needed for some compiler features, ensure it's set.

# Wrap cargo build in nix develop command
# Capture both stdout and stderr of the inner command to the LOG_FILE
nix develop --command bash -c "RUSTC_BOOTSTRAP=1 cargo build --message-format=json --warnings=none 2>&1" > "$LOG_FILE" 2>&1

# Check the exit code of the nix develop command
if [ $? -eq 0 ]; then
    echo "Build successful!"
else
    echo "Build failed. Displaying first $NUM_ERRORS compiler errors:"
    # Filter JSON output for errors and format them nicely
    # Using 'grep -v "level\":\"warning\"' to explicitly exclude warnings.
    # The 'jq -r' command extracts and formats the rendered message.
    grep '"reason":"compiler-message"' "$LOG_FILE" | grep '"level":"error"' | head -n "$NUM_ERRORS" | \
    jq -r 'select(.reason == "compiler-message" and .message.level == "error") | .message.rendered' | \
    sed 's/\\n/\n/g; s/\\t/\t/g; s/\\"/"/g; s/\\r//g; s/\\//g'

    echo "Full log available in $LOG_FILE"
    echo "For advanced debugging, you can use: nix develop --command bash -c \"RUSTC_BOOTSTRAP=1 cargo build --message-format=json --warnings=none\""
    echo "To inspect a compiler error interactively, consider setting RUST_BACKTRACE=1 or using a debugger."
fi
