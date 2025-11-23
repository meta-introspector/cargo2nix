// cargo-submodule-tool-lib/src/analysis/regex_matcher.rs
use anyhow::Result;
use tool_traits_lib::{RegexCaptures, RegexMatcher}; // Using anyhow for Result type

#[cfg(feature = "regex_enabled")]
use regex::Regex;

#[cfg(feature = "regex_enabled")]
impl<'t> RegexCaptures for regex::Captures<'t> {
    fn get(&self, i: usize) -> Option<&str> {
        self.get(i).map(|m| m.as_str())
    }
    fn len(&self) -> usize {
        self.len()
    }
}

#[cfg(feature = "regex_enabled")]
pub struct RealRegexMatcher {
    regex: Regex,
}

#[cfg(feature = "regex_enabled")]
impl RealRegexMatcher {
    pub fn new(pattern: &str) -> Result<Self> {
        Ok(RealRegexMatcher {
            regex: Regex::new(pattern)?,
        })
    }
}

#[cfg(feature = "regex_enabled")]
impl RegexMatcher for RealRegexMatcher {
    fn is_match(&self, text: &str) -> bool {
        self.regex.is_match(text)
    }
    fn captures<'t>(&'t self, text: &'t str) -> Option<Box<dyn RegexCaptures + 't>> {
        self.regex
            .captures(text)
            .map(|c| Box::new(c) as Box<dyn RegexCaptures + 't>)
    }
}

#[cfg(not(feature = "regex_enabled"))]
pub struct DummyRegexMatcher;

#[cfg(not(feature = "regex_enabled"))]
impl DummyRegexMatcher {
    pub fn new(_pattern: &str) -> Result<Self> {
        Ok(DummyRegexMatcher)
    }
}

#[cfg(not(feature = "regex_enabled"))]
impl RegexMatcher for DummyRegexMatcher {
    fn is_match(&self, _text: &str) -> bool {
        false // Dummy implementation always returns false
    }
    fn captures<'t>(&'t self, _text: &'t str) -> Option<Box<dyn RegexCaptures + 't>> {
        None
    }
}

#[cfg(feature = "regex_enabled")]
pub type CurrentRegexMatcher = RealRegexMatcher;
#[cfg(not(feature = "regex_enabled"))]
pub type CurrentRegexMatcher = DummyRegexMatcher;
