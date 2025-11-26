# Complete Dependency Pipeline Report

## Pipeline Flow
rustc → crate → cargo metadata → repo → forks → branches → submodules → cargo.toml → package name → git object → database entry → deps

## Complete Dependency Entries

### boml
- **Rustc Crate**: `boml`
- **Cargo Metadata**: ✅
- **Repo URL**: `https://github.com/bright-shard/boml`

### c
- **Rustc Crate**: `c`
- **Cargo Metadata**: ✅
- **Repo URL**: `https://github.com/IvanUkhov/c`
- **Fork URL**: `https://github.com/meta-introspector/autocfg.git`
- **Branch**: `master	HEAD`
- **Submodule Path**: `submodules/autocfg`
- **Cargo.toml**: `../submodules/BLAKE3/c/blake3_c_rust_bindings/Cargo.toml`
- **Package Name**: `blake3_c_rust_bindings`
- **Git Object**: `d912169ed67977efe5a465269b0e73cb66060c49`

### backtrace
- **Rustc Crate**: `backtrace`
- **Cargo Metadata**: ✅
- **Repo URL**: `https://github.com/rust-lang/backtrace-rs`
- **Fork URL**: `https://github.com/meta-introspector/backtrace-rs`
- **Branch**: `master	HEAD`
- **Submodule Path**: `submodules/backtrace-rs`
- **Git Object**: `f2bcf65c7830b4e2cf8495553b20afd305986a51`

### anyhow.workspace
- **Rustc Crate**: `anyhow.workspace`
- **Cargo Metadata**: ❌
- **Fork URL**: `https://github.com/meta-introspector/anyhow.git`
- **Branch**: `master	HEAD`
- **Submodule Path**: `submodules/anyhow`
- **Git Object**: `eac70d7161109734d060e094d87cb2bd11212a35`

### base-db.workspace
- **Rustc Crate**: `base-db.workspace`
- **Cargo Metadata**: ❌

### anstyle-svg
- **Rustc Crate**: `anstyle-svg`
- **Cargo Metadata**: ✅
- **Repo URL**: `https://github.com/rust-cli/anstyle.git`
- **Fork URL**: `https://github.com/meta-introspector/anstyle`
- **Branch**: `main	HEAD`
- **Submodule Path**: `submodules/anstyle`
- **Git Object**: `368a8719474357f9d54be800b4163c9074561e37`

### blake3
- **Rustc Crate**: `blake3`
- **Cargo Metadata**: ✅
- **Repo URL**: `https://github.com/BLAKE3-team/BLAKE3`
- **Fork URL**: `https://github.com/meta-introspector/BLAKE3.git`
- **Branch**: `master	HEAD`
- **Submodule Path**: `submodules/BLAKE3`
- **Cargo.toml**: `../submodules/BLAKE3/Cargo.toml`
- **Package Name**: `blake3`
- **Git Object**: `eae9bf376a1c4797df7be6e49e735c0a5d91dcb0`
- **Dependencies**: 9

### annotate-snippets
- **Rustc Crate**: `annotate-snippets`
- **Cargo Metadata**: ✅
- **Repo URL**: `https://github.com/rust-lang/annotate-snippets-rs`
- **Fork URL**: `https://github.com/meta-introspector/annotate-snippets-rs`
- **Branch**: `master	HEAD`
- **Submodule Path**: `submodules/annotate-snippets-rs`
- **Git Object**: `ed71e38bde440098dc58077e3224e816ba4a54d0`

### askama
- **Rustc Crate**: `askama`
- **Cargo Metadata**: ✅
- **Repo URL**: `https://github.com/askama-rs/askama`

### base64
- **Rustc Crate**: `base64`
- **Cargo Metadata**: ✅
- **Repo URL**: `https://github.com/marshallpierce/rust-base64`
- **Fork URL**: `https://github.com/meta-introspector/rust-base64`
- **Branch**: `master	HEAD`
- **Submodule Path**: `submodules/rust-base64`
- **Git Object**: `cac5ff84cd771b1a9f52da020b053b35f0ff3ede`

### bitflags.workspace
- **Rustc Crate**: `bitflags.workspace`
- **Cargo Metadata**: ❌
- **Fork URL**: `https://github.com/meta-introspector/bitflags.git`
- **Branch**: `main	HEAD`
- **Submodule Path**: `submodules/bitflags`
- **Git Object**: `7cc8595e93d04d180d39e2f25242dca85dd71228`

### bytecount
- **Rustc Crate**: `bytecount`
- **Cargo Metadata**: ✅
- **Repo URL**: `https://github.com/llogiq/bytecount`

### ansi_term
- **Rustc Crate**: `ansi_term`
- **Cargo Metadata**: ✅
- **Repo URL**: `https://github.com/ogham/rust-ansi-term`

### camino.workspace
- **Rustc Crate**: `camino.workspace`
- **Cargo Metadata**: ❌
- **Fork URL**: `https://github.com/meta-introspector/camino.git`
- **Branch**: `main	HEAD`
- **Submodule Path**: `submodules/camino`
- **Git Object**: `73c7a362a86cb3bc31f0c736973515f5ad056b63`

### aes
- **Rustc Crate**: `aes`
- **Cargo Metadata**: ✅
- **Repo URL**: `https://github.com/RustCrypto/block-ciphers`

### build_helper
- **Rustc Crate**: `build_helper`
- **Cargo Metadata**: ❌

### anstream
- **Rustc Crate**: `anstream`
- **Cargo Metadata**: ✅
- **Repo URL**: `https://github.com/rust-cli/anstyle.git`

### cargo_metadata.workspace
- **Rustc Crate**: `cargo_metadata.workspace`
- **Cargo Metadata**: ❌
- **Fork URL**: `https://github.com/oli-obk/cargo_metadata`
- **Branch**: `main	HEAD`
- **Submodule Path**: `submodules/cargo_metadata`
- **Git Object**: `c08e66cdf534313085ef810ce6f2e0df8a83fc50`

### arrayvec.workspace
- **Rustc Crate**: `arrayvec.workspace`
- **Cargo Metadata**: ❌
- **Fork URL**: `https://github.com/meta-introspector/arrayvec.git`
- **Branch**: `master	HEAD`
- **Submodule Path**: `submodules/arrayvec`
- **Git Object**: `812c83a2b16c7d97c5e5250ebfd785da4e089895`

### byteorder_2
- **Rustc Crate**: `byteorder_2`
- **Cargo Metadata**: ❌
- **Fork URL**: `https://github.com/meta-introspector/byteorder.git`
- **Branch**: `master	HEAD`
- **Submodule Path**: `submodules/byteorder`
- **Git Object**: `5a82625fae462e8ba64cec8146b24a372b4d75c6`

## Pipeline Statistics
- Total rustc crates: 20
- With cargo metadata: 12
- With submodules: 12
- With Cargo.toml: 2
- Complete pipeline: 2
- Completion rate: 10.0%
