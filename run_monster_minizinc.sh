#!/bin/bash

# Monster Group Trait Verification Runner
# Purpose: Solve Monster Group constraints for Rust trait mappings

set -euo pipefail

MINIZINC_EXECUTABLE="minizinc"
MODEL_PATH="monster_traits.mzn"
DATA_FILE="trait_data.dzn"
OUTPUT_LOG="monster_output.log"

echo "--- Starting Monster Group trait verification ---"

# Generate trait data if not exists
if [ ! -f "${DATA_FILE}" ]; then
    echo "Generating trait data..."
    cargo run --bin generate_trait_data
fi

# Run MiniZinc solver
"${MINIZINC_EXECUTABLE}" --solver Gecode \
    "${MODEL_PATH}" \
    "${DATA_FILE}" \
    > "${OUTPUT_LOG}" 2>&1

echo "--- Monster Group verification complete. Output in ${OUTPUT_LOG} ---"

# Verify solution
if grep -q "monster_elements" "${OUTPUT_LOG}"; then
    echo "✓ Monster Group solution found"
    grep "monster_elements\|hecke_values" "${OUTPUT_LOG}"
else
    echo "✗ No solution found"
    exit 1
fi
