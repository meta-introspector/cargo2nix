// monster_semantic_indexer.rs

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// The first 15 supersingular primes (the “omens”)
const SUPERSINGULAR_PRIMES: [u64; 15] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71,
];

#[derive(Debug)]
pub struct IndexedFile {
    pub path: PathBuf,
    pub category: String,
    pub index: u64,
}

#[derive(Debug)]
pub struct SemanticHash {
    pub file_index: u64,
    pub prime_exponents: Vec<(u64, u8)>, // (prime, exponent) — exponent is 0 or 1
    pub hash_value: u128,
}

pub struct Indexer {
    pub files: Vec<IndexedFile>,
    pub supersingular_primes: [u64; 15],
}

impl Indexer {
    pub fn new() -> Self {
        Indexer {
            files: Vec::new(),
            supersingular_primes: SUPERSINGULAR_PRIMES,
        }
    }

    /// Recursively discover .nix, .rs, .toml, .sh, .md files (as per your corpus description)
    pub fn discover_files<P: AsRef<Path>>(&mut self, root: P) -> Result<()> {
        let mut index: u64 = 0;

        for entry in WalkDir::new(root)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }

            let ext = path
                .extension()
                .and_then(|s| s.to_str())
                .map(|s| s.to_lowercase());

            let category = match ext.as_deref() {
                Some("nix") => "Nix Configuration",
                Some("rs") => "Rust Source",
                Some("toml") => "TOML Task/Config",
                Some("sh") => "Shell Script",
                Some("md") => "Documentation",
                _ => continue, // ignore everything else
            };

            self.files.push(IndexedFile {
                path: path.to_owned(),
                category: category.to_owned(),
                index,
            });
            index += 1;
        }

        // Sort by path for deterministic ordering across runs
        self.files
            .sort_by_key(|f| f.path.clone());

        // Re-assign indices after sorting
        for (i, file) in self.files.iter_mut().enumerate() {
            file.index = i as u64;
        }

        Ok(())
    }

    /// Core transformation: index → monster-packed bit block → product of prime powers
    pub fn calculate_hash(&self, file: &IndexedFile) -> SemanticHash {
        let mut prime_exponents = Vec::new();
        let mut hash_value: u128 = 1;
        let mut remaining = file.index;

        for (bit_pos, &prime) in self.supersingular_primes.iter().enumerate() {
            // We only need the lowest 15 bits (because we have 15 primes)
            if bit_pos >= 128 {
                break;
            }
            let bit = (remaining & 1) as u8; // 0 or 1
            remaining >>= 1;

            if bit == 1 {
                // Safe because the largest product (71^1 * …) easily fits in u128
                hash_value = hash_value.saturating_mul(prime as u128);
            }
            prime_exponents.push((prime, bit));
        }

        // If the index was larger than 2¹⁵-1, the higher bits are silently ignored
        // (this is intentional — only the first 15 supersingular primes are used)

        SemanticHash {
            file_index: file.index,
            prime_exponents,
            hash_value,
        }
    }

    /// Pretty-print the entire index with semantic hashes
    pub fn print_report(&self) {
        println!("=== Monster Group Semantic Index ===\n");
        println!("Found {} files (indexed 0..{}\n)", self.files.len(), self.files.len() - 1);

        for file in &self.files {
            let sh = self.calculate_hash(file);

            println!(
                "[{:>4}] {:<18} {}",
                file.index,
                sh.hash_value,
                file.path.display()
            );

            // Optional compact exponent representation
            let exponents: Vec<String> = sh
                .prime_exponents
                .iter()
                .filter(|&&(_, exp)| exp == 1)
                .map(|&(p, _)| p.to_string())
                .collect();

            if !exponents.is_empty() {
                println!("       ∟ primes: {}", exponents.join(" × "));
            }
            println!();
        }
    }
}

fn main() -> Result<()> {
    let mut indexer = Indexer::new();

    // Change this to your actual project root
    let project_root = std::env::args()
        .nth(1)
        .unwrap_or_else(|| ".".to_string());

    indexer
        .discover_files(project_root)
        .context("Failed to discover project files")?;

    indexer.print_report();

    // Example: look up mkcrate.nix specifically (as in your spec)
    if let Some(mkcrate) = indexer.files.iter().find(|f| f.path.ends_with("overlay/mkcrate.nix")) {
        let hash = indexer.calculate_hash(mkcrate);
        println!("Special spotlight — overlay/mkcrate.nix");
        println!("  Index : {}", mkcrate.index);
        println!("  Monster hash : {}", hash.hash_value);
        println!(
            "  Prime factors : {}",
            hash.prime_exponents
                .iter()
                .filter(|&&(_, e)| e == 1)
                .map(|&(p, _)| p.to_string())
                .collect::<Vec<_>>()
                .join(" × ")
        );
    }

    Ok(())
}
