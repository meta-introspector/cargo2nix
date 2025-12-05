use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub content_id: String,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectFileAnalysis {
    pub file_path: String,
    pub content_hash: String,
    pub file_type: String, // "rust", "markdown", "toml", "unknown"
    pub function_names: Option<Vec<String>>, // Only for Rust files
    pub uses: Option<Vec<String>>, // NEW: List of use statements
    pub mods: Option<Vec<String>>, // NEW: List of mod statements
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IngestionChunk {
    pub chunk_id: String, // Hash of the chunk's content or a sequential ID
    pub files: Vec<IngestionFileDescriptor>,
    pub estimated_size_bytes: usize, // Estimated serialized size of this chunk's metadata
                                     // Potentially other metadata like estimated processing time for files in this chunk
}

#[derive(Debug, Serialize, Deserialize, Clone)] // Derive Clone for IngestionFileDescriptor
pub struct IngestionFileDescriptor {
    pub file_path: String,
    pub content_hash: String,
    pub file_type: String, // From ProjectFileAnalysis
}
