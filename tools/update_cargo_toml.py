import re
import os
import subprocess

def generate_deps_from_names_txt(names_txt_path, project_root):
    # Stores { crate_name: (path_string, path_length) }
    candidate_entries = {}
    
    with open(names_txt_path, 'r') as f:
        for line in f:
            line = line.strip()
            if not line:
                continue

            match = re.match(r'(.+?)/Cargo.toml:name = "(.+?)"', line)
            if not match:
                continue

            cargo_toml_path_relative_to_project_root = match.group(1)
            crate_name = match.group(2)

            # 1. Filter by 'submodules/' prefix
            if not cargo_toml_path_relative_to_project_root.startswith("submodules/"):
                continue

            # The path to use for the 'path' attribute in Cargo.toml
            crate_path_for_toml = cargo_toml_path_relative_to_project_root

            # 2. Calculate Depth relative to 'submodules/'
            # Split by '/' and count segments after 'submodules/'
            path_segments = crate_path_for_toml.split('/')
            # The path starts with "submodules/", so the actual crate directory starts after that.
            # e.g., "submodules/foo" -> ["submodules", "foo"] -> depth 1
            # "submodules/foo/bar" -> ["submodules", "foo", "bar"] -> depth 2
            depth = len(path_segments) - 1 # Subtract 1 for "submodules" segment itself

            # 3. Apply Max Depth Filter (max 4 directories after 'submodules/')
            if depth > 4:
                continue

            current_path_length = len(crate_path_for_toml)

            # 4. Duplicate Resolution: choose the shorter path
            if crate_name in candidate_entries:
                existing_path, existing_path_length = candidate_entries[crate_name]
                if current_path_length < existing_path_length:
                    candidate_entries[crate_name] = (crate_path_for_toml, current_path_length)
            else:
                candidate_entries[crate_name] = (crate_path_for_toml, current_path_length)

    # Sort entries by crate name and format for Cargo.toml
    sorted_crate_names = sorted(candidate_entries.keys())
    
    generated_deps = []
    for crate_name in sorted_crate_names:
        path_string, _ = candidate_entries[crate_name]
        generated_deps.append(f'{crate_name} = {{ path = "{path_string}" }}')

    return "\n".join(generated_deps)

def update_cargo_toml(cargo_toml_path, generated_deps_content):
    with open(cargo_toml_path, 'r') as f:
        lines = f.readlines()

    before_deps = []
    after_deps = []
    in_deps_section = False
    
    for line in lines:
        if line.strip() == "[workspace.dependencies]":
            in_deps_section = True
            # We don't add this line to before_deps because we'll add it explicitly later
            continue
        
        if in_deps_section:
            # Check if we've hit another section header
            if line.strip().startswith("[") and line.strip().endswith("]"):
                in_deps_section = False
                after_deps.append(line)
            else:
                # Skip existing dependencies, they will be replaced
                continue
        else:
            before_deps.append(line)

    new_content = "".join(before_deps)
    new_content += "[workspace.dependencies]\n"
    new_content += generated_deps_content
    new_content += "\n" # Add a newline after the generated dependencies for readability
    new_content += "".join(after_deps) # In this case, after_deps will be empty if [workspace.dependencies] was the last section

    return new_content

if __name__ == "__main__":
    names_txt_path = "/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/names.txt"
    project_root = "/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix"
    cargo_toml_path = "/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/Cargo.toml"
    temp_cargo_toml_path = "/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/Cargo.toml.tmp"

    # Generate the new dependencies content using the refined logic
    generated_deps_content = generate_deps_from_names_txt(names_txt_path, project_root)

    # Update the Cargo.toml content
    updated_cargo_toml_content = update_cargo_toml(cargo_toml_path, generated_deps_content)

    # Write to a temporary file
    with open(temp_cargo_toml_path, 'w') as f:
        f.write(updated_cargo_toml_content)

    print(f"Updated content written to {temp_cargo_toml_path}. Please review the diff before applying.")
