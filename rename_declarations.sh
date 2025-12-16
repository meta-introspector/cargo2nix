#!/bin/bash

BASE_DIR="submodules/rust/compiler/rustc_expand/src/ast_fragments_split/"

echo "Renaming files to match declarations..."

# Renames for InvocationCollector and its methods
mv "${BASE_DIR}ast_fragments_helpers_collector.rs" "${BASE_DIR}invocation_collector.rs"
mv "${BASE_DIR}ast_fragments_helpers_cfg.rs" "${BASE_DIR}invocation_collector_cfg.rs"
mv "${BASE_DIR}ast_fragments_helpers_check_attr.rs" "${BASE_DIR}invocation_collector_check_attributes.rs"
mv "${BASE_DIR}ast_fragments_helpers_collect_attrs.rs" "${BASE_DIR}invocation_collector_collect_attr.rs"
mv "${BASE_DIR}ast_fragments_helpers_collect_bang.rs" "${BASE_DIR}invocation_collector_collect_bang.rs"
mv "${BASE_DIR}ast_fragments_helpers_collect_glob.rs" "${BASE_DIR}invocation_collector_collect_glob_delegation.rs"
mv "${BASE_DIR}ast_fragments_helpers_collect.rs" "${BASE_DIR}invocation_collector_collect.rs"
mv "${BASE_DIR}ast_fragments_helpers_expand_cfg_1.rs" "${BASE_DIR}invocation_collector_expand_cfg_attr.rs"
mv "${BASE_DIR}ast_fragments_helpers_expand_cfg_true.rs" "${BASE_DIR}invocation_collector_expand_cfg_true.rs"
mv "${BASE_DIR}ast_fragments_helpers_flat_map_node.rs" "${BASE_DIR}invocation_collector_flat_map_node.rs"
mv "${BASE_DIR}ast_fragments_helpers_take_firstattr.rs" "${BASE_DIR}invocation_collector_take_first_attr.rs"
mv "${BASE_DIR}ast_fragments_helpers_visit_node.rs" "${BASE_DIR}invocation_collector_visit_node.rs"

# Renames for other significant structs/enums
mv "${BASE_DIR}ast_fragments_helpers_dummy.rs" "${BASE_DIR}dummy_visitor_support.rs"
mv "${BASE_DIR}ast_fragments_helpers_expansion.rs" "${BASE_DIR}expansion_config.rs"

echo "Files renamed."

echo "Cleaning up any .rs~, .rs.bak files generated during manual process"
rm -f "${BASE_DIR}"*.rs~
rm -f "${BASE_DIR}"*.rs.bak
echo "Cleanup complete."

echo "Now, you will need to update the mod.rs or the main file to reflect these new module names."
