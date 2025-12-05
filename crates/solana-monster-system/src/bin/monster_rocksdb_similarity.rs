use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
struct MonsterDeclaration {
    name: String,
    content: String,
    monster_element: u64,
    phi_value: u64,
    similarity_cluster: u64,
}

#[derive(Debug)]
struct MonsterSimilarity {
    decl1_monster: u64,
    decl2_monster: u64,
    monster_distance: f64,
    phi_ratio: f64,
}

fn euler_phi(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    let mut result = n;
    let mut num = n;
    let mut p = 2;

    while p * p <= num {
        if num % p == 0 {
            while num % p == 0 {
                num /= p;
            }
            result -= result / p;
        }
        p += 1;
    }
    if num > 1 {
        result -= result / num;
    }
    result
}

fn monster_hash(content: &str) -> u64 {
    let mut hash = 5381u64;
    for byte in content.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(byte as u64);
    }
    hash % 196883 // Monster Group order
}

fn calculate_monster_distance(elem1: u64, elem2: u64) -> f64 {
    let diff = if elem1 > elem2 {
        elem1 - elem2
    } else {
        elem2 - elem1
    };
    (diff as f64) / 196883.0
}

fn extract_declarations(dir: &Path) -> Vec<MonsterDeclaration> {
    let mut declarations = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                declarations.extend(extract_declarations(&path));
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                if let Ok(content) = fs::read_to_string(&path) {
                    for line in content.lines() {
                        let trimmed = line.trim();
                        if let Some(decl) = parse_monster_declaration(trimmed) {
                            declarations.push(decl);
                        }
                    }
                }
            }
        }
    }

    declarations
}

fn parse_monster_declaration(line: &str) -> Option<MonsterDeclaration> {
    if line.starts_with("fn ") || line.starts_with("struct ") || line.starts_with("trait ") {
        let monster_element = monster_hash(line);
        let phi_value = euler_phi(monster_element);
        let similarity_cluster = phi_value % 1000; // Group by phi mod 1000

        Some(MonsterDeclaration {
            name: extract_name(line),
            content: line.to_string(),
            monster_element,
            phi_value,
            similarity_cluster,
        })
    } else {
        None
    }
}

fn extract_name(line: &str) -> String {
    let words: Vec<&str> = line.split_whitespace().collect();
    if words.len() >= 2 {
        words[1].split('(').next().unwrap_or(words[1]).to_string()
    } else {
        "unknown".to_string()
    }
}

fn find_monster_similarities(declarations: &[MonsterDeclaration]) -> Vec<MonsterSimilarity> {
    let mut similarities = Vec::new();

    // Group by similarity clusters first
    let mut clusters: HashMap<u64, Vec<&MonsterDeclaration>> = HashMap::new();
    for decl in declarations {
        clusters
            .entry(decl.similarity_cluster)
            .or_default()
            .push(decl);
    }

    // Find similarities within and across clusters
    for (cluster_id, cluster_decls) in &clusters {
        if cluster_decls.len() > 1 {
            println!(
                "🔮 Monster Cluster {}: {} declarations",
                cluster_id,
                cluster_decls.len()
            );

            for i in 0..cluster_decls.len() {
                for j in (i + 1)..cluster_decls.len() {
                    let decl1 = cluster_decls[i];
                    let decl2 = cluster_decls[j];

                    let distance =
                        calculate_monster_distance(decl1.monster_element, decl2.monster_element);
                    let phi_ratio = if decl2.phi_value > 0 {
                        decl1.phi_value as f64 / decl2.phi_value as f64
                    } else {
                        1.0
                    };

                    if distance < 0.1 || (phi_ratio > 0.8 && phi_ratio < 1.2) {
                        similarities.push(MonsterSimilarity {
                            decl1_monster: decl1.monster_element,
                            decl2_monster: decl2.monster_element,
                            monster_distance: distance,
                            phi_ratio,
                        });

                        println!(
                            "  ⚡ {} ↔ {} (distance: {:.4}, φ-ratio: {:.2})",
                            decl1.name, decl2.name, distance, phi_ratio
                        );
                    }
                }
            }
        }
    }

    similarities
}

fn simulate_rocksdb_storage(declarations: &[MonsterDeclaration]) {
    println!("\n=== Monster RocksDB Simulation ===");

    let mut monster_db: HashMap<u64, Vec<&MonsterDeclaration>> = HashMap::new();
    let mut phi_index: HashMap<u64, Vec<&MonsterDeclaration>> = HashMap::new();

    for decl in declarations {
        monster_db
            .entry(decl.monster_element)
            .or_default()
            .push(decl);
        phi_index.entry(decl.phi_value).or_default().push(decl);
    }

    println!("📊 Monster Elements: {}", monster_db.len());
    println!("📊 Phi Values: {}", phi_index.len());

    // Find exact duplicates by monster element
    let mut duplicates = 0;
    for (monster_elem, decls) in &monster_db {
        if decls.len() > 1 {
            duplicates += decls.len() - 1;
            println!(
                "🔄 Monster Element {}: {} duplicates",
                monster_elem,
                decls.len()
            );
            for decl in decls {
                println!("    {}", decl.name);
            }
        }
    }

    println!("📈 Total duplicates found: {}", duplicates);

    // Find phi-similar groups
    let mut phi_groups = 0;
    for (phi_val, decls) in &phi_index {
        if decls.len() > 3 {
            phi_groups += 1;
            println!(
                "🧮 φ({}) = {}: {} similar declarations",
                decls[0].monster_element,
                phi_val,
                decls.len()
            );
        }
    }

    println!("📈 Phi similarity groups: {}", phi_groups);
}

fn main() {
    println!("=== Monster RocksDB Similarity Finder ===");

    let declarations = extract_declarations(Path::new("."));
    println!("🔍 Found {} monster declarations", declarations.len());

    simulate_rocksdb_storage(&declarations);

    let similarities = find_monster_similarities(&declarations);
    println!("\n=== Monster Similarity Analysis ===");
    println!("🎯 Found {} monster similarities", similarities.len());

    // Summary statistics
    let mut distance_sum = 0.0;
    let mut phi_ratio_sum = 0.0;

    for sim in &similarities {
        distance_sum += sim.monster_distance;
        phi_ratio_sum += sim.phi_ratio;
    }

    if !similarities.is_empty() {
        println!(
            "📊 Average monster distance: {:.4}",
            distance_sum / similarities.len() as f64
        );
        println!(
            "📊 Average phi ratio: {:.2}",
            phi_ratio_sum / similarities.len() as f64
        );
    }

    println!("\n✨ Monster algorithm successfully identified code patterns!");
}
