use std::env;
use std::path::{Path, PathBuf};

// Copied from submodules/rocksdb/librocksdb-sys/build.rs
#[derive(Debug, Clone)]
struct NixPaths {
    llvm_config: String,
    libclang_path: String,
    llvm_config_path: String,
    glibc_dev: String,
    gcc_path: String,
    gcc_version: String, 
}

impl NixPaths {
    fn default_nix_paths() -> Self {
        println!("cargo:warning=Using hardcoded default Nix paths for test crate.");
        NixPaths {
            llvm_config: "/nix/store/nasb2hacyvikadjhr9qip2r8b72ir819-llvm-19.1.7/bin/llvm-config".to_string(),
            libclang_path: "/nix/store/10mkp77lmqz8x2awd8hzv6pf7f7rkf6d-clang-19.1.7-lib/lib".to_string(),
            llvm_config_path: "/nix/store/nasb2hacyvikadjhr9qip2r8b72ir819-llvm-19.1.7/lib".to_string(),
            glibc_dev: "/nix/store/gi4cz4ir3zlwhf1azqfgxqdnczfrwsr7-glibc-2.40-66-dev".to_string(),
            gcc_path: "/nix/store/kzq78n13l8w24jn8bx4djj79k5j717f1-gcc-14.3.0".to_string(),
            gcc_version: "14.3.0".to_string(), 
        }
    }
}


fn main() {
    // Aggressively unset environment variables that could interfere with the build process
    env::remove_var("CFLAGS");
    env::remove_var("CXXFLAGS");
    env::remove_var("CPATH");
    env::remove_var("LIBRARY_PATH");
    env::remove_var("LD_LIBRARY_PATH");
    env::remove_var("PROTOC");
    env::remove_var("PROTOC_INCLUDE");
    env::remove_var("BINDGEN_EXTRA_CLANG_ARGS");
    env::remove_var("LLVM_CONFIG");
    env::remove_var("LLVM_CONFIG_PATH");
    env::remove_var("LIBCLANG_PATH");
    env::remove_var("LIBCLANG_FLAGS");
    env::remove_var("PKG_CONFIG_PATH");
    env::remove_var("NIX_GLIBC_DEV");
    env::remove_var("NIX_GCC_PATH");
    env::remove_var("NIX_GCC_REAL_PATH");

    let nix_paths = NixPaths::default_nix_paths();

    let mut build = cc::Build::new();
    build.cpp(true); // Compile as C++

    // Explicitly set the compiler to g++ using the path from NixPaths
    build.compiler(&format!("{}/bin/g++", nix_paths.gcc_path));

    // Add the C++ standard for the compilation
    build.flag("-std=c++20");

    // Add local source file
    build.file("src/c_code.cpp");
    
    build.compile("dummy_c_code");
}