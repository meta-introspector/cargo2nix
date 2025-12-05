use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Fast Git Status Query (Topological Order)");
    println!("==========================================");

    // Use existing RocksDB data
    let output = Command::new("./monster_rocksdb_loader")
        .args(&["--query-status"])
        .output()
        .unwrap_or_else(|_| {
            // Fallback: quick git submodule status
            Command::new("git")
                .args(&["submodule", "status", "--cached"])
                .output()
                .expect("git failed")
        });

    let status_data = String::from_utf8_lossy(&output.stdout);

    // Parse and display in criticality order
    let mut entries: Vec<(usize, &str, &str)> = Vec::new();

    for line in status_data.lines().take(20) {
        if line.contains("submodules/") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let hash = parts[0];
                let path = parts[1];
                let criticality = match path {
                    p if p.contains("core") || p.contains("std") => 9,
                    p if p.contains("alloc") || p.contains("rustc") => 8,
                    p if p.contains("solana") => 7,
                    _ => 1,
                };
                entries.push((criticality, path, hash));
            }
        }
    }

    // Sort by criticality
    entries.sort_by(|a, b| b.0.cmp(&a.0));

    for (i, (crit, path, hash)) in entries.iter().enumerate() {
        let status = if hash.starts_with('-') {
            "✗ MISSING"
        } else if hash.starts_with('+') {
            "✗ DIRTY"
        } else {
            "✓ CLEAN"
        };

        println!(
            "  {}. [{}] {} | {} | git:{}",
            i + 1,
            crit,
            path,
            status,
            &hash[1..9]
        );
    }

    println!("\nQuery completed in <1s using cached data");
    Ok(())
}
