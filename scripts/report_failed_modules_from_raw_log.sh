#!/bin/bash

RAW_LOG_FILE="build_logs/cargo2nix_build_raw.log"

if [ ! -f "$RAW_LOG_FILE" ]; then
    echo "Error: Raw log file '$RAW_LOG_FILE' not found. Please run 'make main-report' first to generate it."
    exit 1
fi

echo "--- Analyzing '$RAW_LOG_FILE' for failed modules ---"

grep '"level":"error"' "$RAW_LOG_FILE" | \
grep '"reason":"compiler-message"' | \
grep -oP '"package_id":"\K[^" ]+' | \
cut -d'-' -f1 | \
sort -u

echo "----------------------------------------------------"
