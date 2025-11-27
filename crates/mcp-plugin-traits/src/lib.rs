use anyhow::Result;

pub trait McpPlugin {
    /// Returns the name of the plugin.
    fn name(&self) -> &str;

    /// Returns the version of the plugin.
    fn version(&self) -> &str;

    /// Executes the plugin's core logic with a given input.
    /// Returns a Result containing the output string or an anyhow::Error.
    fn execute(&self, input: &str) -> Result<String>;
}

// Optional: A helper function to create a Box<dyn McpPlugin> from a concrete plugin type.
// This might be useful for dynamic loading.
// pub fn create_plugin() -> Box<dyn McpPlugin> {
//     // This would be implemented by the plugin crate itself.
// }