use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use crate::cli::args::GenerateWorkspacesArgs;
use crate::cli::args::Cli;
use cargo_repo_sync_cli::analysis::dep_graph_processor::{DepGraphProcessor, RealDepGraphProcessor};
use cargo_repo_sync_cli::analysis::non_vendored_module_finder::{NonVendoredModuleFinder, RealNonVendoredModuleFinder};
use cargo_repo_sync_cli::analysis::dep_graph_data_merger::{DepGraphDataMerger, RealDepGraphDataMerger, MergedCrateInfo};
use walkdir::WalkDir;
use cargo_metadata::MetadataCommand;

pub fn run_generate_workspaces_command(args: &GenerateWorkspacesArgs, _cli: &Cli) -> Result<()> {
    println!("Generating workspaces...");

    let project_root = &args.project_root;
    let depgraph_dot_file = project_root.join(&args.depgraph_dot_file);
    let tree_file = project_root.join(&args.tree_file);
    let output_dir = project_root.join(&args.output_dir);
    let submodules_dir = project_root.join("submodules");

    // 1. Create a map of crate_name -> path
    let mut crate_paths = HashMap::new();
    for entry in WalkDir::new(&submodules_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_str() == Some("Cargo.toml"))
    {
        let cargo_toml_path = entry.path();
        let metadata = MetadataCommand::new()
            .manifest_path(cargo_toml_path)
            .no_deps()
            .exec()?;
        
        if let Some(package) = metadata.packages.get(0) {
            let crate_name = package.name.to_string();
            let crate_path = cargo_toml_path.parent().unwrap().strip_prefix(project_root).unwrap().to_path_buf();
            crate_paths.insert(crate_name, crate_path);
        }
    }

    // 2. Run the analysis pipeline
    let dep_graph_processor = RealDepGraphProcessor;
    let (graph, nodes) = dep_graph_processor.parse_dot_file(&depgraph_dot_file)?;
    let layer_data = dep_graph_processor.calculate_layers(&graph, &nodes)?;

    let non_vendored_finder = RealNonVendoredModuleFinder;
    let usage_counts = non_vendored_finder.find_and_count_non_vendored(&tree_file, &project_root)?;

    let data_merger = RealDepGraphDataMerger;
    let merged_data = data_merger.merge_data(layer_data, usage_counts)?;

    // 3. Group crates by layer
    let mut layers: HashMap<i32, Vec<(String, PathBuf)>> = HashMap::new();
    for (crate_name, info) in &merged_data {
        if let Some(path) = crate_paths.get(crate_name) {
            layers.entry(info.layer).or_default().push((crate_name.clone(), path.clone()));
        }
    }

    // 4. Create output directories and generate Cargo.toml for each layer
    fs::create_dir_all(&output_dir)?;

    let mut all_layers: Vec<_> = layers.keys().cloned().collect();
    all_layers.sort();

    for layer_num in &all_layers {
        let crates = layers.get(layer_num).unwrap();
        let layer_dir = output_dir.join(format!("layer_{}", layer_num));
        fs::create_dir_all(&layer_dir)?;

        let cargo_toml_path = layer_dir.join("Cargo.toml");
        let mut cargo_toml_content = String::new();

        // [workspace] section
        cargo_toml_content.push_str("[workspace]\n");
        cargo_toml_content.push_str("members = [\n");
        for (i, (_crate_name, crate_path)) in crates.iter().enumerate() {
            let relative_path = pathdiff::diff_paths(project_root.join(crate_path), &layer_dir).unwrap();
            cargo_toml_content.push_str(&format!("    \"{}\"", relative_path.display()));
            if i < crates.len() - 1 {
                cargo_toml_content.push_str(",\n");
            } else {
                cargo_toml_content.push_str("\n");
            }
        }
        cargo_toml_content.push_str("]\n\n");

        // [workspace.dependencies] section
        cargo_toml_content.push_str("[workspace.dependencies]\n");
        for lower_layer_num in all_layers.iter().filter(|l| **l < *layer_num) {
            let lower_layer_crates = layers.get(lower_layer_num).unwrap();
            for (crate_name, crate_path) in lower_layer_crates {
                let relative_path = pathdiff::diff_paths(project_root.join(crate_path), &layer_dir).unwrap();
                cargo_toml_content.push_str(&format!("{} = {{ path = \"{}\" }}\n", crate_name, relative_path.display()));
            }
        }
        cargo_toml_content.push_str("\n");

        // [patch.crates-io] section
        cargo_toml_content.push_str("[patch.crates-io]\n");
        for (crate_name, crate_path) in &crate_paths {
            let relative_path = pathdiff::diff_paths(project_root.join(crate_path), &layer_dir).unwrap();
            cargo_toml_content.push_str(&format!("{} = {{ path = \"{}\" }}\n", crate_name, relative_path.display()));
        }

        fs::write(&cargo_toml_path, cargo_toml_content)?;
    }

    println!("Workspaces generated in {:?}", output_dir);

    Ok(())
}
