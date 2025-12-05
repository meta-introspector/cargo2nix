use anyhow::Result; // Use anyhow::Result for consistency
use serde::{Deserialize, Serialize}; // Added for conditional derives
use serde_json;
use std::fmt::Debug; // Added for RealSerdeAdapter implementation

pub trait SerdeAdapter: Send + Sync {
    fn to_string_pretty<T: ?Sized + Debug + Serialize>(&self, value: &T) -> Result<String, String>;
    fn from_str<'a, T: Debug + Deserialize<'a>>(&self, s: &'a str) -> Result<T, String>;
}

// RealSerdeAdapter: Uses serde_json for actual serialization/deserialization
#[derive(Debug)]
pub struct RealSerdeAdapter;

impl SerdeAdapter for RealSerdeAdapter {
    fn to_string_pretty<T: ?Sized + Debug + Serialize>(&self, value: &T) -> Result<String, String> {
        serde_json::to_string_pretty(value)
            .map_err(|e| format!("Failed to serialize to JSON: {}", e))
    }

    fn from_str<'a, T: Debug + Deserialize<'a>>(&self, s: &'a str) -> Result<T, String> {
        serde_json::from_str(s).map_err(|e| format!("Failed to deserialize from JSON: {}", e))
    }
}

// Conditional type alias for CurrentSerdeAdapter
pub type CurrentSerdeAdapter = RealSerdeAdapter;
