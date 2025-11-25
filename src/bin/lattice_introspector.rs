use cargo2nix::lattice_introspector::{LatticeIntrospector, ConstraintType};
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let lattice_size = if args.len() > 1 {
        args[1].parse().unwrap_or(12)
    } else {
        12
    };
    
    let introspection_rounds = if args.len() > 2 {
        args[2].parse().unwrap_or(5)
    } else {
        5
    };
    
    println!("🔍 Lattice Introspector: MiniZinc Integration");
    println!("Lattice Size: {}, Introspection Rounds: {}", lattice_size, introspection_rounds);
    println!("Monster Group Order: {}", cargo2nix::MONSTER_GROUP_ORDER);
    
    let mut introspector = LatticeIntrospector::new();
    introspector.initialize_lattice(lattice_size);
    
    println!("\n📊 Initial Lattice Configuration:");
    print_lattice_stats(&introspector);
    
    println!("\n🧠 Constraint Analysis:");
    analyze_constraints(&introspector);
    
    // Introspection rounds
    for round in 1..=introspection_rounds {
        println!("\n🔍 Introspection Round {}:", round);
        
        let result = introspector.introspect();
        
        println!("  Level: {}", result.level);
        println!("  Lattice Coherence: {:.4}", result.lattice_coherence);
        println!("  Constraint Satisfaction: {:.4}", result.constraint_satisfaction);
        println!("  Monster Alignment: {:.4}", result.monster_alignment);
        println!("  Optimization Potential: {:.4}", result.optimization_potential);
        
        if !result.recommendations.is_empty() {
            println!("  📋 Recommendations:");
            for (i, rec) in result.recommendations.iter().enumerate() {
                println!("    {}. {}", i + 1, rec);
            }
        }
        
        if round % 2 == 0 {
            println!("  🎯 Intermediate Analysis:");
            analyze_lattice_structure(&introspector);
        }
    }
    
    // Generate MiniZinc model
    println!("\n📝 Generating MiniZinc Model...");
    let minizinc_model = introspector.generate_minizinc_model();
    
    // Save MiniZinc model
    let model_filename = "lattice_introspection.mzn";
    if let Err(e) = fs::write(model_filename, &minizinc_model) {
        eprintln!("Warning: Could not save MiniZinc model: {}", e);
    } else {
        println!("💾 MiniZinc model saved to: {}", model_filename);
    }
    
    // Generate introspection report
    generate_introspection_report(&introspector, introspection_rounds);
    
    println!("\n🎉 Lattice Introspection Complete!");
    println!("Final Introspection Level: {}", introspector.introspection_level);
}

fn print_lattice_stats(introspector: &LatticeIntrospector) {
    let total_nodes = introspector.nodes.len();
    let total_connections: usize = introspector.nodes.iter()
        .map(|n| n.connections.len())
        .sum();
    let avg_depth = introspector.nodes.iter()
        .map(|n| n.introspection_depth as f64)
        .sum::<f64>() / total_nodes as f64;
    
    println!("  Total Nodes: {}", total_nodes);
    println!("  Total Connections: {}", total_connections);
    println!("  Average Introspection Depth: {:.2}", avg_depth);
    println!("  Constraints Defined: {}", introspector.constraints.len());
}

fn analyze_constraints(introspector: &LatticeIntrospector) {
    for (i, constraint) in introspector.constraints.iter().enumerate() {
        let constraint_type_str = match constraint.constraint_type {
            ConstraintType::AllDifferent => "All Different",
            ConstraintType::LinearSum => "Linear Sum",
            ConstraintType::MonsterGroupMod => "Monster Group Modular",
            ConstraintType::LatticeConnectivity => "Lattice Connectivity",
            ConstraintType::IntrospectionDepth => "Introspection Depth",
        };
        
        println!("  {}. {} ({})", 
                 i + 1, 
                 constraint.name, 
                 constraint_type_str);
        println!("     Variables: {}, Bounds: {:?}, Alignment: {:.2}", 
                 constraint.variables.len(),
                 constraint.bounds,
                 constraint.monster_alignment);
    }
}

