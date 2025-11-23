use super::task::Task;
use std::collections::HashMap;
#[cfg(feature = "tool_traits_lib_enabled")]
use tool_traits_lib::serde_adapter::{CurrentSerdeAdapter, SerdeAdapter}; // Assuming Task is in a sibling module

// Define a struct to represent the overall plan
#[cfg_attr(feature = "serde_enabled", derive(Debug, Deserialize, Serialize))]
pub struct Plan {
    pub tasks: HashMap<String, Task>, // Using HashMap for easy lookup by task name
}
