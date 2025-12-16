#!/bin/bash

BASE_DIR="submodules/rust/compiler/rustc_expand/src/ast_fragments_split/"
MAIN_FILE="${BASE_DIR}ast_fragments_helpers.rs"

echo "Cleaning up backup files..."
rm -f "${BASE_DIR}"*.rs~
rm -f "${BASE_DIR}"*.rs.bak
echo "Backup files cleaned."

echo "Adding mod declarations to ${MAIN_FILE}"

# List of files to add mod declarations for (excluding the main file itself and backups/templates)
# Adjusted regex to only pick up the created helper files.
FILES_TO_MOD=$(ls "${BASE_DIR}" | grep -E '^ast_fragments_helpers_[a-zA-Z0-9_]+\.rs$' | sort)

# Read the current content of ast_fragments_helpers.rs
CURRENT_MAIN_FILE_CONTENT=$(<"$MAIN_FILE")

# Generate mod declarations
MOD_DECLS=""
for file in $FILES_TO_MOD; do
    # Extract module name from filename
    module_name=$(basename "$file" .rs)
    MOD_DECLS+="mod ${module_name};\n"
done

# Append mod declarations to the file.
# I will use write_file to replace the entire content of MAIN_FILE
# with CURRENT_MAIN_FILE_CONTENT plus MOD_DECLS.
printf "%s\n%s" "$CURRENT_MAIN_FILE_CONTENT" "$MOD_DECLS" > "$MAIN_FILE.tmp"
mv "$MAIN_FILE.tmp" "$MAIN_FILE"

echo "Mod declarations added to ${MAIN_FILE}. Please review and rebuild."
