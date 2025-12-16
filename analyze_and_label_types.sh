#!/bin/bash

# This script analyzes Rust code for specific type usages, saves the findings to a file,
# and then processes these findings to replace the original type names with
# deterministic, unique labels for analytical purposes.
# It does NOT modify the original source files.
#
# Usage: ./analyze_and_label_types.sh
#
# Output:
#   - type_usage_raw.log: Raw grep output of type usages.
#   - type_usage_labeled.log: Processed output with type names replaced by labels.

# Define the target file where we performed the type aliasing
TARGET_FILE="submodules/rust/compiler/rustc_expand/src/expand.rs"

# Define patterns to search for
# Using word boundaries (\b) to match whole words.
# Added DRT to the patterns.
PATTERN_EXT_CTXT="\bExtCtxt\b"
PATTERN_INVOCATION_COLLECTOR="\bInvocationCollector\b"
PATTERN_DERIVE_RESOLUTION="\bDeriveResolution\b"
PATTERN_DRT="\bDRT\b" # Targeting the generic parameter DRT

# Define deterministic labels
LABEL_EXT_CTXT="_EXT_CTXT_ALIAS_"
LABEL_INVOCATION_COLLECTOR="_INVOCATION_COLLECTOR_ALIAS_"
LABEL_DERIVE_RESOLUTION="_DERIVE_RESOLUTION_ALIAS_"
LABEL_DRT="_DRT_GENERIC_PARAM_" # Label for DRT

RAW_OUTPUT_FILE="type_usage_raw.log"
LABELED_OUTPUT_FILE="type_usage_labeled.log"

echo "--- Step 1: Grepping for type usages in $TARGET_FILE ---"
# Use grep to find and report instances of the old string
# Check exit code directly with 'if mycmd;', not indirectly with $?.
if grep -nE "$PATTERN_EXT_CTXT|$PATTERN_INVOCATION_COLLECTOR|$PATTERN_DERIVE_RESOLUTION|$PATTERN_DRT" "$TARGET_FILE" > "$RAW_OUTPUT_FILE"; then
    echo "Raw type usage saved to $RAW_OUTPUT_FILE"
    echo ""
    echo "--- Step 2: Processing raw output with sed to apply labels ---"
    # Use sed to replace the found patterns with their deterministic labels
    # Use a temporary file for intermediate sed operations to avoid issues with in-place editing
    # and then move it to the final labeled output file.
    # IMPORTANT: The order of sed substitutions matters.
    # Replace the more specific patterns (like full type names) before generic ones (like DRT)
    # to avoid partial replacements.
    sed -E \
        "s/$PATTERN_INVOCATION_COLLECTOR/$LABEL_INVOCATION_COLLECTOR/g; \
         s/$PATTERN_DERIVE_RESOLUTION/$LABEL_DERIVE_RESOLUTION/g; \
         s/$PATTERN_EXT_CTXT/$LABEL_EXT_CTXT/g; \
         s/$PATTERN_DRT/$LABEL_DRT/g" \
        "$RAW_OUTPUT_FILE" > "$LABELED_OUTPUT_FILE"

    echo "Labeled type usage saved to $LABELED_OUTPUT_FILE"
    echo ""
    echo "--- Analysis Complete ---"
    echo "Raw type usage can be found in: $RAW_OUTPUT_FILE"
    echo "Labeled type usage can be found in: $LABELED_OUTPUT_FILE"
else
    echo "No relevant type usages found in $TARGET_FILE. No output files generated."
    rm -f "$RAW_OUTPUT_FILE" "$LABELED_OUTPUT_FILE"
fi
