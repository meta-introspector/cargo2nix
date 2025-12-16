#!/bin/bash

# This script replaces 'pub(crate)' with 'pub' in all files listed in todo.txt.

if [ ! -f "todo.txt" ]; then
    echo "Error: todo.txt not found. Please run 'grep -r -E \"pub\\(crate\\)\" submodules/rust/compiler/ > todo.txt' first."
    exit 1
fi

while IFS= read -r line; do
    # Extract the file path, which is the first field in the grep output,
    # before the first colon.
    filepath=$(echo "$line" | cut -d':' -f1)

    if [ -f "$filepath" ]; then
        echo "Processing $filepath..."
        # Use sed to replace 'pub(crate)' with 'pub' in place.
        # -i for in-place editing.
        # 's/pattern/replacement/g' for global replacement.
        # Parentheses in 'pub(crate)' need to be escaped for sed.
        sed -i 's/pub(crate)/pub/g' "$filepath"
    else
        echo "Warning: File not found: $filepath. Skipping."
    fi
done < todo.txt

echo "Replacement complete."
