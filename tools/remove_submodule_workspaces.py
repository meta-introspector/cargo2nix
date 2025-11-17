import os
import re

def remove_workspace_section(cargo_toml_path):
    with open(cargo_toml_path, 'r') as f:
        lines = f.readlines()

    new_lines = []
    in_workspace_section = False
    changed = False

    for line in lines:
        if line.strip() == '[workspace]':
            in_workspace_section = True
            changed = True
            print(f"  Found [workspace] section in {cargo_toml_path}. Removing.")
            continue
        
        if in_workspace_section:
            # If we encounter another section header, we're out of the [workspace] section
            if line.strip().startswith('[') and line.strip().endswith(']'):
                in_workspace_section = False
                new_lines.append(line)
            elif not line.strip(): # Keep blank lines within the removed section
                continue
            else: # Skip lines within the [workspace] section
                continue
        else:
            new_lines.append(line)
    
    if changed:
        with open(cargo_toml_path, 'w') as f:
            f.writelines(new_lines)
        print(f"  Successfully removed [workspace] section from {cargo_toml_path}")
    else:
        print(f"  No [workspace] section found in {cargo_toml_path}")

def main():
    root_dir = "/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix"
    submodules_dir = os.path.join(root_dir, "submodules")

    print(f"Scanning for Cargo.toml files in submodules under: {submodules_dir}")

    for dirpath, dirnames, filenames in os.walk(submodules_dir):
        for filename in filenames:
            if filename == "Cargo.toml":
                cargo_toml_path = os.path.join(dirpath, filename)
                print(f"Processing {cargo_toml_path}...")
                remove_workspace_section(cargo_toml_path)

if __name__ == "__main__":
    main()
