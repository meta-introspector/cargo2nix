#!/bin/bash

BASE_DIR="/mnt/data1/nix/vendor/rust/cargo2nix" # Absolute path provided by the user

ORIG_FILE="$BASE_DIR/submodules/rust/compiler/rustc_expand/src/ast_fragments_types.rs"
LIB_FILE="$BASE_DIR/submodules/rust/compiler/rustc_expand/src/lib.rs"
SPLIT_DIR="$BASE_DIR/submodules/rust/compiler/rustc_expand/src/ast_fragments_split"
TEMPLATE_DIR="$SPLIT_DIR" # Corrected path: Templates are in the SPLIT_DIR


# Create the split directory if it doesn't exist
mkdir -p "$SPLIT_DIR"

echo "--- Starting file splitting refactoring ---"

# --- 1. Populate new Rust files from templates ---
echo "Populating new Rust files in $SPLIT_DIR/..."
cp "$TEMPLATE_DIR/ast_fragments_enums.rs.template" "$SPLIT_DIR/ast_fragments_enums.rs"
cp "$TEMPLATE_DIR/ast_fragments_macro.rs.template" "$SPLIT_DIR/ast_fragments_macro.rs"
cp "$TEMPLATE_DIR/ast_fragments_helpers.rs.template" "$SPLIT_DIR/ast_fragments_helpers.rs"
cp "$TEMPLATE_DIR/ast_fragments_node_impls.rs.template" "$SPLIT_DIR/ast_fragments_node_impls.rs"

echo "New Rust files created successfully."

# --- 2. Update original ast_fragments_types.rs ---
echo "Updating $ORIG_FILE..."
cp "$TEMPLATE_DIR/ast_fragments_types.rs.new.template" "$ORIG_FILE"

echo "$ORIG_FILE updated with module declarations and re-exports."

# --- 3. Update lib.rs ---
echo "Updating $LIB_FILE..."
# Use sed to replace 'mod ast_fragments_types;' with 'mod ast_fragments_split;'
# This is assuming the line is exactly 'mod ast_fragments_types;' and it's unique
sed -i "s|^mod ast_fragments_types;.*$|mod ast_fragments_split; // New module for the split fragments|" "$LIB_FILE"

echo "$LIB_FILE updated."

echo "--- File splitting refactoring complete ---"
echo "You may need to manually add specific 'use' statements in the new files if compilation errors occur."
