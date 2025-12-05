# Cargo Module Dependency Mapping Report

## Dependency Relationship Flow
cargo module < uses <- cargo module

## Cargo Module Dependencies

### bench (juniper)
- **Git Module**: `juniper`
- **Cargo.toml**: `submodules/juniper/juniper/Cargo.toml`
- **Uses 40 dependencies**:
  - `anyhow` < **uses** <- `bench` (1.0.47)
  - `arcstr` < **uses** <- `bench` (1.1)
  - `async-trait` < **uses** <- `bench` (0.1.39) [crates.io]
  - `auto_enums` < **uses** <- `bench` (0.8) [crates.io]
  - `bigdecimal` < **uses** <- `bench` (0.4)
  - `bson` < **uses** <- `bench` (3.0)
  - `chrono` < **uses** <- `bench` (0.4.30)
  - `chrono-tz` < **uses** <- `bench` (0.10)
  - `compact_str` < **uses** <- `bench` (0.9) [crates.io]
  - `derive_more` < **uses** <- `bench` (2.0)
  - ... and 30 more dependencies

### blake3 (BLAKE3)
- **Git Module**: `BLAKE3`
- **Cargo.toml**: `submodules/BLAKE3/Cargo.toml`
- **Uses 19 dependencies**:
  - `arrayref` < **uses** <- `blake3` (0.3.5) [crates.io]
  - `arrayvec` < **uses** <- `blake3` (0.7.4)
  - `constant_time_eq` < **uses** <- `blake3` (0.3.1)
  - `cfg-if` < **uses** <- `blake3` (1.0.0) [crates.io]
  - `digest` < **uses** <- `blake3` (0.10.1)
  - `memmap2` < **uses** <- `blake3` (0.9)
  - `rayon-core` < **uses** <- `blake3` (1.12.1)
  - `serde` < **uses** <- `blake3` (1.0)
  - `zeroize` < **uses** <- `blake3` (1)
  - `hmac` < **uses** <- `blake3` (0.12.0) [crates.io]
  - ... and 9 more dependencies
- **Used by 2 modules**:
  - `blake3` < **uses** <- `b3sum`
  - `blake3` < **uses** <- `test_vectors`

### subscription (juniper)
- **Git Module**: `juniper`
- **Cargo.toml**: `submodules/juniper/juniper_actix/Cargo.toml`
- **Uses 19 dependencies**:
  - `actix-web` < **uses** <- `subscription` (4.4) [crates.io]
  - `actix-ws` < **uses** <- `subscription` (0.3)
  - `derive_more` < **uses** <- `subscription` (2.0)
  - `futures` < **uses** <- `subscription` (0.3.22)
  - `juniper` < **uses** <- `subscription` (0.17) [path]
  - `juniper_graphql_ws` < **uses** <- `subscription` (0.5) [path]
  - `serde` < **uses** <- `subscription` (1.0.122)
  - `serde_json` < **uses** <- `subscription` (1.0.18) [crates.io]
  - `actix-cors` < **uses** <- `subscription` (0.7) [crates.io]
  - `actix-http` < **uses** <- `subscription` (3.2) [crates.io]
  - ... and 9 more dependencies

### ws_test_suite (juniper)
- **Git Module**: `juniper`
- **Cargo.toml**: `submodules/juniper/juniper_axum/Cargo.toml`
- **Uses 19 dependencies**:
  - `axum` < **uses** <- `ws_test_suite` (0.8)
  - `derive_more` < **uses** <- `ws_test_suite` (2.0)
  - `futures` < **uses** <- `ws_test_suite` (0.3.22)
  - `juniper` < **uses** <- `ws_test_suite` (0.17) [path]
  - `juniper_graphql_ws` < **uses** <- `ws_test_suite` (0.5) [path]
  - `serde` < **uses** <- `ws_test_suite` (1.0.122)
  - `serde_json` < **uses** <- `ws_test_suite` (1.0.18) [crates.io]
  - `bytes` < **uses** <- `ws_test_suite` (1.2) [crates.io]
  - `anyhow` < **uses** <- `ws_test_suite` (1.0) [crates.io]
  - `axum` < **uses** <- `ws_test_suite` (0.8.1)
  - ... and 9 more dependencies

