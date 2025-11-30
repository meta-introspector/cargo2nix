// crates/trait-fixer-rules-real/src/lib.rs

use serde::Deserialize;
use trait_fixer_rules_trait::{ConfigTrait, Rule}; // Import from trait crate
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub rule: Vec<Rule>,
}

impl ConfigTrait for Config {
    fn load() -> Self {
        let path = std::env::var("TRAIT_FIXER_CONFIG")
            .unwrap_or_else(|_| "rules.toml".to_string());
        let content = std::fs::read_to_string(path).expect("Failed to read rules.toml");
        toml::from_str(&content).expect("Invalid TOML in rules.toml")
    }

    fn get_rules(&self) -> &Vec<Rule> {
        &self.rule
    }
}