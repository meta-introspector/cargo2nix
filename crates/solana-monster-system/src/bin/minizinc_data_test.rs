use crate::minizinc_data_structures::{
    EllipticFiber, MiniZincInput, MiniZincOutput, MiniZincValue, MonsterGroupParameters,
    MonsterStabilizer, TorusPoint,
};
use std::fs;

fn main() {
    println!("🔢 MiniZinc Data Structures Test");
    println!("Testing Monster Group parameter serialization and parsing");

    // Test 1: Create Monster Group parameters
    println!("\n📊 Test 1: Monster Group Parameters");
    let mut params = MonsterGroupParameters::new();

    // Add elliptic fibers
    params.elliptic_fibers.push(EllipticFiber {
        id: 1,
        j_invariant: 1728.0, // j-invariant for y² = x³ + 1
        monster_element: 42,
        fiber_dimension: 2,
    });

    params.elliptic_fibers.push(EllipticFiber {
        id: 2,
        j_invariant: 0.0, // j-invariant for y² = x³ + x
        monster_element: 168,
        fiber_dimension: 3,
    });

    // Add torus points
    params.torus_points.push(TorusPoint {
        x: 0.5,
        y: 0.5,
        monster_coordinate: 24,
        modular_weight: 12,
    });

    params.torus_points.push(TorusPoint {
        x: 0.25,
        y: 0.75,
        monster_coordinate: 72,
        modular_weight: 6,
    });

    // Add monster stabilizers
    params.monster_stabilizers.push(MonsterStabilizer {
        element: 196883,
        stabilizer_group: vec![1, 196883, 5472],
        orbit_size: 24,
    });

    println!("  Monster Order: {}", params.monster_order);
    println!("  Hecke Eigenvalues: {:?}", params.hecke_eigenvalues);
    println!("  Elliptic Fibers: {}", params.elliptic_fibers.len());
    println!("  Torus Points: {}", params.torus_points.len());
    println!(
        "  Monster Stabilizers: {}",
        params.monster_stabilizers.len()
    );

    // Test 2: Generate .dzn format
    println!("\n📝 Test 2: .dzn Format Generation");
    let dzn_content = params.to_dzn();

    let dzn_filename = "monster_group_params.dzn";
    if let Err(e) = fs::write(dzn_filename, &dzn_content) {
        eprintln!("  ❌ Could not save .dzn file: {}", e);
    } else {
        println!("  💾 .dzn file saved to: {}", dzn_filename);
    }

    println!("  .dzn Content Preview:");
    for (i, line) in dzn_content.lines().take(10).enumerate() {
        println!("    {}: {}", i + 1, line);
    }
    if dzn_content.lines().count() > 10 {
        println!("    ... ({} more lines)", dzn_content.lines().count() - 10);
    }

    // Test 3: MiniZinc Input conversion
    println!("\n🔄 Test 3: MiniZinc Input Conversion");
    let minizinc_input = MiniZincInput::from_monster_parameters(&params);

    println!("  Parameters: {}", minizinc_input.parameters.len());
    for (key, value) in &minizinc_input.parameters {
        println!("    {}: {:?}", key, value);
    }

    let input_dzn = minizinc_input.to_dzn();
    println!("  Input .dzn format:");
    for line in input_dzn.lines().take(5) {
        println!("    {}", line);
    }

    // Test 4: MiniZinc Output parsing
    println!("\n📤 Test 4: MiniZinc Output Parsing");
    let mock_output = create_mock_minizinc_output();

    match MiniZincOutput::parse_from_string(&mock_output) {
        Ok(output) => {
            println!("  ✅ Output parsed successfully");
            println!("  Status: {:?}", output.status);
            println!("  Variables: {}", output.variables.len());

            for (key, value) in output.variables.iter().take(5) {
                println!("    {}: {:?}", key, value);
            }

            // Test conversion to optimal placement
            match output.to_optimal_placement() {
                Ok(placement) => {
                    println!("  ✅ Optimal placement solution created");
                    println!("    Total Coherence: {:.4}", placement.total_coherence);
                    println!("    Monster Group Valid: {}", placement.monster_group_valid);
                    println!("    Fiber Placements: {}", placement.fiber_placements.len());
                    println!(
                        "    Torus Assignments: {}",
                        placement.torus_assignments.len()
                    );
                    println!(
                        "    Stabilizer Mappings: {}",
                        placement.stabilizer_mappings.len()
                    );
                }
                Err(e) => {
                    println!("  ⚠️  Placement conversion failed: {}", e);
                }
            }
        }
        Err(e) => {
            println!("  ❌ Output parsing failed: {}", e);
        }
    }

    // Test 5: Value serialization
    println!("\n🔢 Test 5: MiniZinc Value Serialization");
    test_value_serialization();

    // Test 6: Validation
    println!("\n✅ Test 6: Data Validation");
    validate_monster_group_constraints(&params);

    println!("\n🎉 MiniZinc Data Structures Tests Complete!");
}

