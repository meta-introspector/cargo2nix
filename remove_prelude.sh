#!/bin/bash

TARGET_DIR="submodules/rust/compiler/rustc_expand/src/ast_fragments_split/"

# Iterate over each .rs file in the target directory
find "$TARGET_DIR" -name "*.rs" | while read -r file; do
  # Check if the prelude exists in the file
  if grep -q "// BEGIN_PRELUDE" "$file"; then
    echo "Removing prelude from $file"
    # Remove lines between BEGIN_PRELUDE and END_PRELUDE markers (inclusive)
    sed -i '/\/\/ BEGIN_PRELUDE/,/\/\/ END_PRELUDE/d' "$file"
  else
    echo "Prelude not found in $file, skipping."
  fi
done
