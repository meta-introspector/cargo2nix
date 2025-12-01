// crates/trait-fixer-rules-trait/src/lib.rs

use serde::Deserialize;

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
    pub trait_name: Vec<String>,
    #[serde(default)]
    pub apply_to: Vec<String>,
    pub condition: String,
}

pub trait ConfigTrait {
    fn load() -> Self;
    fn get_rules(&self) -> &Vec<Rule>;
}
