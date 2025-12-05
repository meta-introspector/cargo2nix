// crates/trait-fixer-rules-mock/src/lib.rs

use trait_fixer_rules_trait::{ConfigTrait, Rule, RuleKind};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MockConfig {
    pub rule: Vec<Rule>,
}

impl ConfigTrait for MockConfig {
    fn load() -> Self {
        // Return a dummy/mock configuration for testing
        MockConfig {
            rule: vec![
                Rule {
                    kind: RuleKind::AddDerive,
                    trait_name: vec!["Debug".to_string(), "PartialEq".to_string()],
                    apply_to: vec![],
                    condition: "true".to_string(),
                },
            ],
        }
    }

    fn get_rules(&self) -> &Vec<Rule> {
        &self.rule
    }
}