### juniper_hyper (juniper)
- **Git Module**: `juniper`
- **Cargo.toml**: `submodules/juniper/juniper_hyper/Cargo.toml`
- **Uses 13 dependencies**:
  - `derive_more` < **uses** <- `juniper_hyper` (2.0)
  - `http-body-util` < **uses** <- `juniper_hyper` (0.1) [crates.io]
  - `hyper` < **uses** <- `juniper_hyper` (1.0)
  - `juniper` < **uses** <- `juniper_hyper` (0.17) [path]
  - `serde_json` < **uses** <- `juniper_hyper` (1.0.18) [crates.io]
  - `url` < **uses** <- `juniper_hyper` (2.0) [crates.io]
  - `hyper` < **uses** <- `juniper_hyper` (1.0)
  - `hyper-util` < **uses** <- `juniper_hyper` (0.1)
  - `juniper` < **uses** <- `juniper_hyper` (0.17) [path]
  - `log` < **uses** <- `juniper_hyper` (0.4) [crates.io]
  - ... and 3 more dependencies

### juniper_codegen (juniper)
- **Git Module**: `juniper`
- **Cargo.toml**: `submodules/juniper/juniper_codegen/Cargo.toml`
- **Uses 10 dependencies**:
  - `derive_more` < **uses** <- `juniper_codegen` (2.0)
  - `proc-macro2` < **uses** <- `juniper_codegen` (1.0.4) [crates.io]
  - `quote` < **uses** <- `juniper_codegen` (1.0.9) [crates.io]
  - `syn` < **uses** <- `juniper_codegen` (2.0)
  - `url` < **uses** <- `juniper_codegen` (2.0) [crates.io]
  - `derive_more` < **uses** <- `juniper_codegen` (2.0)
  - `futures` < **uses** <- `juniper_codegen` (0.3.22) [crates.io]
  - `jiff` < **uses** <- `juniper_codegen` (0.2)
  - `juniper` < **uses** <- `juniper_codegen` [path]
  - `serde` < **uses** <- `juniper_codegen` (1.0.122) [crates.io]
- **Used by 1 modules**:
  - `juniper_codegen` < **uses** <- `bench`

### juniper_rocket (juniper)
- **Git Module**: `juniper`
- **Cargo.toml**: `submodules/juniper/juniper_rocket/Cargo.toml`
- **Uses 10 dependencies**:
  - `derive_more` < **uses** <- `juniper_rocket` (2.0)
  - `juniper` < **uses** <- `juniper_rocket` (0.17) [path]
  - `rocket` < **uses** <- `juniper_rocket` (0.5)
  - `serde_json` < **uses** <- `juniper_rocket` (1.0.18) [crates.io]
  - `either` < **uses** <- `juniper_rocket` (1.8) [crates.io]
  - `inlinable_string` < **uses** <- `juniper_rocket` (0.1.15) [crates.io]
  - `pear` < **uses** <- `juniper_rocket` (0.2.4) [crates.io]
  - `tempfile` < **uses** <- `juniper_rocket` (3.3) [crates.io]
  - `futures` < **uses** <- `juniper_rocket` (0.3.22) [crates.io]
  - `juniper` < **uses** <- `juniper_rocket` (0.17) [path]

### juniper_integration_tests (juniper)
- **Git Module**: `juniper`
- **Cargo.toml**: `submodules/juniper/tests/integration/Cargo.toml`
- **Uses 10 dependencies**:
  - `chrono` < **uses** <- `juniper_integration_tests` (0.4)
  - `derive_more` < **uses** <- `juniper_integration_tests` (2.0)
  - `futures` < **uses** <- `juniper_integration_tests` (0.3) [crates.io]
  - `itertools` < **uses** <- `juniper_integration_tests` (0.14) [crates.io]
  - `juniper` < **uses** <- `juniper_integration_tests` [path]
  - `juniper_subscriptions` < **uses** <- `juniper_integration_tests` [path]
  - `serde` < **uses** <- `juniper_integration_tests` (1.0)
  - `serde_json` < **uses** <- `juniper_integration_tests` (1.0) [crates.io]
  - `tokio` < **uses** <- `juniper_integration_tests` (1.0)
  - `smartstring` < **uses** <- `juniper_integration_tests` (1.0) [crates.io]

