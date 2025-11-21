use anyhow::{Result, Context};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use regex::Regex;
use lazy_static::lazy_static; // Add lazy_static import

pub trait DepGraphProcessor {
    fn parse_dot_file(&self, dot_file_path: &Path) -> Result<(HashMap<String, Vec<String>>, HashSet<String>)>;
    fn calculate_layers(&self, graph: &HashMap<String, Vec<String>>, nodes: &HashSet<String>) -> Result<HashMap<String, i32>>;
}

pub struct RealDepGraphProcessor;

impl DepGraphProcessor for RealDepGraphProcessor {
    fn parse_dot_file(&self, dot_file_path: &Path) -> Result<(HashMap<String, Vec<String>>, HashSet<String>)> {
        lazy_static! {
            static ref NODE_RE: Regex = Regex::new(r#"(\d+)\s*\[\s*label\s*=\s*"([^"]+)""#).unwrap();
            static ref EDGE_RE: Regex = Regex::new(r"(\d+)\s*->\s*(\d+)").unwrap();
            static ref VERSION_RE: Regex = Regex::new(r"\s+\d+\.\d+\.\d+.*\").unwrap(); // Regex for version removal
        }

        let mut graph = HashMap::new();
        let mut nodes = HashSet::new();
        let mut node_id_to_name = HashMap::new();

        let content = std::fs::read_to_string(dot_file_path)
            .with_context(|| format!("Failed to read dot file: {}", dot_file_path.display()))?;

        for line in content.lines() {
            if let Some(captures) = NODE_RE.captures(line) {
                let node_id = captures[1].to_string();
                let mut node_name = captures[2].to_string();
                // Remove version numbers from node_name for cleaner processing
                node_name = VERSION_RE.replace_all(&node_name, "").trim().to_string();
                
                node_id_to_name.insert(node_id, node_name.clone());
                nodes.insert(node_name);
            } else if let Some(captures) = EDGE_RE.captures(line) {
                let source_id = &captures[1];
                let target_id = &captures[2];

                if let (Some(source_name), Some(target_name)) = (node_id_to_name.get(source_id), node_id_to_name.get(target_id)) {
                    graph.entry(source_name.clone()).or_insert_with(Vec::new).push(target_name.clone());
                }
            }
        }
        Ok((graph, nodes))
    }

    fn calculate_layers(&self, graph: &HashMap<String, Vec<String>>, all_nodes: &HashSet<String>) -> Result<HashMap<String, i32>> {
        let mut layers = HashMap::new();
        let mut current_layer = 0;
        let mut nodes_in_current_layer: HashSet<String> = all_nodes.iter()
            .filter(|node| !graph.contains_key(*node) || graph[*node].is_empty())
            .cloned()
            .collect();

        let mut processed_nodes = HashSet::new();

        while !nodes_in_current_layer.is_empty() {
            let mut next_layer_nodes = HashSet::new();
            for node in &nodes_in_current_layer {
                if processed_nodes.insert(node.clone()) {
                    layers.insert(node.clone(), current_layer);
                }
            }

            for potential_next_layer_node in all_nodes.iter().filter(|node| !processed_nodes.contains(*node)) {
                let mut all_deps_assigned = true;
                if let Some(dependencies) = graph.get(potential_next_layer_node) {
                    for dep in dependencies {
                        if !processed_nodes.contains(dep) {
                            all_deps_assigned = false;
                            break;
                        }
                    }
                }
                if all_deps_assigned {
                    next_layer_nodes.insert(potential_next_layer_node.clone());
                }
            }
            nodes_in_current_layer = next_layer_nodes;
            current_layer += 1;
        }

        // Handle any remaining unassigned nodes (e.g., due to cycles)
        for node in all_nodes {
            if !layers.contains_key(node) {
                layers.insert(node.clone(), current_layer); // Assign to a "cycle" layer
            }
        }

        Ok(layers)
    }
}
