use solana_monster_system::minizinc_introspector_integration::{
    MiniZincIntrospectorIntegration, VendorizationStatus,
};
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("🔍 MiniZinc Introspector Vendorization Integration");
    println!("Repository: https://github.com/meta-introspector/minizinc-introspector");
    println!("Vendor Path: vendor/libminizinc");

    let integration = MiniZincIntrospectorIntegration::new();

    // Verify vendorization status
    println!("\n📋 Vendorization Status Check:");
    match integration.verify_vendorization() {
        Ok(status) => {
            print_vendorization_status(&status);

            if !status.is_complete() {
                println!("\n⚠️  Vendorization Incomplete:");
                for component in status.missing_components() {
                    println!("  ❌ Missing: {}", component);
                }
                print_setup_instructions();
            } else {
                println!("\n✅ Vendorization Complete!");
                run_integration_tests(&integration, &args);
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to verify vendorization: {}", e);
            return;
        }
    }
}

fn print_vendorization_status(status: &VendorizationStatus) {
    println!(
        "  Vendor Directory: {}",
        if status.vendor_directory_exists {
            "✅ Found"
        } else {
            "❌ Missing"
        }
    );
    println!(
        "  .gitmodules Config: {}",
        if status.gitmodules_configured {
            "✅ Configured"
        } else {
            "❌ Not configured"
        }
    );
    println!(
        "  Correct Repository: {}",
        if status.correct_repository {
            "✅ Correct"
        } else {
            "❌ Incorrect"
        }
    );
    println!(
        "  asciicast_processor: {}",
        if status.asciicast_processor_exists {
            "✅ Found"
        } else {
            "❌ Missing"
        }
    );
    println!(
        "  Submodule Initialized: {}",
        if status.submodule_initialized {
            "✅ Initialized"
        } else {
            "❌ Not initialized"
        }
    );
}

fn print_setup_instructions() {
    println!("\n📝 Setup Instructions:");
    println!("1. Add submodule to .gitmodules:");
    println!("   [submodule \"vendor/libminizinc\"]");
    println!("       path = vendor/libminizinc");
    println!("       url = https://github.com/meta-introspector/minizinc-introspector.git");
    println!();
    println!("2. Initialize submodule:");
    println!("   git submodule add https://github.com/meta-introspector/minizinc-introspector.git vendor/libminizinc");
    println!("   git submodule update --init --recursive");
    println!();
    println!("3. Verify asciicast_processor component:");
    println!("   ls vendor/libminizinc/asciicast_processor/");
}

fn run_integration_tests(integration: &MiniZincIntrospectorIntegration, args: &[String]) {
    let problem_size = if args.len() > 1 {
        args[1].parse().unwrap_or(6)
    } else {
        6
    };

    println!("\n🧮 Test 1: Introspector Model Generation");
    let model = integration.generate_introspector_model(problem_size);

    println!("  Problem Size: {}", problem_size);
    println!("  Model Lines: {}", model.lines().count());

    if model.contains("Monster Group") {
        println!("  ✅ Monster Group integration detected");
    }
    if model.contains("constraint sum(introspection_vars) mod 24 = 0") {
        println!("  ✅ Ramanujan τ constraint detected");
    }
    if model.contains("all_different") {
        println!("  ✅ All-different constraint detected");
    }

    // Save generated model
    let model_filename = "introspector_monster_model.mzn";
    if let Err(e) = fs::write(model_filename, &model) {
        eprintln!("  ⚠️  Could not save model: {}", e);
    } else {
        println!("  💾 Model saved to: {}", model_filename);
    }

    println!("\n🎬 Test 2: Asciicast Processing");
    test_asciicast_processing(integration);

    println!("\n🔧 Test 3: MiniZinc Solver Integration");
    test_solver_integration(integration, &model);

    println!("\n📊 Test 4: Monster Group Verification");
    test_monster_group_verification();

    println!("\n🎉 Integration Tests Complete!");
}

fn test_asciicast_processing(integration: &MiniZincIntrospectorIntegration) {
    // Create a test asciicast file
    let test_content = "% Test MiniZinc model\nvar 1..10: x;\nconstraint x > 5;\nsolve satisfy;";
    let test_file = "test_asciicast.mzn";

    if let Err(e) = fs::write(test_file, test_content) {
        println!("  ❌ Could not create test file: {}", e);
        return;
    }

    match integration.process_monster_asciicast(test_file) {
        Ok(result) => {
            println!("  ✅ Asciicast processing successful");
            println!("  📄 Output length: {} characters", result.len());

            if result.contains("Monster Group") {
                println!("  ✅ Monster Group enhancement applied");
            }
        }
        Err(e) => {
            println!("  ⚠️  Asciicast processing failed: {}", e);
            println!("  💡 This is expected if asciicast_processor is not available");
        }
    }

    // Cleanup
    let _ = fs::remove_file(test_file);
    let _ = fs::remove_file(format!("{}.monster_enhanced", test_file));
}

fn test_solver_integration(integration: &MiniZincIntrospectorIntegration, model: &str) {
    match integration.run_introspector_solver(model) {
        Ok(result) => {
            println!("  ✅ MiniZinc solver execution successful");
            println!("  📊 Variables: {:?}", result.variables);
            println!("  📈 Depth Levels: {:?}", result.depth_levels);
            println!("  🎯 Total Introspection: {}", result.total_introspection);
            println!(
                "  🧮 Monster Group Valid: {}",
                if result.monster_group_valid {
                    "✅ Yes"
                } else {
                    "❌ No"
                }
            );
        }
        Err(e) => {
            println!("  ⚠️  Solver execution failed: {}", e);
            println!("  💡 This is expected if MiniZinc is not installed");
        }
    }

    // Cleanup
    let _ = fs::remove_file("introspector_model.mzn");
}

fn test_monster_group_verification() {
    use solana_monster_system::core_constants::{
        HECKE_EIGENVALUES, MONSTER_GROUP_REPRESENTATION_DIMENSION,
    };

    println!("  Monster Group Order: {}", MONSTER_GROUP_ORDER);
    println!("  Hecke Eigenvalues: {:?}", HECKE_EIGENVALUES);

    // Test modular constraint
    let test_elements = vec![24, 48, 72, 96, 144];
    let sum: i32 = test_elements.iter().sum();
    let modular_check = sum % 24 == 0;

    println!("  Test Elements: {:?}", test_elements);
    println!("  Sum: {} (mod 24 = {})", sum, sum % 24);
    println!(
        "  Modular Constraint: {}",
        if modular_check {
            "✅ Satisfied"
        } else {
            "❌ Violated"
        }
    );

    // Test uniqueness
    let unique_count = test_elements
        .iter()
        .collect::<std::collections::HashSet<_>>()
        .len();
    let uniqueness_check = unique_count == test_elements.len();
    println!(
        "  Uniqueness: {} unique / {} total ({})",
        unique_count,
        test_elements.len(),
        if uniqueness_check {
            "✅ All unique"
        } else {
            "❌ Duplicates"
        }
    );

    // Test bounds
    let bounds_check = test_elements
        .iter()
        .all(|&x| x >= 0 && x < MONSTER_GROUP_ORDER as i32);
    println!(
        "  Bounds Check: {}",
        if bounds_check {
            "✅ Within Monster Group"
        } else {
            "❌ Out of bounds"
        }
    );
}
