use anyhow::{Result, anyhow};
use rocksdb::DB;
use std::path::{Path, PathBuf};
use serde_json;

use crate::analysis_types::ProjectFileAnalysis; // Assuming ProjectFileAnalysis is public

// Function to retrieve file analysis
pub fn get_file_analysis(db: &DB, target_file_path: &str) -> Result<()> {
    // Normalize the target_file_path to ensure consistent matching
    let normalized_target_path = PathBuf::from(target_file_path)
                                .canonicalize().unwrap_or_else(|_| PathBuf::from(target_file_path))
                                .to_string_lossy().to_string();

    let prefix = "file_analysis:";

    for item in db.iterator(rocksdb::IteratorMode::Start) {
        let (key_bytes, value_bytes) = item?;
        let key_str = String::from_utf8_lossy(&key_bytes);

        // Keys are in format: "file_analysis:<file_type>:<file_path>:<content_hash>"
        let parts: Vec<&str> = key_str.split(':').collect();

        if parts.len() >= 3 && parts[0] == "file_analysis" {
            let _stored_file_type = parts[1]; // We don't need this for path matching here
            let stored_file_path_raw = parts[2];
            
            // Normalize stored_file_path_raw for comparison
            let stored_file_path_normalized = PathBuf::from(stored_file_path_raw)
                                            .canonicalize().unwrap_or_else(|_| PathBuf::from(stored_file_path_raw))
                                            .to_string_lossy().to_string();

            if stored_file_path_normalized.ends_with(&normalized_target_path) || stored_file_path_raw == target_file_path {
                if let Ok(analysis) = serde_json::from_slice::<ProjectFileAnalysis>(&value_bytes) {
                    println!("{}", serde_json::to_string_pretty(&analysis)?);
                    return Ok(());
                } else {
                    eprintln!("Warning: Failed to deserialize RocksDB entry for key: {}", key_str);
                }
            }
        }
    }

    Err(anyhow!("No analysis found for file: {}", target_file_path))
}
