#!/bin/bash

ORIG_FILE="submodules/rust/compiler/rustc_expand/src/ast_fragments_types.rs"
SPLIT_DIR="submodules/rust/compiler/rustc_expand/src/ast_fragments_split"

echo "--- Generating content for ast_fragments_enums.rs ---"
echo "Please manually copy the output below to $SPLIT_DIR/ast_fragments_enums.rs"
echo "Content includes initial 'use' statements, pub enum AstFragment, pub enum AstFragmentKind, and impl AstFragmentKind."
# Use sed to get lines from the start of the file up to and including the end of `impl AstFragmentKind`
# This requires a pattern for the end of `impl AstFragmentKind`
# Looking at the file, it ends before `impl AstFragment {`
sed -n '1,/^impl AstFragmentKind {/p' "$ORIG_FILE"
echo "// End of AstFragmentKind, look for the start of impl AstFragment { to define the end of ast_fragments_enums.rs content"
sed -n '/^        impl AstFragment {/,/^            pub fn make_ast<T: InvocationCollectorNode>(self) -> T::OutputTy {/p' "$ORIG_FILE"


echo ""
echo "--- Generating content for ast_fragments_macro.rs ---"
echo "Please manually copy the output below to $SPLIT_DIR/ast_fragments_macro.rs"
echo "Content includes the macro_rules! ast_fragments! { ... } definition."
# Use sed to extract the macro definition. This is highly brittle.
# A better approach is to rely on manual cut/paste for the macro itself.
# This grep will give context.
grep -E -A 100 "macro_rules! ast_fragments!" "$ORIG_FILE"

echo ""
echo "--- Generating content for ast_fragments_helpers.rs ---"
echo "Please manually copy the output below to $SPLIT_DIR/ast_fragments_helpers.rs"
echo "Content includes SupportsMacroExpansion, AddSemicolon, DummyAstNode trait/impls, InvocationCollectorNode trait, build_single_delegations, ExpansionConfig, walk_* functions, InvocationCollector struct/impl."
# Start after the ast_fragments! macro and its invocation, up to the InvocationCollectorNode impls
grep -n "pub enum SupportsMacroExpansion" "$ORIG_FILE" | head -n 1 | awk -F: '{print $1}' > /tmp/start_helpers.txt
START_HELPERS=$(cat /tmp/start_helpers.txt)
grep -n "impl InvocationCollectorNode for Box<ast::Item>" "$ORIG_FILE" | head -n 1 | awk -F: '{print $1}' > /tmp/start_node_impls.txt
START_NODE_IMPLS=$(cat /tmp/start_node_impls.txt)
sed -n "${START_HELPERS},$(($START_NODE_IMPLS - 1))p" "$ORIG_FILE"

echo ""
echo "--- Generating content for ast_fragments_node_impls.rs ---"
echo "Please manually copy the output below to $SPLIT_DIR/ast_fragments_node_impls.rs"
echo "Content includes all 'impl InvocationCollectorNode for ...' blocks."
# Start from the first 'impl InvocationCollectorNode for' block until the end of the file.
grep -n "impl InvocationCollectorNode for Box<ast::Item>" "$ORIG_FILE" | head -n 1 | awk -F: '{print $1}' > /tmp/start_node_impls_actual.txt
START_NODE_IMPLS_ACTUAL=$(cat /tmp/start_node_impls_actual.txt)
sed -n "${START_NODE_IMPLS_ACTUAL},$p" "$ORIG_FILE"

echo ""
echo "Once you have created these files, please remember to remove their content from the original $ORIG_FILE."
echo "Also, in $ORIG_FILE, you will need to add 'mod' statements for the new modules, and potentially 'use' statements in the new files."
echo "Specifically, in $ORIG_FILE, replace the content with something like:"
echo "use std::rc::Rc;"
echo "use std::{iter, mem, slice, sync::Arc};"
echo ""
echo "// Other necessary use statements that are common to all modules"
echo ""
echo "mod ast_fragments_enums;"
echo "mod ast_fragments_macro;"
echo "mod ast_fragments_helpers;"
echo "mod ast_fragments_node_impls;"
echo ""
echo "And in each new file, you will need to add appropriate 'use' statements from the original file."
echo "This manual work is unavoidable due to the complexity of Rust's macro system and interdependencies."

rm -f /tmp/enum_lines.txt /tmp/macro_start.txt /tmp/macro_end.txt /tmp/start_helpers.txt /tmp/start_node_impls.txt /tmp/start_node_impls_actual.txt