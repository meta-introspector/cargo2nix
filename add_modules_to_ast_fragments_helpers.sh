#!/bin/bash

FILE="submodules/rust/compiler/rustc_expand/src/ast_fragments_split/ast_fragments_helpers.rs"
BASE_DIR="submodules/rust/compiler/rustc_expand/src/ast_fragments_split/" # Still useful for context/other commands

echo "Adding mod declarations to ${FILE}..."

# This list comes from the manual git commit output you provided and my renaming plan.
declare -a MODULE_NAMES=(
    "invocation_collector"
    "invocation_collector_cfg"
    "invocation_collector_check_attributes"
    "invocation_collector_collect"
    "invocation_collector_collect_attr"
    "invocation_collector_collect_bang"
    "invocation_collector_collect_glob_delegation"
    "invocation_collector_expand_cfg_attr"
    "invocation_collector_expand_cfg_true"
    "invocation_collector_flat_map_node"
    "invocation_collector_take_first_attr"
    "invocation_collector_visit_node"
    "dummy_visitor_support"
    "expansion_config"
    "ast_fragments_helpers_walk_helpers_1" # Renamed from ast_fragments_helpers_1
    "ast_fragments_helpers_walk_helpers_2" # Renamed from ast_fragments_helpers_2
    "ast_fragments_helpers_visit_id_helpers" # Renamed from ast_fragments_helpers_visit1
    "ast_fragments_helpers_filter_map"
    "ast_fragments_helpers_flat_map_arm"
    "ast_fragments_helpers_flat_map_expr_field"
    "ast_fragments_helpers_flat_map_generic"
    "ast_fragments_helpers_flat_map_item"
    "ast_fragments_helpers_flat_map_param"
    "ast_fragments_helpers_flat_map_pat_associate"
    "ast_fragments_helpers_flat_map_pat_field"
    "ast_fragments_helpers_flat_map_pat_field_ref"
    "ast_fragments_helpers_flat_map_pat_foreign"
    "ast_fragments_helpers_flat_map_pat_variant"
    "ast_fragments_helpers_flat_map_pat_where"
    "ast_fragments_helpers_flat_map_statement"
    "ast_fragments_helpers_visit_block"
    "ast_fragments_helpers_visit_crate"
    "ast_fragments_helpers_visit_expr"
    "ast_fragments_helpers_visit_id"
    "ast_fragments_helpers_visit_pat"
    "ast_fragments_helpers_visit_type"
    "ast_fragments_helpers_walk"
)

# Append the mod declarations to the file.
for module in "${MODULE_NAMES[@]}"; do
    echo "pub mod ${module};" >> "$FILE" # Added 'pub'
done

echo "Mod declarations added to ${FILE}. Please review and rebuild."