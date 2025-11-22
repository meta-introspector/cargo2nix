use anyhow::Result;
use std::collections::HashMap;
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MergedCrateInfo {
    pub layer: i32,
    pub usage_count: u32,
}

pub trait DepGraphDataMerger {
    fn merge_data(
        &self,
        layer_data: HashMap<String, i32>,
        usage_counts: HashMap<String, u32>,
    ) -> Result<HashMap<String, MergedCrateInfo>>;
}

pub struct RealDepGraphDataMerger;

impl DepGraphDataMerger for RealDepGraphDataMerger {
    fn merge_data(
        &self,
        layer_data: HashMap<String, i32>,
        usage_counts: HashMap<String, u32>,
    ) -> Result<HashMap<String, MergedCrateInfo>> {
        let mut merged_data = HashMap::new();
        let mut all_crates: Vec<String> = layer_data.keys().cloned().collect();
        all_crates.extend(usage_counts.keys().cloned());
        all_crates.sort_unstable();
        all_crates.dedup();

        for crate_name in all_crates {
            let layer = *layer_data.get(&crate_name).unwrap_or(&-1);
            let usage_count = *usage_counts.get(&crate_name).unwrap_or(&0);

            if layer != -1 { // Only include crates that have layer information
                merged_data.insert(
                    crate_name,
                    MergedCrateInfo {
                        layer,
                        usage_count,
                    },
                );
            }
        }
        Ok(merged_data)
    }
}