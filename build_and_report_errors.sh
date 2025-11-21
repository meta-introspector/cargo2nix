#!/usr/bin/env bash

LOG_FILE="build.log"
NUM_ERRORS=20 # Default number of errors to display
CONTEXT_LINES=5 # Number of context lines to show after an error (approximate for sed)

echo "Running cargo build and logging output to $LOG_FILE..."
export PATH="$HOME/.cargo/bin:$PATH"
#cargo build --verbose > "$LOG_FILE" 2>&1
cargo build  > "$LOG_FILE" 2>&1

if [ $? -eq 0 ]; then
    echo "Build successful!"
else
    echo "Build failed. Displaying first $NUM_ERRORS errors with context (excluding warnings):"
    # This sed command prints blocks starting with "error:" and ending before the next "error:" or "warning:".
    # It also filters out lines that are just warnings within these blocks.
    # The `head` command limits the total number of lines, which is an approximation for limiting error blocks.
    grep -E -A "$CONTEXT_LINES" "error[:\[]" "$LOG_FILE" | head -n $((NUM_ERRORS * (CONTEXT_LINES + 1)))

    echo "Full log available in $LOG_FILE"
fi
