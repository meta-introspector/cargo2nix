#!/bin/bash

FILE="submodules/rust/compiler/rustc_expand/src/ast_fragments_split/mod.rs"

echo "Updating ${FILE} with new module declarations..."

# Read current content
CURRENT_CONTENT=$(<"$FILE")

# Find the line after which to insert new mods (after existing pub mod ast_fragments_invoke;)
INSERT_AFTER_LINE_NUM=$(grep -n 'pub mod ast_fragments_invoke;' "$FILE" | cut -d: -f1)

# New mod declarations
NEW_MODS=$(cat <<'EOF_NEW_MODS'
pub mod invocation_collector;
pub mod dummy_visitor_support;
pub mod expansion_config;
pub mod cfg_false_reporter_impl;
EOF_NEW_MODS
)

# Write new mods to a temporary file
printf "%s\n" "$NEW_MODS" > /tmp/new_mods.tmp

# Insert new mods after the specified line using sed 'r' command
sed -i.bak "${INSERT_AFTER_LINE_NUM}r /tmp/new_mods.tmp" "$FILE"

rm /tmp/new_mods.tmp

echo "Updated ${FILE}. Please review and rebuild."