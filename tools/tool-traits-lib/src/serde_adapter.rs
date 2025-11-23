#[cfg(feature = "serde_json_enabled")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::path::Path;

pub trait SerdeAdapter: Send + Sync {
    fn to_string_pretty<T: ?Sized + Serialize + Debug>(&self, value: &T) -> Result<String, String>;
    fn from_str<'a, T: Deserialize<'a> + Debug>(&self, s: &'a str) -> Result<T, String>;
}

#[cfg(feature = "serde_json_enabled")]
pub struct RealSerdeAdapter;

#[cfg(feature = "serde_json_enabled")]
impl SerdeAdapter for RealSerdeAdapter {
    fn to_string_pretty<T: ?Sized + Serialize + Debug>(&self, value: &T) -> Result<String, String> {
        serde_json::to_string_pretty(value)
            .map_err(|e| format!("Failed to serialize to JSON: {:?}", e))
    }

    fn from_str<'a, T: Deserialize<'a> + Debug>(&self, s: &'a str) -> Result<T, String> {
        serde_json::from_str(s).map_err(|e| format!("Failed to deserialize from JSON: {:?}", e))
    }
}

#[cfg(not(feature = "serde_json_enabled"))]
pub struct DummySerdeAdapter;

#[cfg(not(feature = "serde_json_enabled"))]
impl SerdeAdapter for DummySerdeAdapter {
    fn to_string_pretty<T: ?Sized + Serialize + Debug>(
        &self,
        _value: &T,
    ) -> Result<String, String> {
        Err("Serde JSON feature not enabled.".to_string())
    }

    fn from_str<'a, T: Deserialize<'a> + Debug>(&self, _s: &'a str) -> Result<T, String> {
        Err("Serde JSON feature not enabled.".to_string())
    }
}

#[cfg(feature = "serde_json_enabled")]
pub type CurrentSerdeAdapter = RealSerdeAdapter;
#[cfg(not(feature = "serde_json_enabled"))]
pub type CurrentSerdeAdapter = DummySerdeAdapter;
