// crates/tool-refactorings/src/repository_indexer_config.rs

/// Trait for providing RepositoryIndexer configuration values.
pub trait RepositoryIndexerConfigProvider {
    fn get_default_index_dir(&self) -> &str;
    fn get_metadata_filename(&self) -> &str;
    fn get_chunk_map_filename(&self) -> &str;
    fn get_repository_subdir_name(&self) -> &str;
    fn get_faiss_index_filename(&self) -> &str;
    fn get_docstore_filename(&self) -> &str;
    fn get_index_mapping_filename(&self) -> &str;
}

/// Dummy implementation of RepositoryIndexerConfigProvider returning hardcoded values.
pub struct DefaultRepositoryIndexerConfig;

impl RepositoryIndexerConfigProvider for DefaultRepositoryIndexerConfig {
    fn get_default_index_dir(&self) -> &str {
        "gemini_indices" // Simulating Constants.DEFAULT_INDEX_DIR
    }

    fn get_metadata_filename(&self) -> &str {
        "metadata.json"
    }

    fn get_chunk_map_filename(&self) -> &str {
        "chunk_map.json"
    }

    fn get_repository_subdir_name(&self) -> &str {
        "repository"
    }

    fn get_faiss_index_filename(&self) -> &str {
        "index.faiss"
    }

    fn get_docstore_filename(&self) -> &str {
        "docstore.json"
    }

    fn get_index_mapping_filename(&self) -> &str {
        "index_mapping.json"
    }
}
