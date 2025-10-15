#!/usr/bin/env bash
nix develop --command cargo run -- -vvv --file Cargo.nix --overwrite 2>&1 | tee build_output.log