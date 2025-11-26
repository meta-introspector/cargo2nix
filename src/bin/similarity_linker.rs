use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
struct Declaration {
    name: String,
    decl_type: String,
    cas_id: u64,
    content_hash: u64,
    file_path: String,
    signature: String,
}

#[derive(Debug)]
struct SimilarityLink {
    decl1_cas: u64,
    decl2_cas: u64,
    similarity_score: f64,
    link_type: String,
}

fn calculate_content_hash(content: &str) -> u64 {
    let mut hash = 5381u64;
    for byte in content.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(byte as u64);
    }
    hash
}

fn calculate_similarity(sig1: &str, sig2: &str) -> f64 {
    let words1: Vec<&str> = sig1.split_whitespace().collect();
    let words2: Vec<&str> = sig2.split_whitespace().collect();
    
    let common = words1.iter().filter(|w| words2.contains(w)).count();
    let total = (words1.len() + words2.len()) as f64;
    
    if total == 0.0 { 0.0 } else { (2.0 * common as f64) / total }
}

fn extract_declarations(dir: &Path) -> Vec<Declaration> {
    let mut declarations = Vec::new();
    
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "rs") {
                if let Ok(content) = fs::read_to_string(&path) {
                    for (i, line) in content.lines().enumerate() {
                        let trimmed = line.trim();
                        if let Some(decl) = parse_declaration(trimmed, &path, i) {
                            declarations.push(decl);
                        }
                    }
                }
            }
        }
    }
    
    declarations
}

fn parse_declaration(line: &str, file_path: &Path, line_num: usize) -> Option<Declaration> {
    let content_hash = calculate_content_hash(line);
    let cas_id = content_hash % 196883; // Monster Group order
    
    if line.starts_with("fn ") {
        let name = line.split_whitespace().nth(1)?.split('(').next()?;
        Some(Declaration {
            name: name.to_string(),
            decl_type: "fn".to_string(),
            cas_id,
            content_hash,
            file_path: file_path.to_string_lossy().to_string(),
            signature: line.to_string(),
        })
    } else if line.starts_with("struct ") {
        let name = line.split_whitespace().nth(1)?.split('{').next()?.split('(').next()?;
        Some(Declaration {
            name: name.to_string(),
            decl_type: "struct".to_string(),
            cas_id,
            content_hash,
            file_path: file_path.to_string_lossy().to_string(),
            signature: line.to_string(),
        })
    } else if line.starts_with("trait ") {
        let name = line.split_whitespace().nth(1)?.split('{').next()?.split(':').next()?;
        Some(Declaration {
            name: name.to_string(),
            decl_type: "trait".to_string(),
            cas_id,
            content_hash,
            file_path: file_path.to_string_lossy().to_string(),
            signature: line.to_string(),
        })
    } else {
        None
    }
}

fn find_similarities(declarations: &[Declaration]) -> Vec<SimilarityLink> {
    let mut links = Vec::new();
    
    for i in 0..declarations.len() {
        for j in (i + 1)..declarations.len() {
            let decl1 = &declarations[i];
            let decl2 = &declarations[j];
            
            let similarity = calculate_similarity(&decl1.signature, &decl2.signature);
            
            if similarity > 0.7 {
                let link_type = if decl1.decl_type == decl2.decl_type {
                    "same_type".to_string()
                } else {
                    "cross_type".to_string()
                };
                
                links.push(SimilarityLink {
                    decl1_cas: decl1.cas_id,
                    decl2_cas: decl2.cas_id,
                    similarity_score: similarity,
                    link_type,
                });
            }
        }
    }
    
    links
}

fn main() {
    println!("=== Declaration Similarity Linker ===");
    
    let declarations = extract_declarations(Path::new("./src"));
    println!("Found {} declarations", declarations.len());
    
    let mut content_map: HashMap<u64, Vec<&Declaration>> = HashMap::new();
    for decl in &declarations {
        content_map.entry(decl.content_hash).or_default().push(decl);
    }
    
    println!("\n=== Content-Addressable Index ===");
    for (content_id, decls) in &content_map {
        if decls.len() > 1 {
            println!("Content ID {}: {} identical declarations", content_id, decls.len());
            for decl in decls {
                println!("  {} {} (CAS: {})", decl.decl_type, decl.name, decl.cas_id);
            }
        }
    }
    
    let links = find_similarities(&declarations);
    println!("\n=== Similarity Links ===");
    println!("Found {} similarity links", links.len());
    
    for link in &links {
        let decl1 = declarations.iter().find(|d| d.cas_id == link.decl1_cas).unwrap();
        let decl2 = declarations.iter().find(|d| d.cas_id == link.decl2_cas).unwrap();
        
        println!("LINK: {} {} ↔ {} {} (similarity: {:.2}, type: {})",
                 decl1.decl_type, decl1.name,
                 decl2.decl_type, decl2.name,
                 link.similarity_score, link.link_type);
    }
    
    println!("\n=== Summary ===");
    println!("Total declarations: {}", declarations.len());
    println!("Unique content hashes: {}", content_map.len());
    println!("Similarity links: {}", links.len());
}
