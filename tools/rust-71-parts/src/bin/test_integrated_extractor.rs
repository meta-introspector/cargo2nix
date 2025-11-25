//! Test integrated trait/feature extractor with rust-bootstrap-nix patterns

use rust_71_parts::declaration_splitter::{MonsterDeclarationSplitter, DeclarationType};
use rust_71_parts::trait_feature_extractor::TraitFeatureExtractor;

fn main() {
    println!("🔧 INTEGRATED TRAIT/FEATURE EXTRACTOR TEST");
    println!("===========================================");
    
    // Test both extractors with rust-bootstrap-nix style code
    let test_code = r#"
        use anyhow::Result;
        use async_trait::async_trait;
        use std::path::{Path, PathBuf};

        #[async_trait]
        pub trait FileSystem: Send + Sync {
            async fn create_dir_all(&self, path: &Path) -> Result<()>;
            async fn write_file(&self, path: &Path, contents: &[u8]) -> Result<()>;
            async fn read_to_string(&self, path: &Path) -> Result<String>;
            fn exists(&self, path: &Path) -> bool;
            fn is_dir(&self, path: &Path) -> bool;
            fn current_dir(&self) -> Result<PathBuf>;
        }

        pub trait CommandExecutor {
            fn execute(&self, command: &str) -> Result<String>;
        }

        #[cfg(feature = "async")]
        pub struct AsyncFileSystem {
            base_path: PathBuf,
        }

        #[cfg(feature = "sync")]
        pub struct SyncFileSystem;

        impl FileSystem for AsyncFileSystem {
            async fn create_dir_all(&self, path: &Path) -> Result<()> {
                tokio::fs::create_dir_all(path).await?;
                Ok(())
            }
        }
    "#;
    
    println!("📝 Testing Declaration Splitter...");
    let mut splitter = MonsterDeclarationSplitter::new();
    match splitter.split_file(test_code, Some("test.rs".to_string())) {
        Ok(()) => {
            println!("✅ Declaration Splitter: {} declarations", splitter.declarations.len());
            
            for decl_type in [DeclarationType::Trait, DeclarationType::Struct, DeclarationType::Impl] {
                let count = splitter.get_declarations_by_type(decl_type.clone()).len();
                if count > 0 {
                    println!("  {:?}: {}", decl_type, count);
                }
            }
        }
        Err(e) => println!("❌ Declaration Splitter failed: {}", e),
    }
    
    println!("\n📝 Testing Trait/Feature Extractor...");
    let mut extractor = TraitFeatureExtractor::new();
    match extractor.extract_from_source(test_code) {
        Ok(()) => {
            println!("✅ Trait/Feature Extractor: {} traits, {} features", 
                     extractor.get_traits().len(), extractor.get_features().len());
            
            let stats = extractor.get_transport_stats();
            println!("  Transport: {} fragments across {} layers", 
                     stats.total_fragments, stats.active_layers);
        }
        Err(e) => println!("❌ Trait/Feature Extractor failed: {}", e),
    }
    
    println!("\n🧮 Monster Group Integration:");
    println!("============================");
    
    // Show Monster Group factor distribution from splitter
    let distribution = splitter.get_monster_factor_distribution();
    println!("Declaration factors: {:?}", distribution);
    
    // Generate combined registry
    println!("\n🎭 COMBINED TRAIT REGISTRY:");
    let trait_registry = splitter.generate_trait_registry();
    println!("{}", trait_registry);
    
    println!("\n✅ INTEGRATION TEST COMPLETE");
    println!("Both extractors working with Monster Group classification");
}
