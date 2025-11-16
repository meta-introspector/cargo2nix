import os
import sys

#sys.stdout.write("[patch.crates-io]\n")

config_content = sys.stdin.read()
root_dir = "/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix"
submodules_abs_path = os.path.join(root_dir, "submodules")

absolute_config_content = ""
seen = {}
for line in config_content.splitlines():
    if 'path = "submodules/' in line:
        line = line.replace('path = "submodules/', f'path = "{submodules_abs_path}/')
    if line not in seen:
        absolute_config_content += line + "\n"
        seen[line]=1

sys.stdout.write(absolute_config_content)
