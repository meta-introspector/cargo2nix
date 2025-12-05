use anyhow::Result;
use regex::{self, Regex};
use tool_traits_lib::regex_adapter::{RegexCaptures, RegexMatcher}; // Using anyhow for Result type // For the actual regex implementation

// RealRegexMatcher: Uses the actual regex crate for matching
#[derive(Debug)]
pub struct RealRegexMatcher {
    regex: Regex,
}

impl RegexMatcher for RealRegexMatcher {
    fn new(re: &str) -> Result<Self, String> {
        let regex = Regex::new(re).map_err(|e| e.to_string())?;
        Ok(RealRegexMatcher { regex })
    }

    fn is_match(&self, text: &str) -> bool {
        self.regex.is_match(text)
    }

    fn captures<'t>(&'t self, text: &'t str) -> Option<Box<dyn RegexCaptures + 't>> {
        self.regex
            .captures(text)
            .map(|caps| Box::new(RealRegexCaptures { captures: caps }) as Box<dyn RegexCaptures>)
    }
}

pub struct RealRegexCaptures<'t> {
    captures: regex::Captures<'t>,
}

impl<'t> RegexCaptures for RealRegexCaptures<'t> {
    fn get(&self, i: usize) -> Option<&str> {
        self.captures.get(i).map(|m| m.as_str())
    }
    fn len(&self) -> usize {
        self.captures.len()
    }
}

// DummyRegexMatcher: Provides dummy implementations for testing or when regex feature is not enabled
#[derive(Debug)]
pub struct DummyRegexMatcher;

impl DummyRegexMatcher {
    pub fn new(_re: &str) -> Result<Self> {
        Ok(DummyRegexMatcher)
    }
}

impl RegexMatcher for DummyRegexMatcher {
    fn new(_re: &str) -> Result<Self, String> {
        Ok(DummyRegexMatcher)
    }

    fn is_match(&self, _text: &str) -> bool {
        true // Always matches in dummy mode
    }

    fn captures<'t>(&'t self, _text: &'t str) -> Option<Box<dyn RegexCaptures + 't>> {
        None // No captures in dummy mode
    }
}

pub struct DummyRegexCaptures;

impl<'t> RegexCaptures for DummyRegexCaptures {
    fn get(&self, _i: usize) -> Option<&str> {
        None
    }
    fn len(&self) -> usize {
        0
    }
}

// Conditional type alias for CurrentRegexMatcher
#[cfg(feature = "regex_enabled")]
pub type CurrentRegexMatcher = RealRegexMatcher;
#[cfg(not(feature = "regex_enabled"))]
pub type CurrentRegexMatcher = DummyRegexMatcher;

#[cfg(feature = "regex_enabled")]
pub type CurrentRegexCaptures<'t> = RealRegexCaptures<'t>;
#[cfg(not(feature = "regex_enabled"))]
pub type CurrentRegexCaptures<'t> = DummyRegexCaptures;
