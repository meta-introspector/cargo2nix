#!/bin/bash

FILE="submodules/rust/compiler/rustc_expand/src/ast_fragments_split/ast_fragments_helpers.rs"

echo "Applying sed changes to $FILE"

# OLD block
OLD_BLOCK_START_PATTERN='                            if self.expand_cfg_true(node, attr, pos).as_bool() {'
OLD_BLOCK_END_PATTERN='                            return;'

# NEW block
NEW_BLOCK=$(cat <<'EOF_NEW'
                            if self.expand_cfg_true(node, attr, pos).as_bool() {
                                continue;
                            }

                            // Handle cfg_false logic directly within InvocationCollector::visit_node
                            self.cx.sess.psess.buffer_lint(
                                self.cfg().get_unused_attribute_lint(true),
                                span,
                                self.cx.current_expansion.lint_node_id,
                                crate::errors::CfgFalseRemoved { span },
                            );
                            node.visit_attrs(|attrs| {
                                attrs.remove(pos);
                            });
                            return;
EOF_NEW
)

# Get line numbers for the block to be replaced
START_LINE=$(grep -n "$OLD_BLOCK_START_PATTERN" "$FILE" | cut -d: -f1)
END_LINE=$(grep -n "$OLD_BLOCK_END_PATTERN" "$FILE" | cut -d: -f1)

# Make sure START_LINE and END_LINE are found and valid
if [ -z "$START_LINE" ] || [ -z "$END_LINE" ]; then
    echo "Error: Could not find start or end line for the block."
    exit 1
fi

# Use sed to replace the block
sed -i.bak "${START_LINE},${END_LINE}c\\
$(echo "$NEW_BLOCK" | sed -e 's/[]\/\$%*.^[]/\\&/g')
" "$FILE"

echo "Changes applied. Please review the file and rebuild."

