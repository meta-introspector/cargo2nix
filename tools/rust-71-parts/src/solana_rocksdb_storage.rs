//! Solana RocksDB Storage - Shared memory for Solana validator and agents

use crate::chunked_ast_processor::AstChunk;
use rocksdb::{DB, Options};
use serde::{Deserialize, Serialize};

pub struct SolanaRocksStorage {
    db: DB,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageStats {
    pub chunk_count: usize,
    pub total_size_bytes: u64,
    pub storage_path: String,
}

impl SolanaRocksStorage {
    pub fn new(db_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        let db = DB::open(&opts, db_path)?;
        Ok(Self { db })
    }

    pub fn store_chunk(&self, chunk: &AstChunk) -> Result<(), Box<dyn std::error::Error>> {
        let key = format!("chunk:{}", chunk.chunk_id);
        let value = bincode::serialize(chunk)?;
        self.db.put(&key, &value)?;
        
        // Index by Monster Group factor
        let factor_key = format!("factor:{}:{}", chunk.monster_factor, chunk.chunk_id);
        self.db.put(&factor_key, &chunk.chunk_id.to_le_bytes())?;
        
        Ok(())
    }

    pub fn get_chunk(&self, chunk_id: u64) -> Result<Option<AstChunk>, Box<dyn std::error::Error>> {
        let key = format!("chunk:{}", chunk_id);
        match self.db.get(&key)? {
            Some(bytes) => Ok(Some(bincode::deserialize(&bytes)?)),
            None => Ok(None),
        }
    }

    pub fn store_chunks(&self, chunks: &[AstChunk]) -> Result<(), Box<dyn std::error::Error>> {
        for chunk in chunks {
            self.store_chunk(chunk)?;
        }
        Ok(())
    }

    pub fn get_chunks_by_factor(&self, factor: u64) -> Result<Vec<AstChunk>, Box<dyn std::error::Error>> {
        let prefix = format!("factor:{}:", factor);
        let mut chunks = Vec::new();
        
        let iter = self.db.prefix_iterator(&prefix);
        for item in iter {
            let (_, chunk_id_bytes) = item?;
            let chunk_id = u64::from_le_bytes(
                chunk_id_bytes[..8].try_into().unwrap_or([0; 8])
            );
            
            if let Some(chunk) = self.get_chunk(chunk_id)? {
                chunks.push(chunk);
            }
        }
        
        Ok(chunks)
    }

    pub fn list_chunk_ids(&self) -> Result<Vec<u64>, Box<dyn std::error::Error>> {
        let mut chunk_ids = Vec::new();
        let iter = self.db.iterator(rocksdb::IteratorMode::Start);
        
        for item in iter {
            let (key, _) = item?;
            let key_str = String::from_utf8_lossy(&key);
            
            if key_str.starts_with("chunk:") {
                if let Some(id_str) = key_str.strip_prefix("chunk:") {
                    if let Ok(chunk_id) = id_str.parse::<u64>() {
                        chunk_ids.push(chunk_id);
                    }
                }
            }
        }
        
        Ok(chunk_ids)
    }

    pub fn get_stats(&self) -> StorageStats {
        let chunk_count = self.list_chunk_ids().unwrap_or_default().len();
        StorageStats {
            chunk_count,
            total_size_bytes: 0, // TODO: Calculate from DB
            storage_path: "rocksdb".to_string(),
        }
    }

    pub fn get_all_chunks(&self) -> Result<Vec<AstChunk>, Box<dyn std::error::Error>> {
        let mut chunks = Vec::new();
        let chunk_ids = self.list_chunk_ids()?;
        
        for chunk_id in chunk_ids {
            if let Some(chunk) = self.get_chunk(chunk_id)? {
                chunks.push(chunk);
            }
        }
        
        Ok(chunks)
    }
}
