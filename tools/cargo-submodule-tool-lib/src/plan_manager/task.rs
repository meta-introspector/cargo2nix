use std::collections::HashMap;
#[cfg(feature = "tool_traits_lib_enabled")]
use tool_traits_lib::serde_adapter::{CurrentSerdeAdapter, SerdeAdapter};

// Define a struct to represent a single task from the TOML files
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Task {
    pub name: String,
    pub description: String,
    pub status: String,
    #[serde(default)]
    pub depends_on: Vec<String>,
    #[serde(default)]
    pub command: Option<String>, // Command to execute for this task
    #[serde(default)]
    pub path: Option<String>, // Path where the command should be executed
}
