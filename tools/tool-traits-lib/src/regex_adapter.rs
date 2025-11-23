use std::fmt::Debug;

pub trait RegexAdapter: Send + Sync {
    fn new(re: &str) -> Result<Self, String>
    where
        Self: Sized;
    fn is_match(&self, text: &str) -> bool;
    fn captures<'t>(&'t self, text: &'t str) -> Option<Box<dyn RegexCaptures + 't>>;
}

pub trait RegexCaptures {
    fn get(&self, name_or_idx: impl RegexCaptureIndex) -> Option<String>;
}

pub trait RegexCaptureIndex {
    fn to_index(&self) -> Option<usize>;
    fn to_name(&self) -> Option<&str>;
}

impl RegexCaptureIndex for usize {
    fn to_index(&self) -> Option<usize> {
        Some(*self)
    }
    fn to_name(&self) -> Option<&str> {
        None
    }
}

impl RegexCaptureIndex for &str {
    fn to_index(&self) -> Option<usize> {
        None
    }
    fn to_name(&self) -> Option<&str> {
        Some(*self)
    }
}

#[cfg(not(feature = "regex_enabled"))]
pub struct DummyRegexAdapter;

#[cfg(not(feature = "regex_enabled"))]
impl RegexAdapter for DummyRegexAdapter {
    fn new(_re: &str) -> Result<Self, String> {
        Err("Regex feature not enabled.".to_string())
    }

    fn is_match(&self, _text: &str) -> bool {
        false
    }

    fn captures<'t>(&'t self, _text: &'t str) -> Option<Box<dyn RegexCaptures + 't>> {
        None
    }
}

#[cfg(not(feature = "regex_enabled"))]
pub struct DummyRegexCaptures;

#[cfg(not(feature = "regex_enabled"))]
impl RegexCaptures for DummyRegexCaptures {
    fn get(&self, _name_or_idx: impl RegexCaptureIndex) -> Option<String> {
        None
    }
}

#[cfg(all(feature = "regex_enabled", feature = "real_regex_adapter_lib_enabled"))]
pub type CurrentRegexAdapter = real_regex_adapter_lib::RealRegexAdapter;
#[cfg(not(all(feature = "regex_enabled", feature = "real_regex_adapter_lib_enabled")))]
pub type CurrentRegexAdapter = DummyRegexAdapter;
