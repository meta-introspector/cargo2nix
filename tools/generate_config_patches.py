import re
import os

def generate_new_config(tree_file, config_file, cargo_lock_file, output_file):
    # 1. Parse tree.txt
    submodule_paths = {}
    with open(tree_file, 'r') as f:
        for line in f:
            # Revised regex to specifically capture the absolute path starting with /data/data/.../submodules/
            # This version avoids the unbalanced parenthesis error by not trying to match the outer parentheses of the entire line.
            match = re.match(r'^\s*├──\s+(\S+)\s+v\S+\s+.*?(?P<path>/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/[^\s)]+)', line)
            if match:
                package_name = match.group(1)
                full_path = match.group('path')
                config_dir = os.path.dirname(config_file) # e.g., /data/.../cargo2nix/.cargo
                project_root = os.path.dirname(config_dir) # e.g., /data/.../cargo2nix
                relative_path_to_submodule = os.path.relpath(full_path, project_root)
                submodule_paths[package_name] = relative_path_to_submodule

    # 2. Parse .cargo/config.toml
    existing_patched_packages = set()
    commented_out_packages = set()
    
    with open(config_file, 'r') as f:
        original_config_lines = f.readlines()

    in_crates_io_patch_section = False
    for i, line in enumerate(original_config_lines):
        stripped_line = line.strip()

        if stripped_line == '[patch.crates-io]':
            in_crates_io_patch_section = True
            continue
        elif stripped_line.startswith('[patch."https://github.com/meta-introspector/'):
            in_crates_io_patch_section = False # This is a different patch section
            # Extract package name from the next line for URL patches
            if i + 1 < len(original_config_lines):
                next_line = original_config_lines[i + 1].strip()
                pkg_match = re.match(r'^\s*(\S+)\s*=\s*{\s*', next_line)
                if pkg_match:
                    existing_patched_packages.add(pkg_match.group(1))
            continue

        if in_crates_io_patch_section:
            match_crates_io_patch = re.match(r'^\s*(\S+)\s*=\s*{\s*path\s*=\s*"(.+)"\s*}', stripped_line)
            if match_crates_io_patch:
                existing_patched_packages.add(match_crates_io_patch.group(1))
                continue
            
            match_commented_crates_io = re.match(r'^\s*#+\s*(\S+)\s*=\s*{\s*path\s*=\s*"(.+)"\s*}', stripped_line)
            if match_commented_crates_io:
                commented_out_packages.add(match_commented_crates_io.group(1))
                continue
        else: # Not in [patch.crates-io] section, but could be other commented lines
            match_commented_crates_io = re.match(r'^\s*#+\s*(\S+)\s*=\s*{\s*path\s*=\s*"(.+)"\s*}', stripped_line)
            if match_commented_crates_io:
                commented_out_packages.add(match_commented_crates_io.group(1))
                continue
            
            match_commented_crates_io_double_hash = re.match(r'^\s*##(\S+)\s*=\s*{\s*path\s*=\s*"(.+)"\s*}', stripped_line)
            if match_commented_crates_io_double_hash:
                commented_out_packages.add(match_commented_crates_io_double_hash.group(1))
                continue

    # 3. Parse Cargo.lock
    cargo_lock_packages = set()
    with open(cargo_lock_file, 'r') as f:
        content = f.read()
        for match in re.finditer(r'\[\[package\]\]\s*\nname = "([^"]+)"', content):
            cargo_lock_packages.add(match.group(1))

    # 4. Generate new patches
    new_crates_io_patches_to_add = []
    for package_name in sorted(cargo_lock_packages):
        if package_name in submodule_paths:
            if package_name not in existing_patched_packages and package_name not in commented_out_packages:
                relative_path = submodule_paths[package_name]
                new_crates_io_patches_to_add.append(f'    {package_name} = {{ path = "{relative_path}" }}')

    # 5. Construct config.toml.new
    output_lines = []
    crates_io_section_start_index = -1
    crates_io_section_end_index = -1

    for i, line in enumerate(original_config_lines):
        if line.strip() == '[patch.crates-io]':
            crates_io_section_start_index = i
            # Find the end of the section (next section header or end of file)
            for j in range(i + 1, len(original_config_lines)):
                if original_config_lines[j].strip().startswith('[') and not original_config_lines[j].strip().startswith('[patch."https://github.com/meta-introspector/'):
                    crates_io_section_end_index = j
                    break
            if crates_io_section_end_index == -1: # Reached end of file
                crates_io_section_end_index = len(original_config_lines)
            break
    
    if crates_io_section_start_index != -1:
        # Add lines before the [patch.crates-io] section
        output_lines.extend(original_config_lines[:crates_io_section_start_index])
        # Add the [patch.crates-io] header
        output_lines.append(original_config_lines[crates_io_section_start_index])
        
        # Add existing patches from the original [patch.crates-io] section
        for line in original_config_lines[crates_io_section_start_index + 1 : crates_io_section_end_index]:
            stripped_line = line.strip()
            # Only add lines that are not commented out and are not duplicates of new patches
            if not stripped_line.startswith('#'):
                match_crates_io_patch = re.match(r'^\s*(\S+)\s*=\s*{\s*path\s*=\s*"(.+)"\s*}', stripped_line)
                if match_crates_io_patch and match_crates_io_patch.group(1) not in [p.strip().split(' ')[0] for p in new_crates_io_patches_to_add]:
                    output_lines.append(line)
                elif not match_crates_io_patch: # Preserve other non-patch lines (e.g., blank lines)
                    output_lines.append(line)

        # Add new patches
        output_lines.extend([p + '\n' for p in new_crates_io_patches_to_add])
        # Add lines after the [patch.crates-io] section
        output_lines.extend(original_config_lines[crates_io_section_end_index:])
    else:
        # If [patch.crates-io] section was not found, add all original lines and then the new section
        output_lines.extend(original_config_lines)
        output_lines.append('\n[patch.crates-io]\n')
        output_lines.extend([p + '\n' for p in new_crates_io_patches_to_add])

    with open(output_file, 'w') as f:
        f.writelines(output_lines)

    print(f"Generated {output_file} with new patches.")

# Define file paths
tree_file = '/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/tree.txt'
config_file = '/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/.cargo/config.toml'
cargo_lock_file = '/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/Cargo.lock'
output_file = '/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/config.toml.new'

# Call the function
generate_new_config(tree_file, config_file, cargo_lock_file, output_file)
