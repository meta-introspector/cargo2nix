use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Define a struct to represent a single task from the TOML files
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
    pub path: Option<String>,    // Path where the command should be executed
}
