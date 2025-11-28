use anyhow::{Result, anyhow};
use rocksdb::DB;
use std::collections::HashMap;
use serde_json;

use crate::analysis_types::ProjectFileAnalysis; // Assuming ProjectFileAnalysis is public

// Function to query project analysis
pub fn query_project_analysis(db: &DB) -> Result<()> {
    let mut all_analyses = Vec::new();
    let mut content_hash_counts: HashMap<String, usize> = HashMap::new();
    let mut duplicate_content_hashes: HashMap<String, Vec<String>> = HashMap::new();
    let mut file_type_counts: HashMap<String, usize> = HashMap::new();
    let mut unique_file_type_hashes: HashMap<String, HashMap<String, usize>> = HashMap::new();


    let prefix = b"file_analysis:";

    for item in db.iterator(rocksdb::IteratorMode::Start) {
        let (key_bytes, value_bytes) = item?;
        let key_str = String::from_utf8_lossy(&key_bytes);

        if key_str.starts_with("file_analysis:") {
            if let Ok(analysis) = serde_json::from_slice::<ProjectFileAnalysis>(&value_bytes) {
                *content_hash_counts.entry(analysis.content_hash.clone()).or_insert(0) += 1;
                *file_type_counts.entry(analysis.file_type.clone()).or_insert(0) += 1;
                *unique_file_type_hashes.entry(analysis.file_type.clone())
                                         .or_insert_with(HashMap::new)
                                         .entry(analysis.content_hash.clone())
                                         .or_insert(0) += 1;
                all_analyses.push(analysis);
            } else {
                eprintln!("Warning: Failed to deserialize RocksDB entry: {}", key_str);
            }
        }
    }

    // Identify duplicate content hashes
    for analysis in &all_analyses {
        if let Some(&count) = content_hash_counts.get(&analysis.content_hash) {
            if count > 1 {
                duplicate_content_hashes.entry(analysis.content_hash.clone())
                                        .or_insert_with(Vec::new)
                                        .push(analysis.file_path.clone());
            }
        }
    }

    eprintln!("\n--- Project Analysis Summary ---");
    eprintln!("Total analyzed files indexed: {}", all_analyses.len());
    eprintln!("Unique file content hashes (overall): {}", content_hash_counts.len());
    eprintln!("Files with duplicate content hashes (overall): {}", duplicate_content_hashes.len());

    eprintln!("\n--- Analysis by File Type ---");
    for (file_type, count) in file_type_counts {
        let unique_hashes = unique_file_type_hashes.get(&file_type).map_or(0, |m| m.len());
        eprintln!("  {}: Total = {}, Unique Hashes = {}", file_type, count, unique_hashes);
    }


    if !duplicate_content_hashes.is_empty() {
        eprintln!("\n--- Details of Duplicate Content Hashes (Overall) ---");
        // Sort duplicates by content hash for consistent output
        let mut sorted_duplicates: Vec<_> = duplicate_content_hashes.into_iter().collect();
        sorted_duplicates.sort_by(|a, b| a.0.cmp(&b.0));

        for (hash, paths) in sorted_duplicates {
            eprintln!("Content Hash: {}", hash);
            // Sort paths for consistent output
            let mut sorted_paths = paths;
            sorted_paths.sort();
            for path in sorted_paths {
                eprintln!("  - {}", path);
            }
        }
    } else {
        eprintln!("\nNo duplicate content hashes found.");
    }

    Ok(())
}
