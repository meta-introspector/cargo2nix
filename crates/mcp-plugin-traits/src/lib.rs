use anyhow::Result;
use serde::{Deserialize, Serialize}; // Add this

/// Represents the morphological index of a plugin, describing its capabilities and structure.
#[derive(Debug, Serialize, Deserialize, Clone)] // Added Clone for easier handling if needed
pub struct MorphologicalIndex {
    /// List of capabilities provided by the plugin (e.g., "code_analysis", "ast_transformation").
    pub capabilities: Vec<String>,
    /// Supported input data formats (e.g., "rust_code", "minizinc_model", "json").
    pub input_formats: Vec<String>,
    /// Supported output data formats.
    pub output_formats: Vec<String>,
    /// Semantic tags or keywords describing the plugin's domain or function.
    pub semantic_tags: Vec<String>,
    // Add more fields as needed for detailed morphological description
}

pub trait McpPlugin {
    /// Returns the name of the plugin.
    fn name(&self) -> &str;

    /// Returns the version of the plugin.
    fn version(&self) -> &str;

    /// Executes the plugin's core logic with a given input.
    /// Returns a Result containing the output string or an anyhow::Error.
    fn execute(&self, input: &str) -> Result<String>;

    /// Returns the morphological index of the plugin.
    fn morphological_index(&self) -> MorphologicalIndex; // Add this
}

// Optional: A helper function to create a Box<dyn McpPlugin> from a concrete plugin type.
// This might be useful for dynamic loading.
// pub fn create_plugin() -> Box<dyn McpPlugin> {
//     // This would be implemented by the plugin crate itself.
// }
