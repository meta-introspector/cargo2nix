use crate::agent_vector_db::{AgentCodeVector, MemeValidatorSidechain};
use crate::semantic_constraints::GödelNumber;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Unified agent memory record: cargo/ast/decl = solana account = agent memory = nix store = git object = semantic hash = lmfdb entry = wikidata node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMemoryRecord {
    // Core identifiers
    pub git_hash: String,
    pub nix_store_path: String,
    pub solana_account: String,
    pub semantic_hash: GödelNumber,
    
    // Code structure
    pub cargo_crate: String,
    pub ast_node_type: String,
    pub declaration_name: String,
    
    // Embeddings and vectors
    pub code_embedding: Vec<f64>,
    pub semantic_vector: Vec<f64>,
    
    // External mappings
    pub lmfdb_entry_id: Option<String>,
    pub wikidata_node_id: Option<String>,
    
    // Metadata
    pub file_path: String,
    pub line_number: u32,
    pub compilation_target: String,
}

/// Hugging Face dataset formatter for agent memory records
#[derive(Debug)]
pub struct AgentMemoryFormatter {
    pub records: Vec<AgentMemoryRecord>,
    pub dataset_name: String,
}

impl AgentMemoryFormatter {
    pub fn new(dataset_name: String) -> Self {
        Self {
            records: Vec::new(),
            dataset_name,
        }
    }

    /// Convert RockDB agent code vector to unified memory record
    pub fn format_agent_vector(&mut self, vector: &AgentCodeVector, metadata: AgentMemoryMetadata) -> Result<(), String> {
        let record = AgentMemoryRecord {
            git_hash: hex::encode(vector.git_hash),
            nix_store_path: vector.nix_store_path.clone(),
            solana_account: hex::encode(vector.solana_account),
            semantic_hash: vector.meme_signature.clone(),
            
            cargo_crate: metadata.cargo_crate.clone(),
            ast_node_type: metadata.ast_node_type.clone(),
            declaration_name: metadata.declaration_name.clone(),
            
            code_embedding: vector.embedding.clone(),
            semantic_vector: self.compute_semantic_vector(&vector.meme_signature),
            
            lmfdb_entry_id: self.derive_lmfdb_id(&vector.meme_signature),
            wikidata_node_id: self.derive_wikidata_id(&metadata.declaration_name),
            
            file_path: metadata.file_path,
            line_number: metadata.line_number,
            compilation_target: metadata.compilation_target,
        };

        self.records.push(record);
        Ok(())
    }

    /// Export to Hugging Face dataset format (parquet)
    pub fn export_to_hf_dataset(&self, output_path: &str) -> Result<(), String> {
        // Convert to JSON for HF compatibility
        let json_data = serde_json::to_string_pretty(&self.records)
            .map_err(|e| format!("JSON serialization error: {}", e))?;
        
        std::fs::write(format!("{}/dataset.json", output_path), json_data)
            .map_err(|e| format!("File write error: {}", e))?;

        // Create HF dataset config
        let config = HFDatasetConfig {
            dataset_name: self.dataset_name.clone(),
            description: "Unified agent memory: cargo/ast/decl = solana account = nix store = git object = semantic hash = lmfdb entry = wikidata node".to_string(),
            features: self.get_hf_features(),
            num_records: self.records.len(),
        };

        let config_json = serde_json::to_string_pretty(&config)
            .map_err(|e| format!("Config serialization error: {}", e))?;
        
        std::fs::write(format!("{}/dataset_info.json", output_path), config_json)
            .map_err(|e| format!("Config write error: {}", e))?;

        Ok(())
    }

    /// Sync with meme validator sidechain
    pub fn sync_with_sidechain(&mut self, sidechain: &MemeValidatorSidechain) -> Result<(), String> {
        // Extract all agent code vectors from sidechain
        for (git_hash, node) in &sidechain.vector_db.cluster_nodes {
            for vector in &node.code_vectors {
                let metadata = AgentMemoryMetadata {
                    cargo_crate: "unknown".to_string(),
                    ast_node_type: "unknown".to_string(), 
                    declaration_name: format!("decl_{}", hex::encode(&git_hash[..4])),
                    file_path: vector.nix_store_path.clone(),
                    line_number: 1,
                    compilation_target: "solana-ebpf".to_string(),
                };
                
                self.format_agent_vector(vector, metadata)?;
            }
        }
        Ok(())
    }

