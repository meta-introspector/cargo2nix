//! Chunked AST Processor - Read rustc in 4K compressed blocks with syn caching

use syn::{parse_file, File};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

const CHUNK_SIZE: usize = 4096; // 4K blocks

/// Compressed AST chunk with Monster Group metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstChunk {
    pub chunk_id: u64,
    pub monster_factor: u64,
    pub compressed_data: Vec<u8>,
    pub original_size: usize,
    pub file_path: String,
    pub byte_range: (usize, usize),
}

/// Cached syn data for fast access
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedSynData {
    pub file_hash: u64,
    pub ast_summary: AstSummary,
    pub chunks: Vec<AstChunk>,
}

/// AST summary for quick filtering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstSummary {
    pub trait_count: usize,
    pub function_count: usize,
    pub struct_count: usize,
    pub impl_count: usize,
    pub monster_signature: u64,
}

/// Chunked AST processor with caching
pub struct ChunkedAstProcessor {
    cache: HashMap<String, CachedSynData>,
    compression_level: u32,
}

impl ChunkedAstProcessor {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            compression_level: 6, // Balanced compression
        }
    }

    /// Process rustc file into 4K chunks
    pub fn process_file(&mut self, file_path: &str) -> Result<Vec<AstChunk>, Box<dyn std::error::Error>> {
        // Check cache first
        if let Some(cached) = self.cache.get(file_path) {
            let current_hash = self.compute_file_hash(file_path)?;
            if cached.file_hash == current_hash {
                return Ok(cached.chunks.clone());
            }
        }

        // Read and parse file
        let source = fs::read_to_string(file_path)?;
        let syntax_tree = parse_file(&source)?;
        
        // Generate AST summary
        let summary = self.generate_ast_summary(&syntax_tree);
        
        // Split into 4K chunks
        let chunks = self.split_into_chunks(&source, file_path, &summary)?;
        
        // Cache results
        let file_hash = self.compute_file_hash(file_path)?;
        self.cache.insert(file_path.to_string(), CachedSynData {
            file_hash,
            ast_summary: summary,
            chunks: chunks.clone(),
        });
        
        Ok(chunks)
    }

    /// Split source into compressed 4K chunks
    fn split_into_chunks(&self, source: &str, file_path: &str, summary: &AstSummary) -> Result<Vec<AstChunk>, Box<dyn std::error::Error>> {
        let mut chunks = Vec::new();
        let bytes = source.as_bytes();
        
        for (i, chunk_bytes) in bytes.chunks(CHUNK_SIZE).enumerate() {
            let chunk_id = i as u64;
            let monster_factor = self.compute_chunk_monster_factor(chunk_bytes, summary);
            
            // Compress chunk
            let compressed = self.compress_chunk(chunk_bytes)?;
            
            let chunk = AstChunk {
                chunk_id,
                monster_factor,
                compressed_data: compressed,
                original_size: chunk_bytes.len(),
                file_path: file_path.to_string(),
                byte_range: (i * CHUNK_SIZE, (i + 1) * CHUNK_SIZE.min(bytes.len())),
            };
            
            chunks.push(chunk);
        }
        
        Ok(chunks)
    }

    /// Generate AST summary for filtering
    fn generate_ast_summary(&self, syntax_tree: &File) -> AstSummary {
        let mut trait_count = 0;
        let mut function_count = 0;
        let mut struct_count = 0;
        let mut impl_count = 0;
        
        for item in &syntax_tree.items {
            match item {
                syn::Item::Trait(_) => trait_count += 1,
                syn::Item::Fn(_) => function_count += 1,
                syn::Item::Struct(_) => struct_count += 1,
                syn::Item::Impl(_) => impl_count += 1,
                _ => {}
            }
        }
        
        let monster_signature = self.compute_monster_signature(trait_count, function_count, struct_count, impl_count);
        
        AstSummary {
            trait_count,
            function_count,
            struct_count,
            impl_count,
            monster_signature,
        }
    }

    /// Compute Monster Group factor for chunk
    fn compute_chunk_monster_factor(&self, chunk_bytes: &[u8], summary: &AstSummary) -> u64 {
        let chunk_hash = chunk_bytes.iter().fold(0u64, |acc, &b| acc.wrapping_mul(71).wrapping_add(b as u64));
        let summary_hash = summary.monster_signature;
        
        (chunk_hash.wrapping_add(summary_hash)) % 71 + 1
    }

    /// Compute Monster Group signature from AST counts
    fn compute_monster_signature(&self, traits: usize, functions: usize, structs: usize, impls: usize) -> u64 {
        let hash = (traits as u64).wrapping_mul(2)
            .wrapping_add((functions as u64).wrapping_mul(3))
            .wrapping_add((structs as u64).wrapping_mul(5))
            .wrapping_add((impls as u64).wrapping_mul(7));
        
        hash % (71 * 71) + 1 // Use 71^2 for more granularity
    }

    /// Compress chunk using gzip
    fn compress_chunk(&self, data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        use flate2::{write::GzEncoder, Compression};
        use std::io::Write;
        
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(data)?;
        let compressed = encoder.finish()?;
        
        Ok(compressed)
    }

    /// Decompress chunk
    pub fn decompress_chunk(&self, chunk: &AstChunk) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        use flate2::read::GzDecoder;
        use std::io::Read;
        
        let mut decoder = GzDecoder::new(&chunk.compressed_data[..]);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed)?;
        
        Ok(decompressed)
    }

    /// Compute file hash for cache validation
    fn compute_file_hash(&self, file_path: &str) -> Result<u64, Box<dyn std::error::Error>> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let metadata = fs::metadata(file_path)?;
        let mut hasher = DefaultHasher::new();
        
        file_path.hash(&mut hasher);
        metadata.len().hash(&mut hasher);
        metadata.modified()?.duration_since(std::time::UNIX_EPOCH)?.as_secs().hash(&mut hasher);
        
        Ok(hasher.finish())
    }

    /// Query chunks by Monster Group factor
    pub fn query_chunks_by_factor(&self, target_factor: u64) -> Vec<&AstChunk> {
        self.cache.values()
            .flat_map(|cached| &cached.chunks)
            .filter(|chunk| chunk.monster_factor == target_factor)
            .collect()
    }

    /// Query chunks by AST type counts
    pub fn query_by_ast_summary<F>(&self, predicate: F) -> Vec<&CachedSynData>
    where
        F: Fn(&AstSummary) -> bool,
    {
        self.cache.values()
            .filter(|cached| predicate(&cached.ast_summary))
            .collect()
    }

    /// Save cache to disk
    pub fn save_cache(&self, cache_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let serialized = bincode::serialize(&self.cache)?;
        let compressed = self.compress_chunk(&serialized)?;
        fs::write(cache_path, compressed)?;
        Ok(())
    }

    /// Load cache from disk
    pub fn load_cache(&mut self, cache_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        if Path::new(cache_path).exists() {
            let compressed = fs::read(cache_path)?;
            
            use flate2::read::GzDecoder;
            use std::io::Read;
            
            let mut decoder = GzDecoder::new(&compressed[..]);
            let mut decompressed = Vec::new();
            decoder.read_to_end(&mut decompressed)?;
            
            self.cache = bincode::deserialize(&decompressed)?;
        }
        Ok(())
    }

    /// Get compression statistics
    pub fn get_compression_stats(&self) -> CompressionStats {
        let mut total_original = 0;
        let mut total_compressed = 0;
        let mut chunk_count = 0;
        
        for cached in self.cache.values() {
            for chunk in &cached.chunks {
                total_original += chunk.original_size;
                total_compressed += chunk.compressed_data.len();
                chunk_count += 1;
            }
        }
        
        CompressionStats {
            total_original_bytes: total_original,
            total_compressed_bytes: total_compressed,
            compression_ratio: if total_original > 0 { 
                total_compressed as f64 / total_original as f64 
            } else { 
                0.0 
            },
            chunk_count,
        }
    }
}

/// Compression statistics
#[derive(Debug)]
pub struct CompressionStats {
    pub total_original_bytes: usize,
    pub total_compressed_bytes: usize,
    pub compression_ratio: f64,
    pub chunk_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_chunked_processing() {
        let mut processor = ChunkedAstProcessor::new();
        
        // Test with sample Rust code
        let test_code = r#"
            trait TestTrait {
                fn method(&self);
            }
            
            struct TestStruct {
                field: i32,
            }
            
            impl TestTrait for TestStruct {
                fn method(&self) {}
            }
        "#;
        
        // Write to temp file
        let temp_path = "/tmp/test_rust_file.rs";
        std::fs::write(temp_path, test_code).unwrap();
        
        // Process file
        let chunks = processor.process_file(temp_path).unwrap();
        
        assert!(!chunks.is_empty());
        assert!(chunks[0].monster_factor >= 1 && chunks[0].monster_factor <= 71);
        
        // Test decompression
        let decompressed = processor.decompress_chunk(&chunks[0]).unwrap();
        assert!(!decompressed.is_empty());
        
        // Cleanup
        std::fs::remove_file(temp_path).ok();
    }
}
