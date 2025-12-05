#!/bin/bash
set -e

echo "=== Ensuring CRQ-016-nixify branches ==="

# Main repo
echo "1. Main repo:"
if git show-branch CRQ-016-nixify 2>/dev/null; then
    git checkout CRQ-016-nixify
    echo "  ✓ Switched to existing CRQ-016-nixify"
else
    git checkout -b CRQ-016-nixify
    echo "  ✓ Created CRQ-016-nixify from $(git branch --show-current)"
fi

# Submodules in topological order
SUBMODULES=(
    "submodules/allocator-api2"
    "submodules/cargo" 
    "submodules/rust-base64"
    "submodules/gitoxide"
)

for submod in "${SUBMODULES[@]}"; do
    if [ -d "$submod" ]; then
        echo "2. $submod:"
        cd "$submod"
        
        if git show-branch CRQ-016-nixify 2>/dev/null; then
            git checkout CRQ-016-nixify
            echo "  ✓ Switched to existing CRQ-016-nixify"
        else
            current=$(git branch --show-current)
            git checkout -b CRQ-016-nixify
            echo "  ✓ Created CRQ-016-nixify from $current"
        fi
        cd - > /dev/null
    else
        echo "  ✗ Missing: $submod"
    fi
done

# Check missing Rust cargos
echo "3. Missing Rust cargos:"
./enhanced_cargo_mapper | grep -E "std|core|alloc|rustc" | head -5 || echo "  Run enhanced_cargo_mapper to identify missing"

echo "=== Branch setup complete ==="
