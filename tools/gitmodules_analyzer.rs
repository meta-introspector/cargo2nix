use std::fs;
use std::collections::{HashMap, HashSet};
use std::process::Command;

#[derive(Debug, Clone)]
struct GitModule {
    name: String,
    path: String,
    url: String,
    branch: Option<String>,
    location: String, // File path where found
}

#[derive(Debug)]
struct CargoGitDep {
    name: String,
    url: String,
    rev: Option<String>,
    branch: Option<String>,
    tag: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut all_modules = Vec::new();
    let mut cargo_deps = Vec::new();
    
    // Read git inventory
    if let Ok(content) = fs::read_to_string("git_files_inventory2.txt") {
        println!("Processing git inventory...");
        process_git_inventory(&content, &mut all_modules)?;
    } else if let Ok(content) = fs::read_to_string("git_files_inventory.txt") {
        println!("Processing git inventory...");
        process_git_inventory(&content, &mut all_modules)?;
    }
    
    // Find all Cargo.lock files and extract git deps
    find_cargo_git_deps(&mut cargo_deps)?;
    
    // Analyze duplicates and conflicts
    analyze_duplicates(&all_modules, &cargo_deps)?;
    
    Ok(())
}

fn process_git_inventory(content: &str, modules: &mut Vec<GitModule>) -> Result<(), Box<dyn std::error::Error>> {
    for line in content.lines() {
        if line.ends_with("/.gitmodules") {
            let gitmodules_path = line;
            if let Ok(gitmodules_content) = fs::read_to_string(gitmodules_path) {
                parse_gitmodules(&gitmodules_content, gitmodules_path, modules);
            }
        }
    }
    
    println!("Found {} git submodules from .gitmodules files", modules.len());
    Ok(())
}

fn parse_gitmodules(content: &str, location: &str, modules: &mut Vec<GitModule>) {
    let mut current_module = GitModule {
        name: String::new(),
        path: String::new(),
        url: String::new(),
        branch: None,
        location: location.to_string(),
    };
    
    for line in content.lines() {
        let line = line.trim();
        
        if line.starts_with("[submodule \"") {
            if !current_module.name.is_empty() {
                modules.push(current_module.clone());
            }
            current_module.name = line[12..line.len()-2].to_string();
            current_module.path.clear();
            current_module.url.clear();
            current_module.branch = None;
        } else if line.starts_with("path = ") {
            current_module.path = line[7..].to_string();
        } else if line.starts_with("url = ") {
            current_module.url = line[6..].to_string();
        } else if line.starts_with("branch = ") {
            current_module.branch = Some(line[9..].to_string());
        }
    }
    
    if !current_module.name.is_empty() {
        modules.push(current_module);
    }
}

fn find_cargo_git_deps(deps: &mut Vec<CargoGitDep>) -> Result<(), Box<dyn std::error::Error>> {
    // Find all Cargo.lock files
    let output = Command::new("find")
        .args(&[".", "-name", "Cargo.lock"])
        .output()?;
    
    let cargo_locks = String::from_utf8_lossy(&output.stdout);
    
    for cargo_lock_path in cargo_locks.lines() {
        if let Ok(content) = fs::read_to_string(cargo_lock_path) {
            parse_cargo_lock(&content, deps);
        }
    }
    
    println!("Found {} git dependencies from Cargo.lock files", deps.len());
    Ok(())
}

fn parse_cargo_lock(content: &str, deps: &mut Vec<CargoGitDep>) {
    let mut in_package = false;
    let mut current_name = String::new();
    let mut current_source = String::new();
    
    for line in content.lines() {
        let line = line.trim();
        
        if line == "[[package]]" {
            in_package = true;
            current_name.clear();
            current_source.clear();
        } else if in_package {
            if line.starts_with("name = \"") {
                current_name = line[8..line.len()-1].to_string();
            } else if line.starts_with("source = \"git+") {
                current_source = line[15..line.len()-1].to_string();
                
                if !current_name.is_empty() && !current_source.is_empty() {
                    let (url, rev, branch, tag) = parse_git_source(&current_source);
                    deps.push(CargoGitDep {
                        name: current_name.clone(),
                        url,
                        rev,
                        branch,
                        tag,
                    });
                }
            } else if line.is_empty() {
                in_package = false;
            }
        }
    }
}

fn parse_git_source(source: &str) -> (String, Option<String>, Option<String>, Option<String>) {
    let mut url = source.to_string();
    let mut rev = None;
    let mut branch = None;
    let mut tag = None;
    
    if let Some(query_start) = source.find('?') {
        url = source[..query_start].to_string();
        let query = &source[query_start+1..];
        
        for param in query.split('&') {
            if let Some((key, value)) = param.split_once('=') {
                match key {
                    "rev" => rev = Some(value.to_string()),
                    "branch" => branch = Some(value.to_string()),
                    "tag" => tag = Some(value.to_string()),
                    _ => {}
                }
            }
        }
    }
    
    (url, rev, branch, tag)
}

fn analyze_duplicates(modules: &[GitModule], cargo_deps: &[CargoGitDep]) -> Result<(), Box<dyn std::error::Error>> {
    let mut url_counts = HashMap::new();
    let mut name_counts = HashMap::new();
    let mut all_urls = HashSet::new();
    
    // Count submodule duplicates
    for module in modules {
        *url_counts.entry(&module.url).or_insert(0) += 1;
        *name_counts.entry(&module.name).or_insert(0) += 1;
        all_urls.insert(&module.url);
    }
    
    // Add cargo git deps
    for dep in cargo_deps {
        all_urls.insert(&dep.url);
    }
    
    println!("\n=== DUPLICATE ANALYSIS ===");
    println!("Total unique URLs: {}", all_urls.len());
    println!("Total submodules: {}", modules.len());
    println!("Total cargo git deps: {}", cargo_deps.len());
    
    println!("\nDuplicate URLs in submodules:");
    for (url, count) in &url_counts {
        if *count > 1 {
            println!("  {} ({}x)", url, count);
            for module in modules {
                if &module.url == *url {
                    println!("    {} at {}", module.name, module.location);
                }
            }
        }
    }
    
    println!("\nDuplicate names in submodules:");
    for (name, count) in &name_counts {
        if *count > 1 {
            println!("  {} ({}x)", name, count);
        }
    }
    
    // Generate comprehensive report
    let mut report = String::new();
    report.push_str("# Git Repository Analysis Report\n\n");
    
    report.push_str("## Submodules by Location\n");
    for module in modules {
        report.push_str(&format!(
            "- **{}** at `{}` → `{}`{}\n  Location: {}\n",
            module.name,
            module.path,
            module.url,
            module.branch.as_ref().map(|b| format!(" (branch: {})", b)).unwrap_or_default(),
            module.location
        ));
    }
    
    report.push_str("\n## Cargo Git Dependencies\n");
    for dep in cargo_deps {
        report.push_str(&format!(
            "- **{}** → `{}`{}{}{}\n",
            dep.name,
            dep.url,
            dep.rev.as_ref().map(|r| format!(" (rev: {})", r)).unwrap_or_default(),
            dep.branch.as_ref().map(|b| format!(" (branch: {})", b)).unwrap_or_default(),
            dep.tag.as_ref().map(|t| format!(" (tag: {})", t)).unwrap_or_default()
        ));
    }
    
    fs::write("git_analysis_report.md", report)?;
    println!("\nDetailed report saved to git_analysis_report.md");
    
    Ok(())
}
