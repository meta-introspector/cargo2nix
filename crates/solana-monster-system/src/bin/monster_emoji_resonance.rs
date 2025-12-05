use cargo2nix::MonsterEmojiOptimizer;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: monster_emoji_resonance <emoji1> [emoji2] ...");
        println!("Example: monster_emoji_resonance 👹 🌙 ✨ 🔢 🎯");
        return;
    }

    let emojis: Vec<String> = args[1..].iter().cloned().collect();

    println!("# The Monster Group Optimizes Emoji-Prime Resonance");
    println!(
        "# Applying Monstrous Moonshine to {} emojis\n",
        emojis.len()
    );

    let mappings = MonsterEmojiOptimizer::optimize_resonance(emojis);

    for mapping in &mappings {
        println!(
            "Emoji: {}, Monster Element: {}, Prime: {}, Hecke: {}, Resonance: {:?}",
            mapping.emoji,
            mapping.monster_element,
            mapping.prime_resonance,
            mapping.hecke_value,
            mapping.resonance_level
        );
    }

    // Verify Monster Group constraint
    let sum: u64 = mappings.iter().map(|m| m.monster_element).sum();
    let modular_check = sum % 24 == 0;

    println!(
        "\nMonster Group Constraint: Σ elements ≡ 0 (mod 24) {}",
        if modular_check { "✓" } else { "✗" }
    );

    let total_resonance: u64 = mappings.iter().map(|m| m.prime_resonance).sum();
    println!("Total Monstrous Resonance: {}", total_resonance);

    if modular_check {
        println!("\n🎉 Perfect Monstrous Moonshine achieved! 🌙👹");
    }
}
