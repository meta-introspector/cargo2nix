#!/bin/bash

FILE="submodules/rust/compiler/rustc_expand/src/ast_fragments_split/ast_fragments_helpers.rs"

echo "Applying changes to $FILE"

# --- Change InvocationCollector::visit_node ---
OLD_VISIT_NODE_BLOCK_START_PATTERN='                            if self.expand_cfg_true(node, attr, pos).as_bool() {'
OLD_VISIT_NODE_BLOCK_END_PATTERN='                            return;'

NEW_VISIT_NODE_BLOCK=$(cat <<'EOF_NEW_VISIT_NODE'
                            if self.expand_cfg_true(node, attr, pos).as_bool() {
                                continue;
                            }

                            self.report_cfg_false(node, span, pos);
                            return;
EOF_NEW_VISIT_NODE
)

START_LINE=$(grep -n "$OLD_VISIT_NODE_BLOCK_START_PATTERN" "$FILE" | cut -d: -f1)
END_LINE=$(grep -n "$OLD_VISIT_NODE_BLOCK_END_PATTERN" "$FILE" | cut -d: -f1)

if [ -z "$START_LINE" ] || [ -z "$END_LINE" ]; then
    echo "Error: Could not find start or end line for the visit_node block."
    exit 1
fi

sed -i.bak "${START_LINE},${END_LINE}c\\n$(echo "$NEW_VISIT_NODE_BLOCK" | sed -e 's/[]\/\$\*\.^\[\]/\\&/g')"
"
$FILE


# --- Implement CfgFalseReporter for InvocationCollector ---
# Find the end of the `impl<'a, 'b> MutVisitor for InvocationCollector<'a, 'b>` block
# And insert the new impl block after it.
INSERT_AFTER_LINE=$(grep -n '}' "$FILE" | grep -E 'impl<\'\'a, \'\'b> MutVisitor for InvocationCollector<\'\'a, \'\'b> {' -B 100 | tail -1 | cut -d: -f1)

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

# Insert the new impl block
sed -i.bak "${INSERT_AFTER_LINE}a\\n$(echo "$NEW_IMPL_BLOCK" | sed -e 's/[]\/\$\*\.^\[\]/\\&/g')"
"
$FILE


echo "Changes applied. Please review the file and rebuild."