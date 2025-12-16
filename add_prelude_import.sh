#!/bin/bash

IMPORT_STATEMENT="use rustc_expand_base_lib::prelude::*;"
TARGET_DIR="submodules/rust/compiler/rustc_expand/src/ast_fragments_split/"

# Iterate over each .rs file in the target directory
find "$TARGET_DIR" -name "*.rs" | while read -r file; do
  # Check if the import statement already exists in the file
  if ! grep -qF "$IMPORT_STATEMENT" "$file"; then
    echo "Adding '$IMPORT_STATEMENT' to $file"
    # Prepend the import statement to the file
    # Use 'echo -e' to handle the newline after the statement
    (echo "$IMPORT_STATEMENT"; cat "$file") > "$file.tmp" && mv "$file.tmp" "$file"
  else
    echo "Import statement already exists in $file, skipping."
  fi
done
