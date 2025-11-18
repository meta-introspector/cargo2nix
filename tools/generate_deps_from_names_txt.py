import re
import os
import sys # Import sys module

def generate_deps_from_names_txt(names_txt_path, project_root):
    # Stores { crate_name: (path_string, path_length) }
    candidate_entries = {}
    
    with open(names_txt_path, 'r') as f:
        for line in f:
            line = line.strip()
            if not line:
                continue

            # New parsing for lines like "package_name,directory_path"
            parts = line.split(',', 1) # Split only on the first comma
            if len(parts) != 2:
                continue
            
            crate_name = parts[0].strip()
            directory_path = parts[1].strip()

            # The path to use for the 'path' attribute in Cargo.toml
            crate_path_for_toml = directory_path

            # 1. Filter by 'submodules/' prefix (if still desired, as filtered_packages.txt might contain other paths)
            # Assuming filtered_packages.txt might still contain non-submodule paths, keep this filter.
            if not crate_path_for_toml.startswith("submodules/"):
                continue

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

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python generate_deps_from_names_txt.py <input_file_path>")
        sys.exit(1)
    
    names_txt_path = sys.argv[1]
    project_root = "/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix" # This can also be passed as an argument if needed
    
    generated_dependencies = generate_deps_from_names_txt(names_txt_path, project_root)
    print(generated_dependencies)