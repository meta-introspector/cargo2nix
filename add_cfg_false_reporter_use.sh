#!/bin/bash

FILE="submodules/rust/compiler/rustc_expand_base_lib/src/invocation_collector_node.rs"

echo "Correcting use statement for CfgFalseReporter in $FILE"

# Remove the incorrectly inserted line
sed -i.bak '/^nuse crate::cfg_false_reporter::CfgFalseReporter;/d' "$FILE"

# Find the line to insert the use statement after (after existing use crate::{...});
INSERT_AFTER_LINE=$(grep -n 'use crate::{Annotatable, AstFragment, AstFragmentKind, AddSemicolon};' "$FILE" | head -1 | cut -d: -f1)

# The correct new use statement
NEW_USE_STATEMENT="use crate::cfg_false_reporter::CfgFalseReporter;"

# Insert the correct use statement after the specified line
sed -i.bak "${INSERT_AFTER_LINE}a\${NEW_USE_STATEMENT}" "$FILE"

echo "Changes applied. Please review the file and rebuild."