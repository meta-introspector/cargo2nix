use anyhow::Result;
use std::collections::HashMap;
#[cfg(feature = "nix_generation")] // Conditionally compile regex
use regex::Regex;
#[cfg(feature = "nix_generation")] // Conditionally compile lazy_static
use lazy_static::lazy_static; // Add lazy_static import

use git_wrapper_lib::git_types::MergedCrateInfo; // Import MergedCrateInfo from git_wrapper_lib

pub trait DepGraphProcessor {
    fn process_dep_graph(&self, dot_content: &str) -> Result<HashMap<String, MergedCrateInfo>>;
}

#[cfg(feature = "nix_generation")] // Conditionally compile RealDepGraphProcessor
pub struct RealDepGraphProcessor;

#[cfg(feature = "nix_generation")] // Conditionally compile impl block
impl DepGraphProcessor for RealDepGraphProcessor {
    fn process_dep_graph(&self, dot_content: &str) -> Result<HashMap<String, MergedCrateInfo>> {
        lazy_static! {
            static ref NODE_RE: Regex = Regex::new(r#"^  "([^"]+)" \[label="([^"]+)"\];$"#).unwrap();
            static ref EDGE_RE: Regex = Regex::new(r#"^  "([^"]+)" -> "([^"]+)"(?: \[label="([^"]+)"\])?;$"#).unwrap();
            static ref VERSION_RE: Regex = Regex::new(r#" v\d+\.\d+\.\d+(?:-\S+)?"#).unwrap();
        }

        let mut nodes = HashMap::new();
        let mut edges = Vec::new();

        for line in dot_content.lines() {
            if let Some(captures) = NODE_RE.captures(line) {
                let mut node_name = captures[1].to_string();
                let label = captures[2].to_string();
                // Remove version from node name for consistent keying
                node_name = VERSION_RE.replace_all(&node_name, "").trim().to_string();
                nodes.insert(node_name, MergedCrateInfo {
                    layer: -1, // Default value
                    usage_count: 0, // Default value
                });
            } else if let Some(captures) = EDGE_RE.captures(line) {
                let mut from_node = captures[1].to_string();
                let mut to_node = captures[2].to_string();
                from_node = VERSION_RE.replace_all(&from_node, "").trim().to_string();
                to_node = VERSION_RE.replace_all(&to_node, "").trim().to_string();
                edges.push((from_node, to_node));
            }
        }

        // Populate dependencies (this part is not directly used in MergedCrateInfo, but keeping for context)
        // for (from, to) in edges {
        //     if let Some(info) = nodes.get_mut(&from) {
        //         // info.dependencies.push(to); // MergedCrateInfo doesn't have dependencies field
        //     }
        // }

        Ok(nodes)
    }
}

#[cfg(not(feature = "nix_generation"))] // Dummy implementation when nix_generation is not enabled
pub struct RealDepGraphProcessor;

#[cfg(not(feature = "nix_generation"))]
impl DepGraphProcessor for RealDepGraphProcessor {
    fn process_dep_graph(&self, _dot_content: &str) -> Result<HashMap<String, MergedCrateInfo>> {
        anyhow::bail!("`DepGraphProcessor` requires the `nix_generation` feature to be enabled.");
    }
}