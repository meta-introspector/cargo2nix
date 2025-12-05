#!/usr/bin/env bash

# This script generates Cargo.toml patch overrides for all submodules.

SUBMODULES_DIR="/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules"

echo "[patch.crates-io]"

for dir in "$SUBMODULES_DIR"/*; do
  if [ -d "$dir" ]; then
    crate_name=$(basename "$dir")
    # Exclude .logs directory
    if [ "$crate_name" != ".logs" ]; then
      echo "$crate_name = { path = \"submodules/$crate_name\" }"
    fi
  fi
done