#!/nix/store/a65r9k46dxhyx2gn60bpx7j62anjdjr7-python3-3.13.7/bin/python
import re
import sys

def extract_all_module_names(tree_file):
    module_names = set()
    # Regex to extract package name from any dependency line in tree.txt
    # It handles lines with or without the '─' prefix and with or without a path in parentheses
    package_name_regex = re.compile(r'(?:[─│ ]*\s*)?([a-zA-Z0-9_-]+)\s+v\S+(?:\s+\([^)]+\))?')

    with open(tree_file, 'r') as f:
        for line in f:
            match = package_name_regex.search(line)
            if match:
                module_name = match.group(1)
                # Exclude special entries like "proc-macro" or "*"
                if module_name not in ["proc-macro", "*"]:
                    module_names.add(module_name)
    return sorted(list(module_names))

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python extract_all_module_names.py <tree_file>")
        sys.exit(1)

    tree_file = sys.argv[1]
    names = extract_all_module_names(tree_file)
    for name in names:
        print(name)
