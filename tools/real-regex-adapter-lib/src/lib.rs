use regex::{Captures, Regex};
use tool_traits_lib::regex_adapter::{RegexAdapter, RegexCaptureIndex, RegexCaptures};

pub struct RealRegexAdapter {
    regex: Regex,
}

impl RegexAdapter for RealRegexAdapter {
    fn new(re: &str) -> Result<Self, String> {
        Regex::new(re)
            .map(|regex| RealRegexAdapter { regex })
            .map_err(|e| format!("Failed to compile regex: {:?}", e))
    }

    fn is_match(&self, text: &str) -> bool {
        self.regex.is_match(text)
    }

    fn captures<'t>(&'t self, text: &'t str) -> Option<Box<dyn RegexCaptures + 't>> {
        self.regex
            .captures(text)
            .map(|captures| Box::new(RealRegexCaptures { captures }) as Box<dyn RegexCaptures + 't>)
    }
}

pub struct RealRegexCaptures<'t> {
    captures: Captures<'t>,
}

impl<'t> RegexCaptures for RealRegexCaptures<'t> {
    fn get(&self, name_or_idx: impl RegexCaptureIndex) -> Option<String> {
        if let Some(idx) = name_or_idx.to_index() {
            self.captures.get(idx).map(|m| m.as_str().to_string())
        } else if let Some(name) = name_or_idx.to_name() {
            self.captures.name(name).map(|m| m.as_str().to_string())
        } else {
            None
        }
    }
}
