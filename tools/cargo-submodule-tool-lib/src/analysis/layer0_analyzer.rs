#[cfg(feature = "tool_traits_lib_enabled")]
use tool_traits_lib::types::MergedCrateInfo;
#[cfg(feature = "anyhow_enabled")]
use anyhow::{anyhow, Result};
use std::collections::HashMap; // Corrected import

#[cfg(feature = "nix_generation")]
pub trait Layer0Analyzer {
    fn find_most_used_layer0_module(
        &self,
        merged_data: &HashMap<String, MergedCrateInfo>,
    ) -> Result<Option<(String, u32)>>;
}

#[cfg(feature = "nix_generation")]
pub struct RealLayer0Analyzer;

#[cfg(feature = "nix_generation")]
impl Layer0Analyzer for RealLayer0Analyzer {
    fn find_most_used_layer0_module(
        &self,
        merged_data: &HashMap<String, MergedCrateInfo>,
    ) -> Result<Option<(String, u32)>> {
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
                    }
                    None => {
                        most_used_module = Some((crate_name.clone(), info.usage_count));
                    }
                }
            }
        }
        Ok(most_used_module)
    }
}

#[cfg(not(feature = "nix_generation"))]
pub trait Layer0Analyzer {
    fn find_most_used_layer0_module(
        &self,
        merged_data: &HashMap<String, MergedCrateInfo>,
    ) -> Result<Option<(String, u32)>>;
}

#[cfg(not(feature = "nix_generation"))]
pub struct RealLayer0Analyzer;

#[cfg(not(feature = "nix_generation"))]
impl Layer0Analyzer for RealLayer0Analyzer {
    fn find_most_used_layer0_module(
        &self,
        _merged_data: &HashMap<String, MergedCrateInfo>,
    ) -> Result<Option<(String, u32)>> {
        Err(anyhow!(
            "`Layer0Analyzer` requires the `nix_generation` feature to be enabled."
        ))
    }
}