fn analyze_lattice_structure(introspector: &LatticeIntrospector) {
    let monster_elements: Vec<u64> = introspector.nodes.iter()
        .map(|n| n.monster_element)
        .collect();
    
    let sum: u64 = monster_elements.iter().sum();
    let modular_check = sum % 24 == 0;
    
    let unique_elements: std::collections::HashSet<_> = monster_elements.iter().collect();
    let uniqueness_ratio = unique_elements.len() as f64 / monster_elements.len() as f64;
    
    println!("    Monster Group Sum: {} (mod 24 = {})", sum, sum % 24);
    println!("    Modular Constraint: {}", if modular_check { "✓ Satisfied" } else { "✗ Violated" });
    println!("    Uniqueness Ratio: {:.2}%", uniqueness_ratio * 100.0);
    
    let max_connections = introspector.nodes.iter()
        .map(|n| n.connections.len())
        .max()
        .unwrap_or(0);
    let min_connections = introspector.nodes.iter()
        .map(|n| n.connections.len())
        .min()
        .unwrap_or(0);
    
    println!("    Connection Range: {} - {}", min_connections, max_connections);
}

fn generate_introspection_report(introspector: &LatticeIntrospector, rounds: i32) {
    let final_result = introspector.introspect();
    
    let report = format!(
        "# Lattice Introspection Report\n\
        \n\
        ## Configuration\n\
        - Lattice Size: {} nodes\n\
        - Introspection Rounds: {}\n\
        - Final Introspection Level: {}\n\
        - Monster Group Order: {}\n\
        \n\
        ## Final Metrics\n\
        - Lattice Coherence: {:.6}\n\
        - Constraint Satisfaction: {:.6}\n\
        - Monster Alignment: {:.6}\n\
        - Optimization Potential: {:.6}\n\
        \n\
        ## Constraint Analysis\n\
        {}
        \n\
        ## MiniZinc Applications\n\
        - **Combinatorial Optimization**: Lattice node assignment with Monster Group constraints\n\
        - **Resource Allocation**: Introspection depth distribution across lattice\n\
        - **Scheduling**: Connection-based task dependencies with mathematical grounding\n\
        - **Declarative Modeling**: Constraint programming for complex lattice problems\n\
        \n\
        ## Recommendations\n\
        {}
        \n\
        ## Monster Group Properties\n\
        - Order: {}\n\
        - Hecke Eigenvalues: {:?}\n\
        - Ramanujan τ Constraint: sum ≡ 0 (mod 24)\n\
        - Monstrous Moonshine Connection: j-invariant lattice structure",
        introspector.nodes.len(),
        rounds,
        introspector.introspection_level,
        cargo2nix::MONSTER_GROUP_ORDER,
        final_result.lattice_coherence,
        final_result.constraint_satisfaction,
        final_result.monster_alignment,
        final_result.optimization_potential,
        introspector.constraints.iter()
            .enumerate()
            .map(|(i, c)| format!("{}. {} (Alignment: {:.2})", i + 1, c.name, c.monster_alignment))
            .collect::<Vec<_>>()
            .join("\n"),
        final_result.recommendations.iter()
            .enumerate()
            .map(|(i, r)| format!("{}. {}", i + 1, r))
            .collect::<Vec<_>>()
            .join("\n"),
        cargo2nix::MONSTER_GROUP_ORDER,
        cargo2nix::HECKE_EIGENVALUES
    );
    
    let report_filename = "lattice_introspection_report.md";
    if let Err(e) = fs::write(report_filename, &report) {
        eprintln!("Warning: Could not save report: {}", e);
    } else {
        println!("📋 Introspection report saved to: {}", report_filename);
    }
    
    println!("\n🔍 MiniZinc Integration Insights:");
    println!("  • Declarative constraint modeling for lattice optimization");
    println!("  • Monster Group mathematical foundation for search space");
    println!("  • Efficient solver integration for combinatorial problems");
    println!("  • Scalable introspection with formal verification");
}
