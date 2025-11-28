use sha2::{Sha256, Digest};

/// Helper function to calculate SHA256 hash of data
pub fn calculate_content_id(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}
