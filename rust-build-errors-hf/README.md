---
license: mit
---

# Rust Build Errors Dataset

This repository contains a dataset of Rust build errors, including:
- `errors.jsonl`: Raw, line-delimited Cargo JSON diagnostics.
- `patches.toml`: Semantic patches used to fix the errors.
- `flakes/`: The Nix flakes that reproduce the failing builds.

This dataset is intended for training machine learning models to automatically suggest fixes for Rust compilation errors.
