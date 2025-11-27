use mcp_plugin_traits::{McpPlugin, MorphologicalIndex};
use anyhow::{Result, anyhow};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// This is the concrete plugin implementation
struct ExamplePlugin;

impl McpPlugin for ExamplePlugin {
    fn name(&self) -> &str {
        "ExamplePlugin"
    }

    fn version(&self) -> &str {
        "0.1.0"
    }

    fn execute(&self, input: &str) -> Result<String> {
        eprintln!("ExamplePlugin: Received input: '{}'", input);
        let reversed_input: String = input.chars().rev().collect();
        Ok(format!("Processed by ExamplePlugin (v{}): Reversed input is '{}'", self.version(), reversed_input))
    }

    // Implement the new morphological_index method
    fn morphological_index(&self) -> MorphologicalIndex {
        MorphologicalIndex {
            capabilities: vec!["string_reversal".to_string(), "text_processing".to_string()],
            input_formats: vec!["string".to_string()],
            output_formats: vec!["string".to_string()],
            semantic_tags: vec!["example".to_string(), "utility".to_string(), "demo".to_string()],
        }
    }
}

// C-compatible function to create a new plugin instance
// This will be the entry point for dynamic loading.
#[unsafe(no_mangle)] // Directly following compiler's suggestion
pub unsafe extern "C" fn create_plugin() -> *mut dyn McpPlugin {
    Box::into_raw(Box::new(ExamplePlugin))
}

// C-compatible function to destroy a plugin instance
// This is important to prevent memory leaks when dynamically unloading.
#[unsafe(no_mangle)] // Directly following compiler's suggestion
pub unsafe extern "C" fn destroy_plugin(ptr: *mut dyn McpPlugin) {
    if ptr.is_null() {
        return;
    }
    // Retake ownership and drop the Box, which will deallocate the plugin.
    Box::from_raw(ptr); 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_plugin() {
        let plugin = ExamplePlugin;
        assert_eq!(plugin.name(), "ExamplePlugin");
        assert_eq!(plugin.version(), "0.1.0");

        let input = "hello";
        let expected_output = "Processed by ExamplePlugin (v0.1.0): Reversed input is 'olleh'";
        assert_eq!(plugin.execute(input).unwrap(), expected_output);

        let index = plugin.morphological_index();
        assert!(index.capabilities.contains(&"string_reversal".to_string()));
        assert!(index.input_formats.contains(&"string".to_string()));
    }
}