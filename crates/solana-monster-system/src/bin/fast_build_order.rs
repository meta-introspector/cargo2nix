fn main() {
    println!("=== SOLANA RUSTC BUILD ORDER (Real Paths) ===");

    // Using actual RUST_SRC_PATH from tools/Makefile
    let solana_rustc_path =
        "/home/mdupont/nix/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src";
    let compiler_path = format!("{}/compiler", solana_rustc_path);

    let crates = vec![
        (
            1,
            9,
            "solana-rustc",
            "1.18.0",
            "main",
            "agave-rust-solana",
            compiler_path.as_str(),
            "agave-hash",
        ),
        (
            2,
            8,
            "rustc-build-sysroot",
            "0.1.0",
            "solana",
            "rustc-build-sysroot",
            "submodules/rustc-build-sysroot",
            "34e8960e",
        ),
        (
            3,
            8,
            "rustc-demangle",
            "0.1.0",
            "main",
            "rustc-demangle",
            "submodules/rustc-demangle",
            "c5688cfe",
        ),
        (
            4,
            7,
            "cargo",
            "0.75.0",
            "CRQ-016-nixify",
            "cargo",
            "submodules/cargo",
            "8e43074b",
        ),
        (
            5,
            7,
            "serde",
            "1.0.228",
            "master",
            "serde",
            "submodules/serde",
            "e42684f9",
        ),
        (
            6,
            5,
            "allocator-api2",
            "0.2.16",
            "CRQ-016-nixify",
            "allocator-api2",
            "submodules/allocator-api2",
            "8174821c",
        ),
    ];

    for (order, crit, name, ver, branch, repo, path, hash) in crates {
        println!(
            "{}. [{}] {} | {} | {} | {} | {} | git:{}",
            order, crit, name, ver, branch, repo, path, hash
        );
    }

    println!("\nSOLANA_RUSTC_PATH: {}", solana_rustc_path);
    println!("Source: tools/Makefile RUST_SRC_PATH");
}
