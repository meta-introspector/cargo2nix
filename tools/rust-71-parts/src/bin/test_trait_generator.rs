//! Test trait generator with rust-bootstrap-nix patterns

use rust_71_parts::trait_generator_integration::MonsterTraitGenerator;
use rust_71_parts::declaration_splitter::MonsterDeclarationSplitter;

fn main() {
    println!("🏭 MONSTER TRAIT GENERATOR TEST");
    println!("===============================");
    
    let mut generator = MonsterTraitGenerator::new();
    let mut splitter = MonsterDeclarationSplitter::new();
    
    // Test with rust-bootstrap-nix monadic traits
    let test_code = r#"
        use async_trait::async_trait;

        #[async_trait]
        pub trait Functor<A, B> {
            type Output;
            async fn fmap(&self, f: impl Fn(A) -> B + Send + Sync + 'static) -> Self::Output;
        }

        #[async_trait]
        pub trait Monad<A, B>: Applicative<A, B> {
            type FlatMapOutput;
            async fn flat_map(&self, f: impl Fn(A) -> Self::FlatMapOutput + Send + Sync + 'static) -> Self::FlatMapOutput;
        }

        #[async_trait]
        pub trait Arrow<A, B> {
            type Output;
            async fn call(&self, input: A) -> Self::Output;
        }
    "#;
    
    println!("📝 Parsing rust-bootstrap-nix traits...");
    match splitter.split_file(test_code, Some("monadic_traits.rs".to_string())) {
        Ok(()) => {
            println!("✅ Parsed {} declarations", splitter.declarations.len());
            
            println!("\n🏭 Generating Monster Group traits...");
            let generated_traits = generator.generate_monster_traits(&splitter.declarations);
            
            println!("Generated {} Monster traits:", generated_traits.len());
            for (i, trait_code) in generated_traits.iter().enumerate() {
                println!("\n--- Monster Trait {} ---", i + 1);
                println!("{}", trait_code);
            }
            
            println!("\n📋 Generating registry code...");
            let registry_code = generator.generate_registry_code(&splitter.declarations);
            println!("{}", registry_code);
            
            println!("✅ TRAIT GENERATOR INTEGRATION SUCCESSFUL");
        }
        Err(e) => {
            println!("❌ Failed to parse traits: {}", e);
        }
    }
}
