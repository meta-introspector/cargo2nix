use std::collections::HashMap;
use std::env;
use rocksdb::{DB, Options};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
struct Level0Block {
    hash: String,
    content: String,
    file_path: String,
    block_index: usize,
}

#[derive(Debug, Serialize)]
struct ArithmeticNode {
    hash: String,
    operation: String,
    operands: Vec<String>,
    result_type: String,
    monster_coordinate: Vec<f64>, // Changed from array to Vec
}

#[derive(Debug, Serialize)]
struct GeometricEdge {
    from_hash: String,
    to_hash: String,
    dependency_type: String,
    weight: f64,
    curvature: f64,
}

#[derive(Debug, Serialize)]
struct MonsterCompilerGraph {
    nodes: HashMap<String, ArithmeticNode>,
    edges: Vec<GeometricEdge>,
    compilation_order: Vec<String>,
}

fn extract_operations(content: &str) -> Vec<String> {
    let mut ops = Vec::new();
    
    // Extract function definitions
    for line in content.lines() {
        if line.trim().starts_with("fn ") || line.trim().starts_with("pub fn ") {
            ops.push(format!("FUNCTION: {}", line.trim()));
        }
        if line.trim().starts_with("struct ") || line.trim().starts_with("pub struct ") {
            ops.push(format!("STRUCT: {}", line.trim()));
        }
        if line.trim().starts_with("impl ") {
            ops.push(format!("IMPL: {}", line.trim()));
        }
        if line.contains("+=") || line.contains("-=") || line.contains("*=") {
            ops.push(format!("ARITHMETIC: {}", line.trim()));
        }
    }
    
    ops
}

fn compute_monster_coordinate(hash: &str) -> Vec<f64> {
    let mut coord = vec![0.0; 196883];
    let bytes = hash.as_bytes();
    
    for (i, &byte) in bytes.iter().enumerate() {
        let idx = (i * 71 + byte as usize) % 196883;
        coord[idx] = (byte as f64) / 255.0;
    }
    
    coord
}

fn compute_dependency_weight(from_content: &str, to_content: &str) -> f64 {
    let from_lines: Vec<&str> = from_content.lines().collect();
    let to_lines: Vec<&str> = to_content.lines().collect();
    
    let mut shared_tokens = 0;
    let mut total_tokens = 0;
    
    for line in &from_lines {
        for word in line.split_whitespace() {
            total_tokens += 1;
            if to_content.contains(word) {
                shared_tokens += 1;
            }
        }
    }
    
    if total_tokens == 0 { 0.0 } else { shared_tokens as f64 / total_tokens as f64 }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <source_db> <compiler_graph_db>", args[0]);
        std::process::exit(1);
    }

    let source_db_path = &args[1];
    let graph_db_path = &args[2];

    println!("🏗️  Monster Compiler: Arithmetic-Geometric Architecture");
    println!("📊 Source: {}", source_db_path);
    println!("🔗 Graph: {}", graph_db_path);

    let mut opts = Options::default();
    opts.create_if_missing(true);
    
    let source_db = DB::open_for_read_only(&opts, source_db_path, false)?;
    let graph_db = DB::open(&opts, graph_db_path)?;

    let mut blocks = Vec::new();
    let mut nodes = HashMap::new();
    let mut edges = Vec::new();

    // Phase 1: Load blocks and create arithmetic nodes
    println!("📐 Phase 1: Creating arithmetic nodes...");
    let iter = source_db.iterator(rocksdb::IteratorMode::Start);
    
    for item in iter {
        let (key, value) = item?;
        if let Ok(block) = serde_json::from_slice::<Level0Block>(&value) {
            let operations = extract_operations(&block.content);
            let monster_coord = compute_monster_coordinate(&block.hash);
            
            let node = ArithmeticNode {
                hash: block.hash.clone(),
                operation: operations.join("; "),
                operands: vec![block.file_path.clone()],
                result_type: "RustCode".to_string(),
                monster_coordinate: monster_coord,
            };
            
            nodes.insert(block.hash.clone(), node);
            blocks.push(block);
        }
    }

    println!("  ✓ Created {} arithmetic nodes", nodes.len());

    // Phase 2: Create geometric edges
    println!("🔗 Phase 2: Creating geometric edges...");
    for (i, block_a) in blocks.iter().enumerate() {
        for (j, block_b) in blocks.iter().enumerate() {
            if i != j {
                let weight = compute_dependency_weight(&block_a.content, &block_b.content);
                if weight > 0.1 { // Only significant dependencies
                    let curvature = weight * std::f64::consts::PI / 2.0;
                    
                    let edge = GeometricEdge {
                        from_hash: block_a.hash.clone(),
                        to_hash: block_b.hash.clone(),
                        dependency_type: "SEMANTIC".to_string(),
                        weight,
                        curvature,
                    };
                    
                    edges.push(edge);
                }
            }
        }
        
        if i % 10 == 0 {
            println!("  📊 Processed {} nodes for edges", i);
        }
    }

    println!("  ✓ Created {} geometric edges", edges.len());

    // Phase 3: Compute compilation order using Monster Group topology
    println!("🎯 Phase 3: Computing compilation order...");
    let mut compilation_order = Vec::new();
    let mut visited = std::collections::HashSet::new();
    
    // Sort by Monster Group coordinate magnitude
    let mut sorted_blocks = blocks.clone();
    sorted_blocks.sort_by(|a, b| {
        let coord_a = compute_monster_coordinate(&a.hash);
        let coord_b = compute_monster_coordinate(&b.hash);
        let mag_a: f64 = coord_a.iter().map(|x| x * x).sum::<f64>().sqrt();
        let mag_b: f64 = coord_b.iter().map(|x| x * x).sum::<f64>().sqrt();
        mag_a.partial_cmp(&mag_b).unwrap()
    });
    
    for block in sorted_blocks {
        if !visited.contains(&block.hash) {
            compilation_order.push(block.hash.clone());
            visited.insert(block.hash);
        }
    }

    println!("  ✓ Computed compilation order for {} blocks", compilation_order.len());

    // Phase 4: Store Monster Compiler Graph
    println!("💾 Phase 4: Storing compiler graph...");
    let graph = MonsterCompilerGraph {
        nodes,
        edges,
        compilation_order,
    };

    let graph_json = serde_json::to_vec(&graph)?;
    graph_db.put(b"monster_compiler_graph", graph_json)?;

    println!("✅ Monster Compiler Architecture Complete!");
    println!("  🔢 Arithmetic nodes: {}", graph.nodes.len());
    println!("  📐 Geometric edges: {}", graph.edges.len());
    println!("  🎯 Compilation order: {} blocks", graph.compilation_order.len());

    Ok(())
}
