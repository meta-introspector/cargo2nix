#!/usr/bin/env bash

echo "Running nix build -vv..."
nix build -vvv > nix_build_output.log 2>&1
BUILD_EXIT_CODE=$?

if [ $BUILD_EXIT_CODE -eq 0 ]; then
    echo "nix build completed successfully. Output saved to nix_build_output.log"
else
    echo "nix build failed with exit code $BUILD_EXIT_CODE. Output saved to nix_build_output.log"
    echo "Error details:"
    tail -n 20 nix_build_output.log # Show last 20 lines of the log for quick inspection
fi

exit $BUILD_EXIT_CODE
