#!/bin/bash

# This script refactors the 'expand.rs' file by applying specific sed replacements
# to abstract complex types and generic parameters, aligning with the "lisp-like card system" concept.
#
# WARNING: This script modifies the 'expand.rs' file in place.
# It is highly recommended to back up the file before running this script.
#
# Usage: ./refactor_expand_rs.sh

TARGET_FILE="submodules/rust/compiler/rustc_expand/src/expand.rs"

# Ensure the target file exists
if [ ! -f "$TARGET_FILE" ]; then
    echo "Error: Target file $TARGET_FILE not found."
    exit 1
fi

echo "--- Refactoring $TARGET_FILE ---"

# Replacement 1: Update the 'new' function's 'cx' parameter type
# Old: pub fn new(cx: &'a mut crate::base::ExtCtxt<'b, DRT>, monotonic: bool) -> Self {
# New: pub fn new(cx: CtxSpecial!('a, 'b, DRT), monotonic: bool) -> Self {
sed -i "s|pub fn new(cx: &'a mut crate::base::ExtCtxt<'b, DRT>, monotonic: bool) -> Self {|pub fn new(cx: CtxSpecial!('a, 'b, DRT), monotonic: bool) -> Self {|g" "$TARGET_FILE"
if [ $? -ne 0 ]; then echo "Error during sed replacement for 'new' function."; exit 1; fi
echo "Updated 'new' function's 'cx' parameter type."

# Replacement 2: Update 'InvocationCollector' usage in 'collect_invocations'
# Old: let mut collector = InvocationCollector {
# New: let mut collector = CollectorSpecial!('a, 'b, DRT) {
sed -i "s|let mut collector = InvocationCollector {|let mut collector = CollectorSpecial!('a, 'b, DRT) {|g" "$TARGET_FILE"
if [ $? -ne 0 ]; then echo "Error during sed replacement for 'InvocationCollector'."; exit 1; fi
echo "Updated 'InvocationCollector' usage."

# Replacement 3: Update 'DeriveResolution' usage in 'fully_expand_fragment'
# Old: .map(|DeriveResolution { path, item, exts: _, is_const }| {
# New: .map(|ResolverBound { path, item, exts: _, is_const }| {
sed -i "s|\.map(|DeriveResolution { path, item, exts: _, is_const }||\.map(|ResolverBound { path, item, exts: _, is_const }||g" "$TARGET_FILE"
if [ $? -ne 0 ]; then echo "Error during sed replacement for 'DeriveResolution'."; exit 1; fi
echo "Updated 'DeriveResolution' usage."

echo "--- Refactoring complete ---"
