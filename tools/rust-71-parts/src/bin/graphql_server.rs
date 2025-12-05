//! GraphQL server for trait extraction pipeline

use rust_71_parts::declaration_splitter::MonsterDeclarationSplitter;
use rust_71_parts::trait_generator_integration::MonsterTraitGenerator;
use rust_71_parts::graphql_api::{Query, Mutation, graphql_schema};
use serde_json;
use std::io::{self, BufRead, Write};

fn main() {
    println!("🚀 MONSTER GROUP TRAIT EXTRACTION - GRAPHQL API");
    println!("===============================================");
    
    let mut splitter = MonsterDeclarationSplitter::new();
    let generator = MonsterTraitGenerator::new();
    let query = Query;
    let _mutation = Mutation;
    
    println!("GraphQL Schema:");
    println!("{}", graphql_schema());
    
    println!("\n📡 GraphQL Server Ready");
    println!("Available queries:");
    println!("  - pipelineState: Get current pipeline state");
    println!("  - traitsByFactor(factor: Int): Get traits by Monster factor");
    println!("  - monsterFactors: Get all Monster factor distributions");
    
    // Simple REPL for GraphQL queries
    let stdin = io::stdin();
    loop {
        print!("\nGraphQL> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        if stdin.lock().read_line(&mut input).unwrap() == 0 {
            break;
        }
        
        let input = input.trim();
        if input == "quit" || input == "exit" {
            break;
        }
        
        match input {
            "pipelineState" => {
                let state = query.pipeline_state(&splitter);
                println!("{}", serde_json::to_string_pretty(&state).unwrap());
            }
            "monsterFactors" => {
                let factors = query.monster_factors(&splitter);
                println!("{}", serde_json::to_string_pretty(&factors).unwrap());
            }
            "schema" => {
                println!("{}", graphql_schema());
            }
            "demo" => {
                // Load demo data
                let demo_code = r#"
                    trait DemoTrait {
                        fn demo_method(&self);
                    }
                    struct DemoStruct {
                        field: i32,
                    }
                "#;
                let _ = splitter.split_file(demo_code, Some("demo.rs".to_string()));
                println!("Demo data loaded. Try 'pipelineState' now.");
            }
            _ => {
                println!("Available commands: pipelineState, monsterFactors, schema, demo, quit");
            }
        }
    }
    
    println!("GraphQL server stopped.");
}
