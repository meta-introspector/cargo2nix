#!/bin/bash

# Patch for gix/src/object/blob.rs - imara_diff import
sed -i 's|use imara_diff::{self, Algorithm, Diff, InternedInput};|use gix_diff::imara_diff::{self, Algorithm, Diff, InternedInput};|g' submodules/gitoxide/gix/src/object/blob.rs

# Patch for gix/src/object/blob.rs - lines function refactor
sed -i '/gix_diff::blob::diff(algorithm, &input, |before: Range<u32>, after: Range<u32>| {/,/});/c\
                    let input = prep.interned_input();\
                    let mut err = None;\
                    let mut lines = Vec::new();\
\
                    let diff = imara_diff::Diff::compute(algorithm, &input);\
                    for hunk in diff.hunks() {\
                        if err.is_some() {\
                            break;\
                        }\
                        lines.clear();\
                        lines.extend(\ 
                            input.before[hunk.before.start as usize..hunk.before.end as usize]\
                                .iter()\
                                .map(|&line| input.interner[line].as_bstr()),\
                        );\
                        let end_of_before = lines.len();\
                        lines.extend(\ 
                            input.after[hunk.after.start as usize..hunk.after.end as usize]\
                                .iter()\
                                .map(|&line| input.interner[line].as_bstr()),\
                        );\
                        let hunk_before = &lines[..end_of_before];\
                        let hunk_after = &lines[end_of_before..];\
                        if hunk_after.is_empty() {\
                            err = process_hunk(lines::Change::Deletion { lines: hunk_before }).err();\
                        } else if hunk_before.is_empty() {\
                            err = process_hunk(lines::Change::Addition { lines: hunk_after }).err();\
                        } else {\
                            err = process_hunk(lines::Change::Modification {\
                                lines_before: hunk_before,\
                                lines_after: hunk_after,
                            })
                            .err();\
                        }\
                    }\
\
                    if let Some(err) = err {\
                        return Err(lines::Error::ProcessHunk(err));\
                    }
' submodules/gitoxide/gix/src/object/blob.rs

# Patch for gix/src/object/blob.rs - line_counts function refactor
sed -i '/Result<Option<gix_diff::blob::sink::Counter<()>>/,/Ok(Some(counter)) )}/c\
        ) -> Result<Option<(u32, u32)>, gix_diff::blob::platform::prepare_diff::Error> {\
            self.resource_cache.options.skip_internal_diff_if_external_is_configured = false;\
\
            let prep = self.resource_cache.prepare_diff()?\
            match prep.operation {\
                Operation::InternalDiff { algorithm } => {\
                    let input = prep.interned_input();\
                    let diff = imara_diff::Diff::compute(algorithm, &input);\
                    Ok(Some((diff.count_removals(), diff.count_additions())))\
                }\
                Operation::ExternalCommand { .. } => {\
                    unreachable!("we disabled that")\
                }\
                Operation::SourceOrDestinationIsBinary => Ok(None),\
            }\
        }\
' submodules/gitoxide/gix/src/object/blob.rs

# Patch for gix/src/object/tree/diff/mod.rs - counts.1/.0
sed -i 's|lines_added += u64::from(counts.insertions);|lines_added += u64::from(counts.1);|g' submodules/gitoxide/gix/src/object/tree/diff/mod.rs
sed -i 's|lines_removed += u64::from(counts.removals);|lines_removed += u64::from(counts.0);|g' submodules/gitoxide/gix/src/object/tree/diff/mod.rs

# Patch for gix/src/revision/spec/parse/delegate/mod.rs - Replacements SmallVec
sed -i 's|type Replacements = SmallVec<[(ObjectId, ObjectId); 1], 1>;|type Replacements = SmallVec<(ObjectId, ObjectId), 1>;|g' submodules/gitoxide/gix/src/revision/spec/parse/delegate/mod.rs

# Patch for gix/src/repository/object.rs - parents SmallVec
sed -i 's|parents: SmallVec<[ObjectId; 1], 1>,|parents: SmallVec<ObjectId, 1>,|g' submodules/gitoxide/gix/src/repository/object.rs
