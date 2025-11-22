use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use super::task::Task; // Assuming Task is in a sibling module

// Define a struct to represent the overall plan
#[derive(Debug, Deserialize, Serialize)]
pub struct Plan {
    pub tasks: HashMap<String, Task>, // Using HashMap for easy lookup by task name
}
