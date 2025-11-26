use std::fmt::Debug;
use anyhow::Result; // Use anyhow::Result for consistency
#[cfg(feature = "serde_enabled")]
use serde::{Deserialize, Serialize}; // Added for conditional derives
#[cfg(feature = "serde_enabled")]
use serde_json; // Added for RealSerdeAdapter implementation

pub trait SerdeAdapter: Send + Sync {
    fn to_string_pretty<T: ?Sized + Debug + Serialize>(&self, value: &T) -> Result<String, String>;
    fn from_str<'a, T: Debug + Deserialize<'a>>(&self, s: &'a str) -> Result<T, String>;
}

// RealSerdeAdapter: Uses serde_json for actual serialization/deserialization
#[cfg(feature = "serde_enabled")]
#[derive(Debug)]
pub struct RealSerdeAdapter;

#[cfg(feature = "serde_enabled")]
impl SerdeAdapter for RealSerdeAdapter {
    fn to_string_pretty<T: ?Sized + Debug + Serialize>(&self, value: &T) -> Result<String, String> {
        serde_json::to_string_pretty(value)
            .map_err(|e| format!("Failed to serialize to JSON: {}", e))
    }

    fn from_str<'a, T: Debug + Deserialize<'a>>(&self, s: &'a str) -> Result<T, String> {
        serde_json::from_str(s).map_err(|e| format!("Failed to deserialize from JSON: {}", e))
    }
}

// DummySerdeAdapter: Provides dummy implementations for when serde is not enabled
#[cfg(not(feature = "serde_enabled"))]
#[derive(Debug)]
pub struct DummySerdeAdapter;

#[cfg(not(feature = "serde_enabled"))]
impl SerdeAdapter for DummySerdeAdapter {
    fn to_string_pretty<T: ?Sized + Debug>(&self, _value: &T) -> Result<String, String> {
        Ok("dummy_serialized_string".to_string())
    }

    fn from_str<'a, T: Debug + Deserialize<'a>>(&self, _s: &'a str) -> Result<T, String> {
        Err("Serde is not enabled, cannot deserialize.".to_string())
    }
}

// Conditional type alias for CurrentSerdeAdapter
#[cfg(feature = "serde_enabled")]
pub type CurrentSerdeAdapter = RealSerdeAdapter;

#[cfg(not(feature = "serde_enabled"))]
pub type CurrentSerdeAdapter = DummySerdeAdapter;