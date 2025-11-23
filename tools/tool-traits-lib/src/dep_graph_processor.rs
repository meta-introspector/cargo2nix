use super::types::MergedCrateInfo;
use std::collections::HashMap; // Assuming MergedCrateInfo is in types.rs

pub trait DepGraphProcessor: Send + Sync {
    fn process_dep_graph(
        &self,
        dot_content: &str,
    ) -> std::result::Result<HashMap<String, MergedCrateInfo>, String>;
}
