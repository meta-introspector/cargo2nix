#!/nix/store/a65r9k46dxhyx2gn60bpx7j62anjdjr7-python3-3.13.7/bin/python
import re
import os
import sys

def find_non_vendored_modules(tree_file, cargo_lock_file):
    module_usage_counts = {}
    
    # Regex to extract package name and path from any dependency line in tree.txt
    package_path_regex = re.compile(r'^[│\s]*(?:├──|└──)?\s*(\w[\w-]*)\s+v\S+(?:\s+\(([^)]+)\))?')

    sys.stderr.write(f"Processing tree file: {tree_file}\n")

    with open(tree_file, 'r') as f:
        for line in f:
            sys.stderr.write(f"Line: {line.strip()}\n")
            match = package_path_regex.search(line)
            if match:
                package_name = match.group(1)
                package_path = match.group(2) # This will be None if no path in parentheses

                sys.stderr.write(f"  Extracted: name='{package_name}', path='{package_path}'\n")

                is_vendored = False
                if package_path and "/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/" in package_path:
                    is_vendored = True
                
                sys.stderr.write(f"  Is vendored: {is_vendored}\n")

                if not is_vendored:
                    module_usage_counts[package_name] = module_usage_counts.get(package_name, 0) + 1
                    sys.stderr.write(f"  Incremented usage for: {package_name}\n")
            else:
                sys.stderr.write(f"  No match found for line.\n")
    
    sys.stderr.write(f"Final non-vendored module usage counts: {module_usage_counts}\n")
                    

    # Print module names and their usage counts, one per line, if any are found
    if module_usage_counts:
        for module, count in module_usage_counts.items():
            print(f"{module} {count}")

# Define file paths
tree_file = '/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/tree.txt'
cargo_lock_file = '/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/Cargo.lock'
script_path = '/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/find_non_vendored.py'

# Call the function
if __name__ == "__main__":
    find_non_vendored_modules(tree_file, cargo_lock_file)
