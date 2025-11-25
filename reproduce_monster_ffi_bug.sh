#!/bin/bash

set -e

PROJECT_ROOT="/mnt/data1/nix/vendor/rust/cargo2nix"
cd "$PROJECT_ROOT"

echo "Building Monster Group C++ FFI..."

# Compile C++ FFI library
g++ -shared -fPIC -o libmonster_ffi.so src/monster_ffi.cpp -I./include

echo "Setting up library path..."
export LD_LIBRARY_PATH="$PROJECT_ROOT:$LD_LIBRARY_PATH"

echo "Running Monster Group FFI tests..."
cargo test monster_ffi -- --nocapture

echo "Testing Monster Group verification..."
cargo run --bin test_monster_verification

echo "Monster Group FFI tests finished."
