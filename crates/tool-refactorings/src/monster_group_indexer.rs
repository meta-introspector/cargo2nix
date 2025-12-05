// crates/tool-refactorings/src/monster_group_indexer.rs

use std::path::{Path, PathBuf};
use gemini_utils::gemini_eprintln;
use walkdir::WalkDir;

/// Represents a file to be indexed.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct IndexedFile {
    pub path: PathBuf,
    pub category: String,
    pub index: u64,
}

/// Represents the semantic hash as a product of prime powers.
#[derive(Debug, PartialEq, Eq)]
pub struct SemanticHash {
    pub file_index: u64,
    pub prime_exponents: Vec<(u64, u8)>, // (prime, exponent)
    pub hash_value: u128, // Use a large integer type for the product
}

/// The main application state for the Monster Group Indexer.
pub struct Indexer {
    pub files: Vec<IndexedFile>,
    pub supersingular_primes: [u64; 15],
    pub project_root: PathBuf, // Add project_root to scan from
}

impl Indexer {
    /// Initializes the Indexer with the predefined supersingular primes.
    pub fn new(project_root: PathBuf) -> Self {
        let supersingular_primes = [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71,
        ];
        Self {
            files: Vec::new(),
            supersingular_primes,
            project_root,
        }
    }

    /// Recursively scans the project directory for .nix, .rs, and .toml files.
    /// Assigns a unique index to each file.
    pub fn discover_files(&mut self) -> Result<(), String> {
        gemini_eprintln!("Discovering files in :path:", path = self.project_root.display());
        let mut index_counter = 0;

        for entry in WalkDir::new(&self.project_root).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    let category = match extension.to_str() {
                        Some("nix") => "Nix Configuration and Logic".to_string(),
                        Some("rs") => "Core Rust Implementation".to_string(),
                        Some("toml") => "Rust Project Manifests".to_string(),
                        Some("sh") => "Build and Automation Scripts".to_string(), // Added from spec
                        Some("md") => "Conceptual and Technical Documentation".to_string(), // Added from spec
                        _ => continue, // Skip other file types
                    };

                    self.files.push(IndexedFile {
                        path: path.to_path_buf(),
                        category,
                        index: index_counter,
                    });
                    index_counter += 1;
                }
            }
        }
        gemini_eprintln!("Discovered :count: files.", count = self.files.len());
        Ok(())
    }

    /// Calculates the semantic hash for a given IndexedFile.
    pub fn calculate_hash(&self, file: &IndexedFile) -> SemanticHash {
        let mut prime_exponents = Vec::new();
        let mut hash_value: u128 = 1;
        let mut temp_index = file.index;

        for &prime in self.supersingular_primes.iter() {
            let exponent = (temp_index % 2) as u8; // Least significant bit first
            prime_exponents.push((prime, exponent));
            if exponent == 1 {
                hash_value *= prime as u128;
            }
            temp_index /= 2; // Move to the next bit
            if temp_index == 0 {
                // If index is 0, remaining exponents are 0
                while prime_exponents.len() < self.supersingular_primes.len() {
                    prime_exponents.push((self.supersingular_primes[prime_exponents.len()], 0));
                }
                break;
            }
        }
        
        // Ensure prime_exponents vector has all 15 primes, even if index is small
        while prime_exponents.len() < self.supersingular_primes.len() {
             prime_exponents.push((self.supersingular_primes[prime_exponents.len()], 0));
        }


        SemanticHash {
            file_index: file.index,
            prime_exponents,
            hash_value,
        }
    }

    /// Runs the indexing and hash generation process.
    pub fn run(&mut self) -> Result<(), String> {
        self.discover_files()?;
        
        gemini_eprintln!("Generated Semantic Hashes:");
        for file in &self.files {
            let semantic_hash = self.calculate_hash(file);
            gemini_eprintln!(
                "  File: :path:, Index: :index:, Hash: :hash:, Primes: :primes: ",
                path = file.path.display(),
                index = file.index,
                hash = semantic_hash.hash_value,
                primes = semantic_hash.prime_exponents.iter().map(|(p, e)| format!("{}^{}", p, e)).collect::<Vec<String>>().join(" * ")
            );
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    #[test]
    fn test_indexer_initialization() {
        let root = PathBuf::from("/tmp/test_root");
        let indexer = Indexer::new(root.clone());
        assert_eq!(indexer.supersingular_primes.len(), 15);
        assert_eq!(indexer.supersingular_primes[0], 2);
        assert_eq!(indexer.supersingular_primes[14], 71);
        assert_eq!(indexer.project_root, root);
    }

    #[test]
    fn test_file_discovery() {
        let dir = tempdir().unwrap();
        let root = dir.path().to_path_buf();

        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(root.join("src/main.rs"), "").unwrap();
        fs::write(root.join("Cargo.toml"), "").unwrap();
        fs::write(root.join("flake.nix"), "").unwrap();
        fs::write(root.join("README.md"), "").unwrap(); // Should be discovered
        fs::write(root.join("build.sh"), "").unwrap(); // Should be discovered
        fs::write(root.join("test.txt"), "").unwrap(); // Should be skipped

        let mut indexer = Indexer::new(root.clone());
        indexer.discover_files().unwrap();

        assert_eq!(indexer.files.len(), 5); // main.rs, Cargo.toml, flake.nix, README.md, build.sh

        let rs_file = indexer.files.iter().find(|f| f.path.ends_with("main.rs")).unwrap();
        assert_eq!(rs_file.category, "Core Rust Implementation");

        let toml_file = indexer.files.iter().find(|f| f.path.ends_with("Cargo.toml")).unwrap();
        assert_eq!(toml_file.category, "Rust Project Manifests");

        let nix_file = indexer.files.iter().find(|f| f.path.ends_with("flake.nix")).unwrap();
        assert_eq!(nix_file.category, "Nix Configuration and Logic");

        let md_file = indexer.files.iter().find(|f| f.path.ends_with("README.md")).unwrap();
        assert_eq!(md_file.category, "Conceptual and Technical Documentation");

        let sh_file = indexer.files.iter().find(|f| f.path.ends_with("build.sh")).unwrap();
        assert_eq!(sh_file.category, "Build and Automation Scripts");

        // Ensure indices are unique
        let mut indices: Vec<u64> = indexer.files.iter().map(|f| f.index).collect();
        indices.sort();
        indices.dedup();
        assert_eq!(indices.len(), 5);
    }

    #[test]
    fn test_calculate_hash_index_0() {
        let indexer = Indexer::new(PathBuf::from("/tmp/test_root"));
        let file = IndexedFile {
            path: PathBuf::from("dummy.rs"),
            category: "Core Rust Implementation".to_string(),
            index: 0,
        };
        let semantic_hash = indexer.calculate_hash(&file);
        assert_eq!(semantic_hash.hash_value, 1);
        assert_eq!(semantic_hash.prime_exponents.len(), 15);
        assert!(semantic_hash.prime_exponents.iter().all(|&(_, e)| e == 0));
    }

    #[test]
    fn test_calculate_hash_index_1() {
        let indexer = Indexer::new(PathBuf::from("/tmp/test_root"));
        let file = IndexedFile {
            path: PathBuf::from("dummy.rs"),
            category: "Core Rust Implementation".to_string(),
            index: 1,
        };
        let semantic_hash = indexer.calculate_hash(&file);
        assert_eq!(semantic_hash.hash_value, 2); // 2^1 * 3^0 ...
        assert_eq!(semantic_hash.prime_exponents.len(), 15);
        assert_eq!(semantic_hash.prime_exponents[0].0, 2);
        assert_eq!(semantic_hash.prime_exponents[0].1, 1);
        assert!(semantic_hash.prime_exponents[1..].iter().all(|&(_, e)| e == 0));
    }

    #[test]
    fn test_calculate_hash_index_42() {
        let indexer = Indexer::new(PathBuf::from("/tmp/test_root"));
        let file = IndexedFile {
            path: PathBuf::from("mkcrate.nix"),
            category: "Nix Configuration and Logic".to_string(),
            index: 42, // Binary is 101010
        };
        let semantic_hash = indexer.calculate_hash(&file);
        // Hash(42) = 2^0 * 3^1 * 5^0 * 7^1 * 11^0 * 13^1 = 1 * 3 * 1 * 7 * 1 * 13 = 273
        assert_eq!(semantic_hash.hash_value, 273);
        assert_eq!(semantic_hash.prime_exponents.len(), 15);
        assert_eq!(semantic_hash.prime_exponents[0], (2, 0));  // 2^0
        assert_eq!(semantic_hash.prime_exponents[1], (3, 1));  // 3^1
        assert_eq!(semantic_hash.prime_exponents[2], (5, 0));  // 5^0
        assert_eq!(semantic_hash.prime_exponents[3], (7, 1));  // 7^1
        assert_eq!(semantic_hash.prime_exponents[4], (11, 0)); // 11^0
        assert_eq!(semantic_hash.prime_exponents[5], (13, 1)); // 13^1
        assert!(semantic_hash.prime_exponents[6..].iter().all(|&(_, e)| e == 0));
    }

    #[test]
    fn test_run_indexer() {
        let dir = tempdir().unwrap();
        let root = dir.path().to_path_buf();

        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(root.join("src/main.rs"), "").unwrap();
        fs::write(root.join("Cargo.toml"), "").unwrap();

        let mut indexer = Indexer::new(root.clone());
        let result = indexer.run();
        assert!(result.is_ok());
        assert_eq!(indexer.files.len(), 2);
    }
}