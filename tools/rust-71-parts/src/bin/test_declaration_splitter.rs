//! Simple test for Declaration Splitter integration

use rust_71_parts::declaration_splitter::{MonsterDeclarationSplitter, DeclarationType};

fn main() {
    println!("🧪 TESTING DECLARATION SPLITTER INTEGRATION");
    println!("============================================");
    
    let mut splitter = MonsterDeclarationSplitter::new();
    
    // Test with simple Rust code
    let test_code = r#"
        trait TestTrait {
            fn test_method(&self);
        }
        
        struct TestStruct {
            field: i32,
        }
        
        enum TestEnum {
            Variant1,
            Variant2(i32),
        }
        
        fn test_function() -> i32 {
            42
        }
        
        impl TestTrait for TestStruct {
            fn test_method(&self) {
                println!("test");
            }
        }
    "#;
    
    println!("📝 Processing test code...");
    match splitter.split_file(test_code, Some("test.rs".to_string())) {
        Ok(()) => {
            println!("✅ Successfully parsed test code");
            
            println!("\n📊 RESULTS:");
            println!("Total declarations: {}", splitter.declarations.len());
            
            // Show breakdown by type
            for decl_type in [
                DeclarationType::Function,
                DeclarationType::Struct, 
                DeclarationType::Enum,
                DeclarationType::Trait,
                DeclarationType::Impl,
            ] {
                let count = splitter.get_declarations_by_type(decl_type.clone()).len();
                println!("{:?}: {}", decl_type, count);
            }
            
            // Show Monster Group factor distribution
            println!("\n🧮 MONSTER FACTOR DISTRIBUTION:");
            let distribution = splitter.get_monster_factor_distribution();
            for (factor, count) in distribution {
                println!("Factor {}: {} declarations", factor, count);
            }
            
            // Show all declarations
            println!("\n🔬 ALL DECLARATIONS:");
            for (i, decl) in splitter.declarations.iter().enumerate() {
                println!("{}. {} ({:?}) - Factor: {}, Layer: {}", 
                         i + 1, decl.name, decl.declaration_type, 
                         decl.monster_factor, decl.transport_layer);
            }
            
            // Generate trait registry
            println!("\n🎭 TRAIT REGISTRY:");
            let trait_registry = splitter.generate_trait_registry();
            println!("{}", trait_registry);
            
            println!("\n✅ INTEGRATION TEST SUCCESSFUL");
            println!("Declaration splitter working with Monster Group transport");
        }
        Err(e) => {
            println!("❌ Failed to parse test code: {}", e);
        }
    }
}
