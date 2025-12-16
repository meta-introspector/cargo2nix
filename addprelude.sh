#!/bin/bash

# Define the prelude content
PRELUDE="// BEGIN_PRELUDE
use rustc_ast::{self as ast, NodeId};
use rustc_span::{Span, Symbol};
use rustc_feature::Features;
use rustc_hir::limit::Limit;
use smallvec::SmallVec;
// END_PRELUDE
"

TARGET_DIR="submodules/rust/compiler/rustc_expand/src/ast_fragments_split/"

# Iterate over each .rs file in the target directory
find "$TARGET_DIR" -name "*.rs" | while read -r file; do
  # Check if the prelude already exists in the file
  if ! grep -q "// BEGIN_PRELUDE" "$file"; then
    echo "Adding prelude to $file"
    # Prepend the prelude to the file
    echo "$PRELUDE$(cat "$file")" > "$file"
  else
    echo "Prelude already exists in $file, skipping."
  fi
done