    fn compute_semantic_vector(&self, godel_number: &GödelNumber) -> Vec<f64> {
        // Convert Gödel number exponents to semantic vector
        godel_number.exponents.iter().map(|&x| x as f64).collect()
    }

    fn derive_lmfdb_id(&self, godel_number: &GödelNumber) -> Option<String> {
        // Map to LMFDB entry based on prime factorization
        let sum: u32 = godel_number.exponents.iter().map(|&x| x as u32).sum();
        if sum > 0 {
            Some(format!("lmfdb.{}", sum))
        } else {
            None
        }
    }

    fn derive_wikidata_id(&self, declaration_name: &str) -> Option<String> {
        // Simple mapping to Wikidata based on declaration name
        if !declaration_name.is_empty() && declaration_name != "unknown" {
            Some(format!("Q{}", declaration_name.len() * 1000))
        } else {
            None
        }
    }

    fn get_hf_features(&self) -> HashMap<String, String> {
        let mut features = HashMap::new();
        features.insert("git_hash".to_string(), "string".to_string());
        features.insert("nix_store_path".to_string(), "string".to_string());
        features.insert("solana_account".to_string(), "string".to_string());
        features.insert("semantic_hash".to_string(), "object".to_string());
        features.insert("cargo_crate".to_string(), "string".to_string());
        features.insert("ast_node_type".to_string(), "string".to_string());
        features.insert("declaration_name".to_string(), "string".to_string());
        features.insert("code_embedding".to_string(), "array".to_string());
        features.insert("semantic_vector".to_string(), "array".to_string());
        features.insert("lmfdb_entry_id".to_string(), "string".to_string());
        features.insert("wikidata_node_id".to_string(), "string".to_string());
        features.insert("file_path".to_string(), "string".to_string());
        features.insert("line_number".to_string(), "int32".to_string());
        features.insert("compilation_target".to_string(), "string".to_string());
        features
    }
}

#[derive(Debug, Clone)]
pub struct AgentMemoryMetadata {
    pub cargo_crate: String,
    pub ast_node_type: String,
    pub declaration_name: String,
    pub file_path: String,
    pub line_number: u32,
    pub compilation_target: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HFDatasetConfig {
    pub dataset_name: String,
    pub description: String,
    pub features: HashMap<String, String>,
    pub num_records: usize,
}

/// Integration with existing HF dataset validator
pub struct HFIntegration;

impl HFIntegration {
    /// Convert agent memory records to format compatible with existing HF validator
    pub fn convert_to_hf_validator_format(records: &[AgentMemoryRecord]) -> Vec<HFValidatorRecord> {
        records.iter().map(|record| {
            HFValidatorRecord {
                id: record.git_hash.clone(),
                source_code: format!("// {}\n// Nix: {}", record.declaration_name, record.nix_store_path),
                ast_representation: record.ast_node_type.clone(),
                semantic_embedding: record.code_embedding.clone(),
                metadata: HFValidatorMetadata {
                    crate_name: record.cargo_crate.clone(),
                    file_path: record.file_path.clone(),
                    solana_account: record.solana_account.clone(),
                    lmfdb_id: record.lmfdb_entry_id.clone(),
                    wikidata_id: record.wikidata_node_id.clone(),
                },
            }
        }).collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HFValidatorRecord {
    pub id: String,
    pub source_code: String,
    pub ast_representation: String,
    pub semantic_embedding: Vec<f64>,
    pub metadata: HFValidatorMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HFValidatorMetadata {
    pub crate_name: String,
    pub file_path: String,
    pub solana_account: String,
    pub lmfdb_id: Option<String>,
    pub wikidata_id: Option<String>,
}
