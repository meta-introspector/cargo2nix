use anyhow::Result;
use super::types::MergedCrateInfo;
use std::collections::HashMap; // Assuming MergedCrateInfo is in types.rs

pub trait DepGraphProcessor: Send + Sync {
    fn process_dep_graph(
        &self,
        dot_content: &str,
    ) -> Result<HashMap<String, MergedCrateInfo>>;
}
