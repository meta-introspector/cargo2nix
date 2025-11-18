#!/nix/store/hxmi7d6vbdgbzklm4icfk2y83ncw8la9-bash-5.3p3/bin/bash

echo "Searching for cargo2nix executables in Nix store..."

# Get all cargo2nix derivations
derivations=$(ls /nix/store/ | grep cargo2nix | grep "\.drv$")

found_executables=()

for drv in $derivations; do
    drv_path="/nix/store/$drv"
    echo "Inspecting derivation: $drv_path"

    # Query the outputs of the derivation
    outputs=$(nix-store -q --outputs "$drv_path" 2>/dev/null)

    for output_path in $outputs; do
        executable_path="$output_path/bin/cargo2nix"
        if [ -f "$executable_path" ] && [ -x "$executable_path" ]; then
            echo "Found executable: $executable_path"
            found_executables+=("$executable_path")
        fi
    done
done

if [ ${#found_executables[@]} -eq 0 ]; then
    echo "No cargo2nix executables found in Nix store."
else
    echo "--- Found cargo2nix executables ---"
    for exec_path in "${found_executables[@]}"; do
        echo "$exec_path"
    done
    echo "-----------------------------------"
fi