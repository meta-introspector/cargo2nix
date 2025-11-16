#!/nix/store/a65r9k46dxhyx2gn60bpx7j62anjdjr7-python3-3.13.7/bin/python
import sys
import os
import re

def check_unvendored_configs(unvendored_modules_file, cargo_config_file):
    unvendored_modules = []
    with open(unvendored_modules_file, 'r') as f:
        for line in f:
            unvendored_modules.append(line.strip())

    cargo_config_content = ""
    if os.path.exists(cargo_config_file):
        with open(cargo_config_file, 'r') as f:
            cargo_config_content = f.read()

    results = []
    for module in unvendored_modules:
        # Search for the module name as a standalone word or as part of a path
        # This regex tries to match the module name followed by a non-word character
        # or at the end of the string, to avoid partial matches (e.g., 'foo' matching 'foobar')
        # It also handles cases like `module = { path = "..." }` or `module = "..."`
        pattern = r'\b' + re.escape(module) + r'\b'
        found = "Found" if re.search(pattern, cargo_config_content) else "Not Found"
        results.append(f"{module}: {found}")
    
    return results

if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("Usage: python check_unvendored_configs.py <unvendored_modules_file> <cargo_config_file>")
        sys.exit(1)

    unvendored_modules_file = sys.argv[1]
    cargo_config_file = sys.argv[2]

    report_lines = check_unvendored_configs(unvendored_modules_file, cargo_config_file)
    for line in report_lines:
        print(line)
