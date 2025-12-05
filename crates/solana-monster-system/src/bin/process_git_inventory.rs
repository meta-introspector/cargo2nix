use std::collections::HashSet;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string("git_files_inventory.txt")?;
    let mut repos = HashSet::new();

    for line in content.lines() {
        if line.ends_with("/.git") {
            let repo_path = &line[..line.len() - 5]; // Remove "/.git"
            if let Some(name) = repo_path.split('/').last() {
                repos.insert(name.to_string());
            }
        }
    }

    println!("Found {} unique git repositories:", repos.len());
    let mut sorted: Vec<_> = repos.into_iter().collect();
    sorted.sort();

    for repo in &sorted {
        println!("  {}", repo);
    }

    // Save to file
    fs::write("unique_repos.txt", sorted.join("\n"))?;
    println!("\nSaved to unique_repos.txt");

    Ok(())
}
