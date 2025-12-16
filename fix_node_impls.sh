#!/bin/bash

FILE="submodules/rust/compiler/rustc_expand_base_lib/src/invocation_collector_node.rs"

echo "Applying sed changes to $FILE"

# --- Fix take_mac_call ---
OLD_TAKE_MAC_CALL_START_LINE=$(grep -n '    fn take_mac_call(self) -> (ast::MacCall, AttrVec, AddSemicolon) {' "$FILE" | cut -d: -f1)
OLD_TAKE_MAC_CALL_END_LINE=$(grep -n '        (mac.macro_id, attrs, add_semicolon)' "$FILE" | cut -d: -f1)

NEW_TAKE_MAC_CALL_BODY=$(cat <<'EOF_TAKE_MAC'
    fn take_mac_call(self) -> (Box<ast::MacCall>, ast::AttrVec, AddSemicolon) {
        let (mac, attrs, add_semicolon) = match self.0.kind {
            ast::StmtKind::MacCall(mac_stmt) => (mac_stmt.mac, mac_stmt.attrs, AddSemicolon::from_bool(true)),
            _ => panic!("called take_mac_call on non-macro statement"),
        };
        (mac, attrs, add_semicolon)
    }
EOF_TAKE_MAC
)

# Delete old take_mac_call block
sed -i.bak "${OLD_TAKE_MAC_CALL_START_LINE},${OLD_TAKE_MAC_CALL_END_LINE}d" "$FILE"

# Insert new take_mac_call block
# Adjust the line number for insertion as lines were deleted
INSERT_LINE=$OLD_TAKE_MAC_CALL_START_LINE

# Using printf and sed 'r' command for robust multi-line insertion
printf '%s\n' "${NEW_TAKE_MAC_CALL_BODY}" > "$FILE.tmp.insert"
sed -i.bak "${INSERT_LINE}r $FILE.tmp.insert" "$FILE"
rm "$FILE.tmp.insert"


# --- Fix expand_cfg_false ---
# Re-read line numbers after the first modification, as line numbers might have shifted.
# Note: Since the number of lines in take_mac_call didn't change (9 lines to 9 lines),
# the original line numbers for expand_cfg_false should still be valid.
OLD_EXPAND_CFG_FALSE_START_LINE=$(grep -n "    fn expand_cfg_false(&mut self, collector: &mut InvocationCollector<'_,'_'>, pos: usize, span: Span) {" "$FILE" | cut -d: -f1)
OLD_EXPAND_CFG_FALSE_END_LINE=$(grep -n '        self.0.attrs.remove(pos);' "$FILE" | cut -d: -f1)

NEW_EXPAND_CFG_FALSE_BODY=$(cat <<'EOF_EXPAND_CFG'
    fn expand_cfg_false(&mut self, collector: &mut InvocationCollector<'_, '_'>, pos: usize, span: Span) {
        // Handle cfg_false logic for statement
        collector.cx.sess.psess.buffer_lint(
            StripUnconfigured::get_unused_attribute_lint(true),
            span,
            collector.cx.current_expansion.lint_node_id,
            errors::CfgFalseRemoved { span },
        );
        self.visit_attrs(|attrs| {
            attrs.remove(pos);
        });
    }
EOF_EXPAND_CFG
)

# Delete old expand_cfg_false block
sed -i.bak "${OLD_EXPAND_CFG_FALSE_START_LINE},${OLD_EXPAND_CFG_FALSE_END_LINE}d" "$FILE"

# Insert new expand_cfg_false block
INSERT_LINE=$OLD_EXPAND_CFG_FALSE_START_LINE

printf '%s\n' "${NEW_EXPAND_CFG_FALSE_BODY}" > "$FILE.tmp.insert"
sed -i.bak "${INSERT_LINE}r $FILE.tmp.insert" "$FILE"
rm "$FILE.tmp.insert"


echo "Changes applied. Please review the file and rebuild."