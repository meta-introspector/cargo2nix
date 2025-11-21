use serde::{Serialize, Deserialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub enum CargoEditRequest {
    /// Request to read the content of a Cargo.toml file.
    ReadCargoToml {
        path: PathBuf,
    },
    /// Request to apply patches to a Cargo.toml file.
    ApplyPatches {
        path: PathBuf,
        patches: Vec<CargoTomlPatch>,
    },
    // Add other cargo edit operations as needed
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CargoEditResponse {
    /// Response containing the content of a Cargo.toml file.
    CargoTomlContent {
        content: String,
    },
    /// Response indicating the success or failure of applying patches.
    ApplyPatchesResult {
        success: bool,
        message: Option<String>,
    },
    /// Generic error response.
    Error {
        message: String,
    },
    // Add other cargo edit responses as needed
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CargoTomlPatch {
    pub section: String, // e.g., "patch.crates-io"
    pub key: String,     // e.g., "my-crate"
    pub value: String,   // e.g., "{ path = \"../my-crate\" }"
}

