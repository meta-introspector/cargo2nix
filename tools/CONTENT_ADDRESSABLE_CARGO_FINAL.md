# Content-Addressable Cargo.toml Report

## Content Addressing Flow
git obj -> cargo.toml -> name and version -> content hash -> merge by content address

## Content Mappings

### dummy_crate v0.1.0
- **Git Object**: `unknown`
- **Content Hash**: `c66d3843c73d`
- **Repository**: `dummy_crate`
- **Path**: `submodules/dummy_crate/Cargo.toml`
- **Mapping**: `unknown` -> `c66d3843c73d` -> **dummy_crate** v0.1.0

### b3sum v1.8.2
- **Git Object**: `eae9bf376a1c`
- **Content Hash**: `9857d44d77ce`
- **Repository**: `BLAKE3`
- **Path**: `submodules/BLAKE3/b3sum/Cargo.toml`
- **Mapping**: `eae9bf376a1c` -> `9857d44d77ce` -> **b3sum** v1.8.2

### blake3 v1.8.2
- **Git Object**: `eae9bf376a1c`
- **Content Hash**: `e7266fb9765c`
- **Repository**: `BLAKE3`
- **Path**: `submodules/BLAKE3/Cargo.toml`
- **Mapping**: `eae9bf376a1c` -> `e7266fb9765c` -> **blake3** v1.8.2

### test_vectors v0.0.0
- **Git Object**: `eae9bf376a1c`
- **Content Hash**: `af5becb009e2`
- **Repository**: `BLAKE3`
- **Path**: `submodules/BLAKE3/test_vectors/Cargo.toml`
- **Mapping**: `eae9bf376a1c` -> `af5becb009e2` -> **test_vectors** v0.0.0

### blake3_c_rust_bindings v0.0.0
- **Git Object**: `eae9bf376a1c`
- **Content Hash**: `a2999cffbead`
- **Repository**: `BLAKE3`
- **Path**: `submodules/BLAKE3/c/blake3_c_rust_bindings/Cargo.toml`
- **Mapping**: `eae9bf376a1c` -> `a2999cffbead` -> **blake3_c_rust_bindings** v0.0.0

### compiler_version v0.0.0
- **Git Object**: `eae9bf376a1c`
- **Content Hash**: `3a6e53681ef1`
- **Repository**: `BLAKE3`
- **Path**: `submodules/BLAKE3/tools/compiler_version/Cargo.toml`
- **Mapping**: `eae9bf376a1c` -> `3a6e53681ef1` -> **compiler_version** v0.0.0

### instruction_set_support v0.0.0
- **Git Object**: `eae9bf376a1c`
- **Content Hash**: `eb257638448b`
- **Repository**: `BLAKE3`
- **Path**: `submodules/BLAKE3/tools/instruction_set_support/Cargo.toml`
- **Mapping**: `eae9bf376a1c` -> `eb257638448b` -> **instruction_set_support** v0.0.0

### reference_impl v0.0.0
- **Git Object**: `eae9bf376a1c`
- **Content Hash**: `998c3c5b71e3`
- **Repository**: `BLAKE3`
- **Path**: `submodules/BLAKE3/reference_impl/Cargo.toml`
- **Mapping**: `eae9bf376a1c` -> `998c3c5b71e3` -> **reference_impl** v0.0.0

### benchmark v0.0.0
- **Git Object**: `0e14736ad1ac`
- **Content Hash**: `35d2564e5254`
- **Repository**: `juniper`
- **Path**: `submodules/juniper/benches/Cargo.toml`
- **Mapping**: `0e14736ad1ac` -> `35d2564e5254` -> **benchmark** v0.0.0

### juniper_integration_tests v0.0.0
- **Git Object**: `0e14736ad1ac`
- **Content Hash**: `cef00a7879d6`
- **Repository**: `juniper`
- **Path**: `submodules/juniper/tests/integration/Cargo.toml`
- **Mapping**: `0e14736ad1ac` -> `cef00a7879d6` -> **juniper_integration_tests** v0.0.0

### juniper_codegen_tests v0.0.0
- **Git Object**: `0e14736ad1ac`
- **Content Hash**: `802b5d787e33`
- **Repository**: `juniper`
- **Path**: `submodules/juniper/tests/codegen/Cargo.toml`
- **Mapping**: `0e14736ad1ac` -> `802b5d787e33` -> **juniper_codegen_tests** v0.0.0

### subscription v0.9.0
- **Git Object**: `0e14736ad1ac`
- **Content Hash**: `e04583a446db`
- **Repository**: `juniper`
- **Path**: `submodules/juniper/juniper_warp/Cargo.toml`
- **Mapping**: `0e14736ad1ac` -> `e04583a446db` -> **subscription** v0.9.0

### juniper_rocket v0.10.0
- **Git Object**: `0e14736ad1ac`
- **Content Hash**: `ad05d442ff5f`
- **Repository**: `juniper`
- **Path**: `submodules/juniper/juniper_rocket/Cargo.toml`
- **Mapping**: `0e14736ad1ac` -> `ad05d442ff5f` -> **juniper_rocket** v0.10.0

### ws_test_suite v0.3.0
- **Git Object**: `0e14736ad1ac`
- **Content Hash**: `81ebe75cbad2`
- **Repository**: `juniper`
- **Path**: `submodules/juniper/juniper_axum/Cargo.toml`
- **Mapping**: `0e14736ad1ac` -> `81ebe75cbad2` -> **ws_test_suite** v0.3.0

### unknown vunknown
- **Git Object**: `0e14736ad1ac`
- **Content Hash**: `63e74380f02f`
- **Repository**: `juniper`
- **Path**: `submodules/juniper/Cargo.toml`
- **Mapping**: `0e14736ad1ac` -> `63e74380f02f` -> **unknown** vunknown

... and 9 more mappings

## Content Analysis
- Total Cargo.toml files: 24
- Unique content hashes: 24
- Identical content groups: 0

## Repository Breakdown
- **cast**: 1 Cargo.toml files
- **juniper**: 14 Cargo.toml files
- **BLAKE3**: 7 Cargo.toml files
- **dummy_crate**: 1 Cargo.toml files
- **diff**: 1 Cargo.toml files

## Content-Addressable Storage Benefits
- **Deduplication**: Store identical content once by hash
- **Version Tracking**: Git objects track repository state
- **Content Integrity**: Hashes ensure data integrity
- **Cross-Repo Analysis**: Same content visible across repos
- **Merge Capability**: Combine file systems by content address
- **Git Log Integration**: Track changes through git history

## RocksDB Content-Addressable Schema
```
Key: content_hash
Value: {
  name: string,
  version: string,
  git_locations: [{
    git_repo: string,
    git_object: string,
    cargo_toml_path: string
  }]
}
```
