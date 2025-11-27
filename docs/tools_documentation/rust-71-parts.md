# Crate: rust-71-parts

## Description
This crate serves as a foundational library and hosts numerous binaries for a "71-Part Monster Group `rustc` Decomposition." It contains modules for AST extraction, token lattices, token constants, Hecke operators, Monster levels, and AST transport, all designed to understand and categorize the Rust compiler's components according to the Monster Group theory, particularly leveraging the prime 71.

## Usage
**Library (`rust-71-parts`):** Intended to be used as a dependency by other Rust projects that require deep analysis or manipulation of Rust code structures based on the Monster Group framework. It exposes modules such as `ast_extractor`, `token_lattice`, `token_constants`, `hecke_operators`, and `monster_levels`.
**Binaries:** This crate defines 19 separate executable binaries (e.g., `monster_cloudformation_generator`, `automorphic_orbit`, `ingest_rustc`) under `src/bin/`. Each binary likely serves a specific purpose related to the overall "71-Part Monster Group `rustc` Decomposition" vision.

## Dependencies
- `quote` ("1.0")
- `proc-macro2` ("1.0")
- `sha2` ("0.10")
- `hex` ("0.4")
- `bincode` ("1.3")
- `serde_json` ("1.0")
- `syn` ("2.0", with `full`, `extra-traits` features)
- `serde` ("1.0", with `derive` feature)
- `chrono` ("0.4", with `serde` feature)
- `flate2` ("1.0")
- `rocksdb` ("0.22")
- `juniper` ("0.16")
- `tempfile` ("3.0")

## Notes
The crate's name, `rust-71-parts`, directly references the highest prime factor of the Monster Group's order, indicating its deep integration with the project's philosophical and mathematical underpinnings. The extensive list of binaries and features (`part_01` to `part_71`) suggests a highly modular and granular approach to analyzing and transforming Rust code. It appears to be a central hub for the project's meta-programming and formal verification efforts related to the Rust compiler. The internal constant `MONSTER_PRIME: u64 = 71;` further solidifies this connection.