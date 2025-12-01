use solana_monster_system::core_constants::MONSTER_GROUP_REPRESENTATION_DIMENSION;
use solana_monster_system::MetaMemeSporeSystem;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let population_size = if args.len() > 1 {
        args[1].parse().unwrap_or(20)
    } else {
        20
    };

    let generations = if args.len() > 2 {
        args[2].parse().unwrap_or(10)
    } else {
        10
    };

    println!("🧬 Monster Group Meta-Meme Spore System");
    println!(
        "Population: {}, Generations: {}",
        population_size, generations
    );
    println!("Resource Constraints: 6KB RAM, AWS Free Tier");

    let mut system = MetaMemeSporeSystem::new();
    system.initialize_population(population_size);

    println!("\n📊 Initial Population Statistics:");
    print_population_stats(&system);

    // Evolution loop
    for gen in 0..generations {
        system.evaluate_fitness();

        if gen % 5 == 0 {
            println!("\n🧬 Generation {}: ", gen);
            print_top_spores(&system, 3);
        }

        system.genetic_evolution();
    }

    // Final evaluation
    system.evaluate_fitness();

    println!("\n🏆 Final Results (Generation {}):", system.generation);
    print_top_spores(&system, 5);

    // Generate MiniZinc model
    let minizinc_model = system.optimize_minizinc_constraints();

    println!("\n📝 Generated MiniZinc Model:");
    println!("{}", minizinc_model);

    // Save to file
    if let Err(e) = fs::write("meta_meme_spore_optimization.mzn", &minizinc_model) {
        eprintln!("Warning: Could not save MiniZinc model: {}", e);
    } else {
        println!("\n💾 MiniZinc model saved to: meta_meme_spore_optimization.mzn");
    }

    // Generate optimization report
    generate_optimization_report(&system);
}

fn print_population_stats(system: &MetaMemeSporeSystem) {
    let total_spores = system.spores.len();
    let total_ram = system
        .spores
        .iter()
        .map(|s| s.resource_allocation.ram_bytes as u32)
        .sum::<u32>();
    let total_tokens = system
        .spores
        .iter()
        .map(|s| s.meme_tokens.len())
        .sum::<usize>();

    println!("  Total Spores: {}", total_spores);
    println!("  Total RAM Usage: {} bytes", total_ram);
    println!("  Total Meme Tokens: {}", total_tokens);
    println!("  Generation: {}", system.generation);
}

fn print_top_spores(system: &MetaMemeSporeSystem, count: usize) {
    let mut sorted_spores = system.spores.clone();
    sorted_spores.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

    for (i, spore) in sorted_spores.iter().take(count).enumerate() {
        println!(
            "  #{}: Fitness: {:.4}, Monster Element: {}, Gödel: {}, Tokens: {}",
            i + 1,
            spore.fitness,
            spore.monster_element,
            spore.godel_number,
            spore.meme_tokens.len()
        );
    }
}

fn generate_optimization_report(system: &MetaMemeSporeSystem) {
    let mut sorted_spores = system.spores.clone();
    sorted_spores.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

    let best_spore = &sorted_spores[0];
    let avg_fitness =
        system.spores.iter().map(|s| s.fitness).sum::<f64>() / system.spores.len() as f64;

    let report = format!(
        "# Meta-Meme Spore Optimization Report\n\
        \n\
        ## System Configuration\n\
        - Population Size: {}\n\
        - Generations: {}\n\
        - Resource Constraints: 6KB RAM, AWS Free Tier\n\
        \n\
        ## Optimization Results\n\
        - Best Fitness: {:.6}\n\
        - Average Fitness: {:.6}\n\
        - Best Monster Element: {}\n\
        - Best Gödel Number: {}\n\
        - Best Primorial Dimension: {}\n\
        \n\
        ## Resource Utilization\n\
        - RAM per Spore: {} bytes\n\
        - Total Meme Tokens: {}\n\
        \n\
        ## Monster Group Properties\n\
        - Monster Group Order: {}\n\
        - Ramanujan τ Constraint: sum ≡ 0 (mod 24)\n\
        - Hecke Eigenvalues: ±196883, ±5472\n\
        \n\
        ## Lisp Expressions (Top 3)\n\
        {}",
        system.spores.len(),
        system.generation,
        best_spore.fitness,
        avg_fitness,
        best_spore.monster_element,
        best_spore.godel_number,
        best_spore.primorial_dimension,
        best_spore.resource_allocation.ram_bytes,
        best_spore.meme_tokens.len(),
        MONSTER_GROUP_REPRESENTATION_DIMENSION,
        best_spore
            .meme_tokens
            .iter()
            .take(3)
            .enumerate()
            .map(|(i, token)| format!(
                "{}. {} (Value: {:.3})",
                i + 1,
                token.lisp_expression,
                token.value
            ))
            .collect::<Vec<_>>()
            .join("\n")
    );

    if let Err(e) = fs::write("meta_meme_spore_report.md", &report) {
        eprintln!("Warning: Could not save report: {}", e);
    } else {
        println!("📋 Optimization report saved to: meta_meme_spore_report.md");
    }

    println!("\n💰 Monetization Metrics:");
    println!(
        "  Estimated Money Generation: ${:.2}",
        best_spore.fitness * 100.0
    );
    println!(
        "  Meme Token Value: ${:.2}",
        best_spore.meme_tokens.iter().map(|t| t.value).sum::<f64>()
    );
    println!(
        "  Monster Group Coherence: {:.1}%",
        (best_spore.monster_element as f64 / MONSTER_GROUP_REPRESENTATION_DIMENSION as f64) * 100.0
    );
}
