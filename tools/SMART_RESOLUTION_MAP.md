# Smart Crate Resolution Map

## Use Existing Rust Source
Source: `/mnt/data1/nix/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src`

- `rustc_apfloat` → Use existing rustc source

## Add External Submodules Only
- `annotate-snippets` → Add as submodule
- `blake3` → Add as submodule
- `boml` → Add as submodule
- `cranelift-codegen` → Add as submodule
- `cranelift-frontend` → Add as submodule
- `cranelift-jit` → Add as submodule
- `cranelift-module` → Add as submodule
- `cranelift-native` → Add as submodule
- `cranelift-object` → Add as submodule
- `crossbeam-deque` → Add as submodule
- `crossbeam-utils` → Add as submodule
- `derive-where` → Add as submodule
- `derive_setters` → Add as submodule
- `elsa` → Add as submodule
- `ena` → Add as submodule
- `fluent-bundle` → Add as submodule
- `fluent-syntax` → Add as submodule
- `gccjit` → Add as submodule
- `getopts` → Add as submodule
- `gsgdt` → Add as submodule
- `icu_list` → Add as submodule
- `icu_locid` → Add as submodule
- `icu_locid_transform` → Add as submodule
- `icu_provider` → Add as submodule
- `icu_provider_adapters` → Add as submodule
- `intl-memoizer` → Add as submodule
- `jobserver_crate` → Add as submodule
- `libloading` → Add as submodule
- `measureme` → Add as submodule
- `odht` → Add as submodule
- `polonius-engine` → Add as submodule
- `rand_xoshiro` → Add as submodule
- `rustc-literal-escaper` → Add as submodule
- `serde_json` → Add as submodule
- `sha1` → Add as submodule
- `sha2` → Add as submodule
- `shlex` → Add as submodule
- `stable_mir` → Add as submodule
- `target-lexicon` → Add as submodule
- `termize` → Add as submodule
- `thin-vec` → Add as submodule
- `thorin-dwp` → Add as submodule
- `tracing-core` → Add as submodule
- `tracing-subscriber` → Add as submodule
- `tracing-tree` → Add as submodule
- `unic-langid` → Add as submodule
- `unicode-security` → Add as submodule
- `wasm-encoder` → Add as submodule
- `zerovec` → Add as submodule

## Resolution Strategy
1. **DON'T** duplicate rustc source
2. **USE** existing rust-src for all rustc_* crates
3. **ADD** only external dependencies as submodules
4. **APPLY** Monster Protocol to unified source