### b3sum (BLAKE3)
- **Git Module**: `BLAKE3`
- **Cargo.toml**: `submodules/BLAKE3/b3sum/Cargo.toml`
- **Uses 8 dependencies**:
  - `anyhow` < **uses** <- `b3sum` (1.0.25) [crates.io]
  - `blake3` < **uses** <- `b3sum` (1.8) [path]
  - `clap` < **uses** <- `b3sum` (4.0.8)
  - `hex` < **uses** <- `b3sum` (0.4.0) [crates.io]
  - `rayon-core` < **uses** <- `b3sum` (1.12.1) [crates.io]
  - `wild` < **uses** <- `b3sum` (2.0.3) [crates.io]
  - `duct` < **uses** <- `b3sum` (1.0.0) [crates.io]
  - `tempfile` < **uses** <- `b3sum` (3.1.0) [crates.io]

### juniper_book (juniper)
- **Git Module**: `juniper`
- **Cargo.toml**: `submodules/juniper/book/Cargo.toml`
- **Uses 8 dependencies**:
  - `anyhow` < **uses** <- `juniper_book` (1.0) [crates.io]
  - `dataloader` < **uses** <- `juniper_book` (0.18) [crates.io]
  - `derive_more` < **uses** <- `juniper_book` (2.0)
  - `jiff` < **uses** <- `juniper_book` (0.2)
  - `juniper` < **uses** <- `juniper_book` [path]
  - `juniper_subscriptions` < **uses** <- `juniper_book` [path]
  - `serde_json` < **uses** <- `juniper_book` (1.0) [crates.io]
  - `tokio` < **uses** <- `juniper_book` (1.0)

### blake3_c_rust_bindings (BLAKE3)
- **Git Module**: `BLAKE3`
- **Cargo.toml**: `submodules/BLAKE3/c/blake3_c_rust_bindings/Cargo.toml`
- **Uses 8 dependencies**:
  - `arrayref` < **uses** <- `blake3_c_rust_bindings` (0.3.5) [crates.io]
  - `arrayvec` < **uses** <- `blake3_c_rust_bindings` (0.7.0)
  - `page_size` < **uses** <- `blake3_c_rust_bindings` (0.6.0) [crates.io]
  - `rand` < **uses** <- `blake3_c_rust_bindings` (0.9.0) [crates.io]
  - `rand_chacha` < **uses** <- `blake3_c_rust_bindings` (0.9.0) [crates.io]
  - `reference_impl` < **uses** <- `blake3_c_rust_bindings` [path]
  - `cc` < **uses** <- `blake3_c_rust_bindings` (1.0.48) [crates.io]
  - `ignore` < **uses** <- `blake3_c_rust_bindings` (0.4.23) [crates.io]

### juniper_graphql_ws (juniper)
- **Git Module**: `juniper`
- **Cargo.toml**: `submodules/juniper/juniper_graphql_ws/Cargo.toml`
- **Uses 6 dependencies**:
  - `derive_more` < **uses** <- `juniper_graphql_ws` (2.0)
  - `juniper` < **uses** <- `juniper_graphql_ws` (0.17) [path]
  - `juniper_subscriptions` < **uses** <- `juniper_graphql_ws` (0.18) [path]
  - `serde` < **uses** <- `juniper_graphql_ws` (1.0.122)
  - `tokio` < **uses** <- `juniper_graphql_ws` (1.0)
  - `serde_json` < **uses** <- `juniper_graphql_ws` (1.0.18) [crates.io]
- **Used by 2 modules**:
  - `juniper_graphql_ws` < **uses** <- `subscription`
  - `juniper_graphql_ws` < **uses** <- `ws_test_suite`

### juniper_codegen_tests (juniper)
- **Git Module**: `juniper`
- **Cargo.toml**: `submodules/juniper/tests/codegen/Cargo.toml`
- **Uses 6 dependencies**:
  - `rustversion` < **uses** <- `juniper_codegen_tests` (1.0) [crates.io]
  - `derive_more` < **uses** <- `juniper_codegen_tests` (2.0)
  - `futures` < **uses** <- `juniper_codegen_tests` (0.3) [crates.io]
  - `juniper` < **uses** <- `juniper_codegen_tests` [path]
  - `serde` < **uses** <- `juniper_codegen_tests` (1.0)
  - `trybuild` < **uses** <- `juniper_codegen_tests` (1.0.63) [crates.io]