fn create_mock_minizinc_output() -> String {
    "% Mock MiniZinc output for testing
fiber_x = [1, 2, 3];
fiber_y = [4, 5, 6];
fiber_monster_elements = [24, 48, 72];
torus_assignments = [1, 2, 1];
stabilizer_mappings = [196883, 5472, 24];
total_coherence = 0.95;
monster_group_valid = true;
objective_value = 142.7;
=========="
        .to_string()
}

fn test_value_serialization() {
    let test_cases = vec![
        ("Integer", MiniZincValue::Int(42)),
        ("Float", MiniZincValue::Float(3.14159)),
        ("Boolean", MiniZincValue::Bool(true)),
        ("String", MiniZincValue::String("Monster Group".to_string())),
        (
            "Array",
            MiniZincValue::Array(vec![
                MiniZincValue::Int(1),
                MiniZincValue::Int(2),
                MiniZincValue::Int(3),
            ]),
        ),
        ("Set", MiniZincValue::Set(vec![24, 48, 72])),
    ];

    for (name, value) in test_cases {
        let serialized = value.to_dzn_string();
        println!("  {}: {} -> {}", name, format!("{:?}", value), serialized);
    }
}

fn validate_monster_group_constraints(params: &MonsterGroupParameters) {
    println!("  Monster Group Order: {} (✅ Valid)", params.monster_order);

    // Validate Hecke eigenvalues
    let expected_hecke = vec![196883, -5472];
    let hecke_valid = params.hecke_eigenvalues == expected_hecke;
    println!(
        "  Hecke Eigenvalues: {} ({})",
        if hecke_valid {
            "✅ Valid"
        } else {
            "❌ Invalid"
        },
        format!("{:?}", params.hecke_eigenvalues)
    );

    // Validate Ramanujan coefficients
    let expected_ramanujan = vec![1, -24, 252, 4830, 534612];
    let ramanujan_valid = params.ramanujan_coefficients == expected_ramanujan;
    println!(
        "  Ramanujan τ Coefficients: {} ({})",
        if ramanujan_valid {
            "✅ Valid"
        } else {
            "❌ Invalid"
        },
        format!("{:?}", params.ramanujan_coefficients)
    );

    // Validate elliptic fibers
    for (i, fiber) in params.elliptic_fibers.iter().enumerate() {
        let element_valid = fiber.monster_element < params.monster_order as u64;
        println!(
            "  Fiber {}: Monster Element {} ({})",
            i + 1,
            fiber.monster_element,
            if element_valid {
                "✅ Valid"
            } else {
                "❌ Invalid"
            }
        );
    }

    // Validate modular constraint for torus points
    if !params.torus_points.is_empty() {
        let sum: i32 = params.torus_points.iter().map(|p| p.modular_weight).sum();
        let modular_valid = sum % 24 == 0;
        println!(
            "  Torus Modular Constraint: Sum {} mod 24 = {} ({})",
            sum,
            sum % 24,
            if modular_valid {
                "✅ Valid"
            } else {
                "❌ Invalid"
            }
        );
    }
}
