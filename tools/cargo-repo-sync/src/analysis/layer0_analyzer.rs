use anyhow::Result;
use std::collections::HashMap;
use crate::analysis::dep_graph_data_merger::MergedCrateInfo;

pub trait Layer0Analyzer {
    fn find_most_used_layer0_module(&self, merged_data: &HashMap<String, MergedCrateInfo>) -> Result<Option<(String, u32)>>;
}

pub struct RealLayer0Analyzer;

impl Layer0Analyzer for RealLayer0Analyzer {
    fn find_most_used_layer0_module(&self, merged_data: &HashMap<String, MergedCrateInfo>) -> Result<Option<(String, u32)>> {
        let mut most_used_module: Option<(String, u32)> = None;

        for (crate_name, info) in merged_data {
            if info.layer == 0 {
                match most_used_module {
                    Some((_, current_max_count)) => {
                        if info.usage_count > current_max_count {
                            most_used_module = Some((crate_name.clone(), info.usage_count));
                        } else if info.usage_count == current_max_count {
                            // If there's a tie, prefer lexicographically smaller name
                            if crate_name < &most_used_module.as_ref().unwrap().0 {
                                most_used_module = Some((crate_name.clone(), info.usage_count));
                            }
                        }
                    },
                    None => {
                        most_used_module = Some((crate_name.clone(), info.usage_count));
                    },
                }
            }
        }
        Ok(most_used_module)
    }
}