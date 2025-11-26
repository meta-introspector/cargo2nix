# Recursive Git URL Analysis Report

## Summary
- Rustc crates analyzed: 294
- Git URLs found: 12
- Existing submodules: 536
- Missing git URLs: 11
- Crates processed recursively: 609
- Submodule coverage: 8.3%

## Missing Git URLs
These repositories need to be added as submodules:

- https://github.com/rust-lang/annotate-snippets-rs
- https://github.com/marshallpierce/rust-base64
- https://github.com/bright-shard/boml
- https://github.com/BLAKE3-team/BLAKE3
- https://github.com/IvanUkhov/c
- https://github.com/rust-lang/backtrace-rs
- https://github.com/rust-cli/anstyle.git
- https://github.com/RustCrypto/block-ciphers
- https://github.com/askama-rs/askama
- https://github.com/llogiq/bytecount
- https://github.com/DanielKeep/rust-build-helper

## Next Steps
1. Add missing repositories as git submodules
2. Update Monster Protocol to use complete dependency tree
3. Re-run recursive analysis to verify coverage
