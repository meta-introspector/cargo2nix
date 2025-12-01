use hex;
use sha2::{Digest, Sha256};

/// Helper function to calculate SHA256 hash of data
pub fn calculate_content_id(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}
