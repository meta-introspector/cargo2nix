#!/bin/bash

FILE="submodules/rust/compiler/rustc_expand_base_lib/src/invocation_collector_node.rs"

echo "Removing expand_cfg_false implementation from $FILE"

# Get line numbers for expand_cfg_false implementation
# Escaping single quotes in the grep pattern
EXPAND_CFG_FALSE_IMPL_START=$(grep -n "    fn expand_cfg_false(&mut self, collector: &mut InvocationCollector<'_,'_'>, pos: usize, span: Span) {" "$FILE" | cut -d: -f1)
EXPAND_CFG_FALSE_IMPL_END=$(grep -n '        });' "$FILE" | tail -1 | cut -d: -f1) # This is the last line of the function.

# Delete the lines
sed -i.bak "${EXPAND_CFG_FALSE_IMPL_START},${EXPAND_CFG_FALSE_IMPL_END}d" "$FILE"

echo "Changes applied. Please review the file and rebuild."
