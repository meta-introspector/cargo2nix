# Cargo.toml to Git Module Mapping Report

## Mapping Relationship Flow
cargo toml - in file in - git module

## Cargo.toml Files by Git Module

### juniper (13 Cargo.toml files)
- **Git Module Path**: `submodules/juniper`
- **Git URL**: `https://github.com/meta-introspector/juniper`
- **Git Object**: `0e14736ad1acbb8a25f142ec426203c8797c4755`
- **Cargo.toml Files**:
  - **benchmark**
    - Version: `0.0.0`
    - File: `submodules/juniper/benches/Cargo.toml`
    - **Relationship**: `submodules/juniper/benches/Cargo.toml` - **in file in** - `juniper`
  - **juniper_integration_tests**
    - Version: `0.0.0`
    - File: `submodules/juniper/tests/integration/Cargo.toml`
    - **Relationship**: `submodules/juniper/tests/integration/Cargo.toml` - **in file in** - `juniper`
  - **juniper_codegen_tests**
    - Version: `0.0.0`
    - File: `submodules/juniper/tests/codegen/Cargo.toml`
    - **Relationship**: `submodules/juniper/tests/codegen/Cargo.toml` - **in file in** - `juniper`
  - **subscription**
    - Version: `0.9.0`
    - File: `submodules/juniper/juniper_warp/Cargo.toml`
    - **Relationship**: `submodules/juniper/juniper_warp/Cargo.toml` - **in file in** - `juniper`
  - **juniper_rocket**
    - Version: `0.10.0`
    - File: `submodules/juniper/juniper_rocket/Cargo.toml`
    - **Relationship**: `submodules/juniper/juniper_rocket/Cargo.toml` - **in file in** - `juniper`
  - **ws_test_suite**
    - Version: `0.3.0`
    - File: `submodules/juniper/juniper_axum/Cargo.toml`
    - **Relationship**: `submodules/juniper/juniper_axum/Cargo.toml` - **in file in** - `juniper`
  - **subscription**
    - Version: `0.7.0`
    - File: `submodules/juniper/juniper_actix/Cargo.toml`
    - **Relationship**: `submodules/juniper/juniper_actix/Cargo.toml` - **in file in** - `juniper`
  - **juniper_book**
    - Version: `0.0.0`
    - File: `submodules/juniper/book/Cargo.toml`
    - **Relationship**: `submodules/juniper/book/Cargo.toml` - **in file in** - `juniper`
  - **juniper_graphql_ws**
    - Version: `0.5.0`
    - File: `submodules/juniper/juniper_graphql_ws/Cargo.toml`
    - **Relationship**: `submodules/juniper/juniper_graphql_ws/Cargo.toml` - **in file in** - `juniper`
  - **juniper_codegen**
    - Version: `0.17.0`
    - File: `submodules/juniper/juniper_codegen/Cargo.toml`
    - **Relationship**: `submodules/juniper/juniper_codegen/Cargo.toml` - **in file in** - `juniper`
  - ... and 3 more Cargo.toml files

### BLAKE3 (7 Cargo.toml files)
- **Git Module Path**: `submodules/BLAKE3`
- **Git URL**: `https://github.com/meta-introspector/BLAKE3.git`
- **Git Object**: `eae9bf376a1c4797df7be6e49e735c0a5d91dcb0`
- **Cargo.toml Files**:
  - **b3sum**
    - Version: `1.8.2`
    - File: `submodules/BLAKE3/b3sum/Cargo.toml`
    - **Relationship**: `submodules/BLAKE3/b3sum/Cargo.toml` - **in file in** - `BLAKE3`
  - **blake3**
    - Version: `1.8.2`
    - File: `submodules/BLAKE3/Cargo.toml`
    - **Relationship**: `submodules/BLAKE3/Cargo.toml` - **in file in** - `BLAKE3`
  - **test_vectors**
    - Version: `0.0.0`
    - File: `submodules/BLAKE3/test_vectors/Cargo.toml`
    - **Relationship**: `submodules/BLAKE3/test_vectors/Cargo.toml` - **in file in** - `BLAKE3`
  - **blake3_c_rust_bindings**
    - Version: `0.0.0`
    - File: `submodules/BLAKE3/c/blake3_c_rust_bindings/Cargo.toml`
    - **Relationship**: `submodules/BLAKE3/c/blake3_c_rust_bindings/Cargo.toml` - **in file in** - `BLAKE3`
  - **compiler_version**
    - Version: `0.0.0`
    - File: `submodules/BLAKE3/tools/compiler_version/Cargo.toml`
    - **Relationship**: `submodules/BLAKE3/tools/compiler_version/Cargo.toml` - **in file in** - `BLAKE3`
  - **instruction_set_support**
    - Version: `0.0.0`
    - File: `submodules/BLAKE3/tools/instruction_set_support/Cargo.toml`
    - **Relationship**: `submodules/BLAKE3/tools/instruction_set_support/Cargo.toml` - **in file in** - `BLAKE3`
  - **reference_impl**
    - Version: `0.0.0`
    - File: `submodules/BLAKE3/reference_impl/Cargo.toml`
    - **Relationship**: `submodules/BLAKE3/reference_impl/Cargo.toml` - **in file in** - `BLAKE3`

### cast (1 Cargo.toml files)
- **Git Module Path**: `submodules/cast`
- **Git URL**: `https://github.com/meta-introspector/cast.git`
- **Git Object**: `052288097de1846b938e854e27845a93a6f4b59d`
- **Cargo.toml Files**:
  - **cast**
    - Version: `0.3.0`
    - File: `submodules/cast/Cargo.toml`
    - **Relationship**: `submodules/cast/Cargo.toml` - **in file in** - `cast`

### diff (1 Cargo.toml files)
- **Git Module Path**: `submodules/diff`
- **Git URL**: `https://github.com/meta-introspector/diff.git`
- **Git Object**: `36e19c9527fc66d1d2e9c6ba1615680d818447c4`
- **Cargo.toml Files**:
  - **benches**
    - Version: `0.1.13`
    - File: `submodules/diff/Cargo.toml`
    - **Relationship**: `submodules/diff/Cargo.toml` - **in file in** - `diff`

## Mapping Statistics
- Total Cargo.toml files: 22
- Git modules containing Cargo.toml: 4
- Average Cargo.toml files per git module: 5.5

## Git Modules with Most Cargo.toml Files
1. **juniper** - 13 Cargo.toml files
2. **BLAKE3** - 7 Cargo.toml files
3. **cast** - 1 Cargo.toml files
4. **diff** - 1 Cargo.toml files

## RocksDB Cargo-to-Git Schema
```
Key: cargo_toml_path
Value: {
  crate_name: string,
  version: string,
  git_module_path: string,
  git_url: string,
  git_object: string,
  relationship: "in_file_in"
}
```
