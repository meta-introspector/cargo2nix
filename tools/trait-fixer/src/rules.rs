use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub enum RuleKind {
    #[serde(rename = "add_derive")]
    AddDerive,
    #[serde(rename = "add_impl")]
    AddImpl,
    #[serde(rename = "add_impl_default")]
    AddImplDefault,
    #[serde(rename = "remove_impl")]
    RemoveImpl,
}

#[derive(Debug, Deserialize)]
pub struct Rule {
    pub kind: RuleKind,
    pub trait_name: Vec<String>, // Renamed 'trait' to 'trait_name' as 'trait' is a reserved keyword
    #[serde(default)]
    pub apply_to: Vec<String>,
    pub condition: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub rule: Vec<Rule>,
}

impl Config {
    pub fn load() -> Self {
        let path = std::env::var("TRAIT_FIXER_CONFIG")
            .unwrap_or_else(|_| "rules.toml".to_string());
        let content = std::fs::read_to_string(path).expect("Failed to read rules.toml");
        toml::from_str(&content).expect("Invalid TOML in rules.toml")
    }
}
