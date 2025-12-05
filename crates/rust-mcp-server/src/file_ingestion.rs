use crate::analysis_types::ProjectFileAnalysis; // Assuming ProjectFileAnalysis is public
use crate::hasher::calculate_content_id;
use anyhow::{Context, Result};
use quote::ToTokens; // NEW: For to_token_stream()
use rocksdb::DB;
use std::path::Path;
use syn::{ItemFn, ItemMod, ItemUse, visit::Visit};
use walkdir::WalkDir; // Assuming calculate_content_id is public

/// A visitor to collect various Rust items
pub struct RustItemCollector {
    pub functions: Vec<String>,
    pub uses: Vec<String>, // NEW
    pub mods: Vec<String>, // NEW
}

impl RustItemCollector {
    pub fn new() -> Self {
        RustItemCollector {
            functions: Vec::new(),
            uses: Vec::new(),
            mods: Vec::new(),
        } // Initialize new fields
    }
}

impl<'ast> Visit<'ast> for RustItemCollector {
    fn visit_item_fn(&mut self, i: &'ast ItemFn) {
        self.functions.push(i.sig.ident.to_string());
        syn::visit::visit_item_fn(self, i);
    }

    fn visit_item_use(&mut self, i: &'ast ItemUse) {
        // NEW
        self.uses.push(i.to_token_stream().to_string()); // Store the whole use statement
        syn::visit::visit_item_use(self, i);
    }

    fn visit_item_mod(&mut self, i: &'ast ItemMod) {
        // NEW
        self.mods.push(i.ident.to_string()); // Store module name
        syn::visit::visit_item_mod(self, i);
    }
}

// Ingests a single file and stores its analysis in RocksDB
pub fn ingest_single_file(
    db: &DB,
    absolute_file_path: &Path,
    project_root: &Path,
) -> Result<ProjectFileAnalysis> {
    use anyhow::anyhow; // Ensure anyhow::anyhow is in scope for this function

    let relative_file_path = absolute_file_path
        .strip_prefix(project_root)
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| absolute_file_path.to_string_lossy().to_string());

    let extension = absolute_file_path.extension().and_then(|s| s.to_str());

    let (file_type, mut function_names, mut uses, mut mods) = match extension {
        Some("rs") => ("rust", Some(Vec::new()), Some(Vec::new()), Some(Vec::new())),
        Some("md") => ("markdown", None, None, None),
        Some("toml") => ("toml", None, None, None),
        _ => ("unknown", None, None, None),
    };

    if file_type == "unknown" {
        return Err(anyhow!(
            "Cannot ingest unknown file type: {}",
            relative_file_path
        ));
    }

    let code = std::fs::read_to_string(&absolute_file_path)
        .context(format!("Failed to read file {}", relative_file_path))?;
    let file_content_hash = calculate_content_id(code.as_bytes());

    if file_type == "rust" {
        let syntax_tree = match std::panic::catch_unwind(|| syn::parse_file(&code)) {
            Ok(Ok(tree)) => tree,
            Ok(Err(e)) => {
                return Err(anyhow!(
                    "Failed to parse Rust code from file {}: {}",
                    relative_file_path,
                    e
                ));
            }
            Err(e) => {
                return Err(anyhow!(
                    "Panic while parsing Rust code from file {}: {:?}",
                    relative_file_path,
                    e
                ));
            }
        };

        let mut collector = RustItemCollector::new();
        collector.visit_file(&syntax_tree);
        function_names = Some(collector.functions);
        uses = Some(collector.uses);
        mods = Some(collector.mods);
    }

    let analysis = ProjectFileAnalysis {
        file_path: relative_file_path.clone(),
        content_hash: file_content_hash.clone(),
        file_type: file_type.to_string(),
        function_names,
        uses,
        mods,
    };

    // Store the analysis in RocksDB using content-addressable key
    let analysis_json = serde_json::to_string(&analysis)?;
    let db_key_content_addressable = format!(
        "file_analysis:{}:{}:{}",
        file_type, relative_file_path, file_content_hash
    );
    db.put(
        db_key_content_addressable.as_bytes(),
        analysis_json.as_bytes(),
    )
    .context(format!(
        "Failed to write content-addressable analysis for {} to RocksDB",
        relative_file_path
    ))?;

    // Also store in git_tree_entry index (path -> latest content hash)
    let db_key_git_tree = format!("git_tree_entry:{}", relative_file_path);
    db.put(db_key_git_tree.as_bytes(), file_content_hash.as_bytes())
        .context(format!(
            "Failed to write git tree entry for {} to RocksDB",
            relative_file_path
        ))?;

    // NEW: Store a direct lookup from file_path to its latest analysis
    let db_key_file_path_to_analysis = format!("file_path_to_analysis:{}", relative_file_path);
    db.put(
        db_key_file_path_to_analysis.as_bytes(),
        analysis_json.as_bytes(),
    )
    .context(format!(
        "Failed to write file_path_to_analysis index for {} to RocksDB",
        relative_file_path
    ))?;

    eprintln!(
        "Stored analysis for {} in RocksDB (and git_tree_entry).",
        relative_file_path
    );

    Ok(analysis)
}

// Function to scan and ingest project files
pub fn scan_and_ingest_project(db: &DB, project_root: &Path) -> Result<()> {
    let start_time = std::time::Instant::now();
    let mut files_processed = 0;
    for entry in WalkDir::new(project_root)
        .into_iter()
        .filter_entry(|e| {
            !e.path()
                .to_string_lossy()
                .contains("submodules/rust/tests/ui/")
        })
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            match ingest_single_file(db, path, project_root) {
                Ok(_) => files_processed += 1,
                Err(e) => eprintln!("Error ingesting file {}: {}", path.display(), e),
            }
        }
    }
    eprintln!(
        "Processed {} files in {:?}.",
        files_processed,
        start_time.elapsed()
    );
    Ok(())
}