### test_vectors (BLAKE3)
- **Git Module**: `BLAKE3`
- **Cargo.toml**: `submodules/BLAKE3/test_vectors/Cargo.toml`
- **Uses 5 dependencies**:
  - `blake3` < **uses** <- `test_vectors` [path]
  - `hex` < **uses** <- `test_vectors` (0.4.0) [crates.io]
  - `reference_impl` < **uses** <- `test_vectors` [path]
  - `serde` < **uses** <- `test_vectors` (1.0)
  - `serde_json` < **uses** <- `test_vectors` (1.0) [crates.io]

### juniper_subscriptions (juniper)
- **Git Module**: `juniper`
- **Cargo.toml**: `submodules/juniper/juniper_subscriptions/Cargo.toml`
- **Uses 4 dependencies**:
  - `futures` < **uses** <- `juniper_subscriptions` (0.3.22) [crates.io]
  - `juniper` < **uses** <- `juniper_subscriptions` (0.17) [path]
  - `serde_json` < **uses** <- `juniper_subscriptions` (1.0.18) [crates.io]
  - `tokio` < **uses** <- `juniper_subscriptions` (1.0)
- **Used by 3 modules**:
  - `juniper_subscriptions` < **uses** <- `juniper_graphql_ws`
  - `juniper_subscriptions` < **uses** <- `juniper_book`
  - `juniper_subscriptions` < **uses** <- `juniper_integration_tests`

### benchmark (juniper)
- **Git Module**: `juniper`
- **Cargo.toml**: `submodules/juniper/benches/Cargo.toml`
- **Uses 3 dependencies**:
  - `juniper` < **uses** <- `benchmark` [path]
  - `criterion` < **uses** <- `benchmark` (0.7) [crates.io]
  - `tokio` < **uses** <- `benchmark` (1.0)

### benches (diff)
- **Git Module**: `diff`
- **Cargo.toml**: `submodules/diff/Cargo.toml`
- **Uses 2 dependencies**:
  - `quickcheck` < **uses** <- `benches` (1.0.3) [crates.io]
  - `criterion` < **uses** <- `benches` (0.5.1) [crates.io]

### cast (cast)
- **Git Module**: `cast`
- **Cargo.toml**: `submodules/cast/Cargo.toml`
- **Uses 1 dependencies**:
  - `quickcheck` < **uses** <- `cast` (1.0.3) [crates.io]

### compiler_version (BLAKE3)
- **Git Module**: `BLAKE3`
- **Cargo.toml**: `submodules/BLAKE3/tools/compiler_version/Cargo.toml`
- **Uses 1 dependencies**:
  - `cc` < **uses** <- `compiler_version` (1.0.50) [crates.io]

### reference_impl (BLAKE3)
- **Git Module**: `BLAKE3`
- **Cargo.toml**: `submodules/BLAKE3/reference_impl/Cargo.toml`
- **Used by 3 modules**:
  - `reference_impl` < **uses** <- `test_vectors`
  - `reference_impl` < **uses** <- `blake3`
  - `reference_impl` < **uses** <- `blake3_c_rust_bindings`

## Dependency Statistics
- Total cargo modules: 22
- Modules with dependencies: 19
- Modules used by others: 5
- Total dependency relationships: 192
- Average dependencies per module: 10.1

## Most Dependent Modules
1. **bench** - 40 dependencies
2. **blake3** - 19 dependencies
3. **subscription** - 19 dependencies
4. **ws_test_suite** - 19 dependencies
5. **juniper_hyper** - 13 dependencies
6. **juniper_codegen** - 10 dependencies
7. **juniper_rocket** - 10 dependencies
8. **juniper_integration_tests** - 10 dependencies
9. **b3sum** - 8 dependencies
10. **juniper_book** - 8 dependencies

## Most Used Modules
1. **reference_impl** - used by 3 modules
2. **juniper_subscriptions** - used by 3 modules
3. **juniper_graphql_ws** - used by 2 modules
4. **blake3** - used by 2 modules
5. **juniper_codegen** - used by 1 modules
