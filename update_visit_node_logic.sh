#!/bin/bash

FILE="submodules/rust/compiler/rustc_expand/src/ast_fragments_split/invocation_collector_visit_node.rs"

echo "Applying logic changes to $FILE"

OLD_BLOCK_START_PATTERN='                            if self.expand_cfg_true(node, attr, pos).as_bool() {'
OLD_BLOCK_END_PATTERN='                            return;' # End of the original block

NEW_BLOCK=$(cat <<'EOF_NEW'
                            if self.expand_cfg_true(node, attr, pos).as_bool() {
                                continue;
                            }

                            self.report_cfg_false(node, span, pos);
                            return;
EOF_NEW
)

# Get line numbers for the block to be replaced
START_LINE=$(grep -n "$OLD_BLOCK_START_PATTERN" "$FILE" | head -1 | cut -d: -f1)
END_LINE=$(grep -n "$OLD_BLOCK_END_PATTERN" "$FILE" | tail -1 | cut -d: -f1)

# Make sure START_LINE and END_LINE are found and valid
if [ -z "$START_LINE" ] || [ -z "$END_LINE" ]; then
    echo "Error: Could not find start or end line for the block."
    exit 1
fi

# Write the new content to a temporary file
printf "%s\n" "$NEW_BLOCK" > /tmp/replacement_block_visit_node.tmp

# Delete the old block
sed -i.bak "${START_LINE},${END_LINE}d" "$FILE"

# Insert the new block at the start line
sed -i "${START_LINE}r /tmp/replacement_block_visit_node.tmp" "$FILE"

rm /tmp/replacement_block_visit_node.tmp


echo "Changes applied. Please review the file and rebuild."

