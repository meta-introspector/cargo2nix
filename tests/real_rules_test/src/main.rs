// tests/real_rules_test/src/main.rs

use trait_fixer_rules_real::Config as RealConfig;
use trait_fixer_rules_trait::ConfigTrait; // Alias to avoid conflict

fn main() {
    println!("--- Running real rules test ---");

    // Load the real configuration
    let config = RealConfig::load();

    // Assert that rules are loaded
    assert!(
        !config.get_rules().is_empty(),
        "Expected some rules to be loaded"
    );

    println!("Loaded {} rules.", config.get_rules().len());

    println!("Real rules test passed!\n");
}
