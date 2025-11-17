import re
import os
import toml # Assuming toml library is available, if not, I'll need to parse manually
from collections import defaultdict

def parse_members_file(members_file_path):
    """
    Parses submodules/members.txt to get a list of (submodule_path, members_list) tuples.
    """
    workspace_info = [] # List of (relative_submodule_path, members_list)
    with open(members_file_path, 'r') as f:
        for line in f:
            # Example line: submodules/time-rs/Cargo.toml:members = ["time", "time-core", "time-macros"]
            match = re.match(r'(submodules/[^/]+/Cargo\.toml):members = \[(.*)\]', line)
            if match:
                cargo_toml_rel_path = match.group(1)
                members_str = match.group(2).strip()
                
                # Extract submodule_base_path (e.g., "submodules/time-rs")
                submodule_base_path_rel = os.path.dirname(cargo_toml_rel_path)
                
                # Parse members_str into a list
                members_list = []
                # Handle cases like 'crates/cpp_smoke_test', 'crates/as-if-std' or "time", "time-core"
                # This regex is a bit fragile, assuming simple comma-separated quoted strings
                member_matches = re.findall(r'"([^"]+)"', members_str)
                members_list.extend(member_matches)
                member_matches_single_quote = re.findall(r"'([^']+)'", members_str)
                members_list.extend(member_matches_single_quote)

                # Filter out '.' members if they are not explicitly handled as sub-crates
                # For now, we're focusing on explicit sub-crates like time, time-core
                members_list = [m for m in members_list if m != '.']

                if members_list: # Only add if actual members are found
                    workspace_info.append((submodule_base_path_rel, members_list))
    return workspace_info

def generate_patch_entries(project_root, workspace_info):
    """
    Generates [patch] entries for .cargo/config.toml for each workspace member.
    """
    generated_patches = defaultdict(list) # {patch_section_header: [entry_lines]}

    for submodule_base_path_rel, members_list in workspace_info:
        submodule_name = os.path.basename(submodule_base_path_rel)
        
        # Construct the patch section header based on the meta-introspector pattern
        patch_section_header = f'[patch."https://github.com/meta-introspector/{submodule_name}"]'
        
        for member_name in members_list:
            # Construct the absolute path for the member
            member_abs_path = os.path.join(project_root, submodule_base_path_rel, member_name)
            entry_line = f'{member_name} = {{ path = "{member_abs_path}" }}'
            generated_patches[patch_section_header].append(entry_line)
            
    return generated_patches

def update_config_toml(config_toml_path, new_patches):
    """
    Reads existing .cargo/config.toml, updates patch sections, and writes back.
    """
    existing_config = {}
    try:
        with open(config_toml_path, 'r') as f:
            existing_config = toml.load(f)
    except FileNotFoundError:
        print(f"Warning: {config_toml_path} not found. Creating a new one.")
    except Exception as e:
        print(f"Error loading existing {config_toml_path}: {e}. Starting with empty config.")
        existing_config = {}

    # Ensure [patch] section exists
    if 'patch' not in existing_config:
        existing_config['patch'] = {}

    # Update existing patch sections or add new ones
    for patch_header, entries in new_patches.items():
        # Extract the repo URL from the header
        repo_url_match = re.match(r'\[patch\."([^"]+)"\]', patch_header)
        if repo_url_match:
            repo_url = repo_url_match.group(1)
            
            # Ensure the patch section for this repo_url exists
            if repo_url not in existing_config['patch']:
                existing_config['patch'][repo_url] = {}
            
            # Add/update entries for this patch section
            for entry_line in entries:
                # Parse "crate_name = { path = "..." }"
                entry_match = re.match(r'([^=]+) = \{ path = "([^"]+)" \}', entry_line)
                if entry_match:
                    crate_name = entry_match.group(1).strip()
                    path = entry_match.group(2).strip()
                    existing_config['patch'][repo_url][crate_name] = {'path': path}
                else:
                    print(f"Warning: Could not parse entry line: {entry_line}")
        else:
            print(f"Warning: Could not parse patch header: {patch_header}")

    # Write the updated config back to the file
    with open(config_toml_path, 'w') as f:
        toml.dump(existing_config, f)
    print(f"Updated {config_toml_path}")


def main():
    project_root = "/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix"
    members_file = os.path.join(project_root, "submodules/members.txt")
    config_toml_path = os.path.join(project_root, ".cargo/config.toml")

    workspace_info = parse_members_file(members_file)
    new_patches = generate_patch_entries(project_root, workspace_info)
    
    update_config_toml(config_toml_path, new_patches)

if __name__ == "__main__":
    main()
