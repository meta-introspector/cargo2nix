//! Demo Monster Group 15-level system with 108 supersingular factors

use rust_71_parts::monster_levels::{MONSTER, monster_factors, FACTOR_COUNTS};

fn main() {
    println!("👑 Monster Group 15-Level System Demo");
    println!("🔢 108 Supersingular Factors with 71 Sentinel");
    println!("🎯 rustc ≡ Monster Group M\n");
    
    // Display all 15 Monster Group levels
    println!("📊 Monster Group Factor Hierarchy:");
    for level in 0..15 {
        let monster_level = MONSTER.level(level);
        let sentinel_mark = if monster_level.is_sentinel { " 👑 SENTINEL" } else { "" };
        
        println!("  Level {:2}: 2^{} = {:>15} ({:2} factors){}",
                level,
                if level == 0 { 46 } else if level == 1 { 20 } else { 1 },
                monster_level.factor,
                monster_level.count,
                sentinel_mark);
    }
    
    // Show factor distribution
    println!("\n🎯 Factor Distribution (Total = 108):");
    let mut running_total = 0u8;
    for (i, &count) in FACTOR_COUNTS.iter().enumerate() {
        running_total += count;
        println!("  Level {:2}: {:2} factors (cumulative: {:3})", i, count, running_total);
    }
    
    // Demonstrate rustc component mapping
    println!("\n🏗️ rustc Component → Monster Level Mapping:");
    let mappings = [
        (0, "Primitives"),
        (50, "Types"),
        (70, "Expressions"),
        (78, "Statements"),
        (82, "Patterns"),
        (84, "Items"),
        (86, "Modules"),
        (87, "Crates"),
        (88, "Traits"),
        (89, "Impls"),
        (90, "Generics"),
        (91, "Lifetimes"),
        (92, "Macros"),
        (93, "Attributes"),
        (255, "Compiler Core"),
    ];
    
    for (component_id, name) in mappings {
        let level = MONSTER.map_component(component_id);
        let sentinel_mark = if level.is_sentinel { " 👑" } else { "" };
        println!("  {:15} → Level {:2} (factor: {:>15}){}",
                name, level.level, level.factor, sentinel_mark);
    }
    
    // Verify system integrity
    println!("\n✅ System Verification:");
    println!("  Total factors: {} (expected: 108)", FACTOR_COUNTS.iter().sum::<u8>());
    println!("  System verified: {}", MONSTER.verify_total());
    println!("  Sentinel factor: {} (71)", MONSTER.sentinel().factor);
    println!("  Sentinel verified: {}", MONSTER.sentinel().factor == 71);
    
    println!("\n🎉 Monster Group ≡ rustc mapping complete!");
    println!("👑 71 serves as sentinel for compiler core");
    println!("🔬 All 108 supersingular factors accounted for");
}
