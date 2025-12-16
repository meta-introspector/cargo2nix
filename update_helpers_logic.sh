#!/bin/bash

FILE="submodules/rust/compiler/rustc_expand/src/ast_fragments_split/ast_fragments_helpers.rs"

echo "Applying logic changes to $FILE"

# --- Modify InvocationCollector::visit_node ---
OLD_VISIT_NODE_BLOCK_START_PATTERN='                            if self.expand_cfg_true(node, attr, pos).as_bool() {'
OLD_VISIT_NODE_BLOCK_END_PATTERN='                            return;' # End of the original block

NEW_VISIT_NODE_BLOCK=$(cat <<'EOF_NEW_VISIT_NODE'
                            if self.expand_cfg_true(node, attr, pos).as_bool() {
                                continue;
                            }

                            self.report_cfg_false(node, span, pos);
                            return;
EOF_NEW_VISIT_NODE
)

# Get line numbers for the block to be replaced
START_LINE=$(grep -n "$OLD_VISIT_NODE_BLOCK_START_PATTERN" "$FILE" | head -1 | cut -d: -f1)
END_LINE=$(grep -n "$OLD_VISIT_NODE_BLOCK_END_PATTERN" "$FILE" | tail -1 | cut -d: -f1)

# Make sure START_LINE and END_LINE are found and valid
if [ -z "$START_LINE" ] || [ -z "$END_LINE" ]; then
    echo "Error: Could not find start or end line for the visit_node block."
    exit 1
fi

# Write the new content to a temporary file
printf "%s\n" "$NEW_VISIT_NODE_BLOCK" > /tmp/replacement_block_visit_node.tmp

# Delete the old block
sed -i.bak "${START_LINE},${END_LINE}d" "$FILE"

# Insert the new block at the start line
sed -i "${START_LINE}r /tmp/replacement_block_visit_node.tmp" "$FILE"

rm /tmp/replacement_block_visit_node.tmp


# --- Implement CfgFalseReporter for InvocationCollector ---
# Find the end of the `impl<'a, 'b> MutVisitor for InvocationCollector<'a, 'b>` block
# The previous sed call might change line numbers, so recalculate.
# The target is the '}' of the impl MutVisitor block.
# Robustly find the line number of the closing brace for the MutVisitor impl.
# We'll search for the impl line, then find the matching closing brace.

# Get the line number of the `impl MutVisitor for InvocationCollector` declaration
IMPL_DECL_LINE=$(grep -n 'impl<'\''a, '\''b> MutVisitor for InvocationCollector<'\''a, '\''b'> {' "$FILE" | head -1 | cut -d: -f1)

# Find the matching closing brace '}' for that impl block
# This requires a more complex awk or a manual count, but for typical impl blocks, 
# a simple grep for '}' on or after IMPL_DECL_LINE can work.
# Let's assume there are no nested impls or complex structures that would confuse this.

# To find the END line for the MutVisitor impl block.
# This assumes the impl block ends with a '}' on a line by itself.
MUT_VISITOR_IMPL_END_LINE=$(grep -n '}' "$FILE" | awk -v start_line="${IMPL_DECL_LINE}" '
    BEGIN {brace_count = 0; in_block = 0}
    $1 ~ /^([0-9]+):.*impl<\x27\x27a, \x27\x27b> MutVisitor for InvocationCollector<\x27\x27a, \x27\x27b'> {/ {
        if ($1 == start_line) {
            in_block = 1;
        }
    }
    in_block == 1 {
        brace_count += gsub(/{/, "{", $0);
        brace_count -= gsub(/}/, "}", $0);
        if (brace_count == 0 && in_block == 1) {
            print substr($1, 1, index($1, ":")-1);
            exit;
        }
    }' | cut -d: -f1)

if [ -z "$MUT_VISITOR_IMPL_END_LINE" ]; then
    echo "Error: Could not find end of MutVisitor impl block."
    exit 1
fi

NEW_IMPL_BLOCK=$(cat <<'EOF_IMPL_CFG_FALSE'

impl<'a, 'b> CfgFalseReporter for InvocationCollector<'a, 'b> {
    fn report_cfg_false<N: HasAttrs + HasNodeId>(&mut self, node: &mut N, attr_span: Span, attr_pos: usize) {
        self.cx.sess.psess.buffer_lint(
            self.cfg().get_unused_attribute_lint(true),
            attr_span,
            self.cx.current_expansion.lint_node_id,
            crate::errors::CfgFalseRemoved { span: attr_span },
        );
        node.visit_attrs(|attrs| {
            attrs.remove(attr_pos);
        });
    }
}
EOF_IMPL_CFG_FALSE
)

# Write the new impl block to a temporary file
printf "%s\n" "$NEW_IMPL_BLOCK" > /tmp/new_impl_block.tmp

# Insert the new impl block after the calculated line
sed -i "${MUT_VISITOR_IMPL_END_LINE}r /tmp/new_impl_block.tmp" "$FILE"

rm /tmp/new_impl_block.tmp


echo "Changes applied. Please review the file and rebuild."
