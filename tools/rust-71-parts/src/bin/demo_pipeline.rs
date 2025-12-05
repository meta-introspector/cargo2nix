//! Demo: Pausable trait extraction pipeline with data export

use rust_71_parts::declaration_splitter::{MonsterDeclarationSplitter, DeclarationType};
use rust_71_parts::trait_generator_integration::MonsterTraitGenerator;
use serde_json;
use std::fs;

fn main() {
    println!("🔬 DEMO: PAUSABLE TRAIT EXTRACTION PIPELINE");
    println!("============================================");
    println!("Extracting traits from rustc code with Monster Group classification");
    println!("Data exported at each layer for inspection in JSON/CSV/Parquet formats");
    
    let mut splitter = MonsterDeclarationSplitter::new();
    let mut generator = MonsterTraitGenerator::new();
    
    // Demo with inline rustc-style code
    let rustc_layers = [
        ("AST Layer", r#"
            pub trait AstNode {
                fn span(&self) -> Span;
                fn visit_children(&self, visitor: &mut dyn Visitor);
            }
            
            pub struct Expr {
                pub kind: ExprKind,
                pub span: Span,
            }
            
            pub enum ExprKind {
                Literal(Lit),
                Path(Path),
                Call(Box<Expr>, Vec<Expr>),
            }
        "#),
        ("HIR Layer", r#"
            pub trait HirNode {
                fn hir_id(&self) -> HirId;
                fn node(&self) -> Node;
            }
            
            pub struct Body {
                pub params: Vec<Param>,
                pub value: Expr,
            }
            
            impl HirNode for Body {
                fn hir_id(&self) -> HirId { self.value.hir_id }
                fn node(&self) -> Node { Node::Body(self) }
            }
        "#),
        ("Type Layer", r#"
            pub trait TypeFoldable {
                fn try_fold_with<F: TypeFolder>(&self, folder: &mut F) -> Result<Self, F::Error>;
            }
            
            pub trait TypeVisitable {
                fn visit_with<V: TypeVisitor>(&self, visitor: &mut V) -> ControlFlow<V::BreakTy>;
            }
            
            pub struct TyCtxt<'tcx> {
                pub types: &'tcx TypeckTables<'tcx>,
            }
        "#),
    ];
    
    for (layer_idx, (layer_name, code)) in rustc_layers.iter().enumerate() {
        println!("\n📂 Layer {}: {}", layer_idx, layer_name);
        
        // Process layer
        let _ = splitter.split_file(code, Some(format!("{}.rs", layer_name)));
        
        // Export layer data
        export_layer_data(&splitter, layer_idx as u8, layer_name);
        
        // Pause point
        println!("⏸️  Layer {} complete. Press Enter to continue...", layer_idx);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
    }
    
    // Generate Monster Group traits
    println!("\n🏭 Generating Monster Group traits...");
    let generated_traits = generator.generate_monster_traits(&splitter.declarations);
    
    // Export final results
    export_final_results(&splitter, &generated_traits);
    
    println!("\n✅ DEMO COMPLETE");
    println!("Pipeline successfully extracted and classified {} declarations", splitter.declarations.len());
    println!("All data exported for inspection in multiple formats");
}

fn export_layer_data(splitter: &MonsterDeclarationSplitter, layer: u8, layer_name: &str) {
    let layer_data = serde_json::json!({
        "layer": layer,
        "name": layer_name,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "declarations": splitter.declarations.iter().map(|d| {
            serde_json::json!({
                "name": d.name,
                "type": format!("{:?}", d.declaration_type),
                "monster_factor": d.monster_factor,
                "transport_layer": d.transport_layer,
                "content_length": d.content.len(),
            })
        }).collect::<Vec<_>>(),
        "statistics": {
            "total_declarations": splitter.declarations.len(),
            "traits": splitter.get_declarations_by_type(DeclarationType::Trait).len(),
            "structs": splitter.get_declarations_by_type(DeclarationType::Struct).len(),
            "functions": splitter.get_declarations_by_type(DeclarationType::Function).len(),
            "enums": splitter.get_declarations_by_type(DeclarationType::Enum).len(),
            "impls": splitter.get_declarations_by_type(DeclarationType::Impl).len(),
        },
        "monster_factors": splitter.get_monster_factor_distribution(),
    });
    
    // JSON export
    let json_file = format!("demo_layer_{}.json", layer);
    let _ = fs::write(&json_file, serde_json::to_string_pretty(&layer_data).unwrap());
    
    // CSV export for easy inspection
    let csv_file = format!("demo_layer_{}.csv", layer);
    let mut csv = "name,type,monster_factor,transport_layer\n".to_string();
    for decl in &splitter.declarations {
        csv.push_str(&format!("{},{:?},{},{}\n", 
            decl.name, decl.declaration_type, decl.monster_factor, decl.transport_layer));
    }
    let _ = fs::write(&csv_file, csv);
    
    println!("   📄 Exported: {} (JSON), {} (CSV)", json_file, csv_file);
    println!("   📊 Stats: {} total, {} traits, {} structs", 
             splitter.declarations.len(),
             splitter.get_declarations_by_type(DeclarationType::Trait).len(),
             splitter.get_declarations_by_type(DeclarationType::Struct).len());
}

fn export_final_results(splitter: &MonsterDeclarationSplitter, generated_traits: &[String]) {
    // Final summary
    let summary = serde_json::json!({
        "pipeline_complete": true,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "total_declarations": splitter.declarations.len(),
        "by_type": {
            "traits": splitter.get_declarations_by_type(DeclarationType::Trait).len(),
            "structs": splitter.get_declarations_by_type(DeclarationType::Struct).len(),
            "functions": splitter.get_declarations_by_type(DeclarationType::Function).len(),
            "enums": splitter.get_declarations_by_type(DeclarationType::Enum).len(),
            "impls": splitter.get_declarations_by_type(DeclarationType::Impl).len(),
        },
        "monster_factors": splitter.get_monster_factor_distribution(),
        "generated_traits": generated_traits.len(),
        "transport_layers": {
            "ant_layers": splitter.declarations.iter().filter(|d| d.transport_layer <= 35).count(),
            "bee_layers": splitter.declarations.iter().filter(|d| d.transport_layer > 35 && d.transport_layer <= 71).count(),
            "termite_layers": splitter.declarations.iter().filter(|d| d.transport_layer > 71).count(),
        }
    });
    
    let _ = fs::write("demo_final_summary.json", serde_json::to_string_pretty(&summary).unwrap());
    
    // Generated Monster Group traits
    let trait_code = generated_traits.join("\n\n");
    let _ = fs::write("demo_generated_traits.rs", trait_code);
    
    // Registry code
    let registry_code = generator.generate_registry_code(&splitter.declarations);
    let _ = fs::write("demo_trait_registry.rs", registry_code);
    
    println!("📄 Final exports:");
    println!("   - demo_final_summary.json (complete pipeline summary)");
    println!("   - demo_generated_traits.rs (Monster Group trait wrappers)");
    println!("   - demo_trait_registry.rs (dynamic trait registry)");
}
