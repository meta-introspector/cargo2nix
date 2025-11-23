import toml
import os
from collections import defaultdict, deque

def get_crate_name(cargo_toml_path):
    """Extracts the package name from a Cargo.toml file."""
    try:
        with open(cargo_toml_path, 'r') as f:
            cargo_data = toml.load(f)
        return cargo_data.get('package', {}).get('name')
    except Exception as e:
        print(f"Error reading {cargo_toml_path}: {e}")
        return None

def get_path_dependencies(cargo_toml_path):
    """
    Extracts path dependencies from a Cargo.toml file.
    Returns a list of (dependency_name, relative_path) tuples.
    """
    dependencies = []
    try:
        with open(cargo_toml_path, 'r') as f:
            cargo_data = toml.load(f)

        for dep_type in ['dependencies', 'build-dependencies', 'dev-dependencies']:
            for dep_name, dep_info in cargo_data.get(dep_type, {}).items():
                if isinstance(dep_info, dict) and 'path' in dep_info:
                    dependencies.append((dep_name, dep_info['path']))
                elif isinstance(dep_info, str) and dep_info.startswith('./'):
                    # Handle cases where path is directly a string, e.g., dep = "./path/to/dep"
                    dependencies.append((dep_name, dep_info))
    except Exception as e:
        print(f"Error parsing dependencies in {cargo_toml_path}: {e}")
    return dependencies

def resolve_path(base_path, relative_path):
    """Resolves a relative path to an absolute path."""
    return os.path.abspath(os.path.join(os.path.dirname(base_path), relative_path))

def build_crate_path_map(cargo_toml_files):
    """
    Builds a map from crate name to its Cargo.toml absolute path.
    Handles potential duplicate crate names by prioritizing the first encountered.
    """
    crate_path_map = {}
    for path in cargo_toml_files:
        crate_name = get_crate_name(path)
        if crate_name and crate_name not in crate_path_map:
            crate_path_map[crate_name] = path
    return crate_path_map

def build_dependency_graph(start_crate_name, crate_path_map, all_cargo_toml_files):
    """
    Builds a dependency graph starting from a given crate.
    Returns a tuple: (graph, crate_layers, leaf_crates)
    """
    graph = defaultdict(list)
    reverse_graph = defaultdict(list)
    crate_layers = {}
    visited = set()
    queue = deque([(start_crate_name, 0)]) # (crate_name, layer)

    if start_crate_name not in crate_path_map:
        print(f"Error: Starting crate '{start_crate_name}' not found in crate_path_map.")
        return graph, crate_layers, set()

    # Initialize with the starting crate
    crate_layers[start_crate_name] = 0
    visited.add(start_crate_name)

    while queue:
        current_crate_name, current_layer = queue.popleft()
        current_cargo_toml_path = crate_path_map[current_crate_name]

        deps = get_path_dependencies(current_cargo_toml_path)
        
        is_leaf = True
        for dep_name, rel_path in deps:
            resolved_dep_path = resolve_path(current_cargo_toml_path, rel_path)
            
            # Find the actual crate name for the resolved path
            dep_crate_name = None
            for name, path in crate_path_map.items():
                if path == resolved_dep_path or os.path.dirname(path) == resolved_dep_path:
                    dep_crate_name = name
                    break
            
            if dep_crate_name:
                is_leaf = False
                if dep_crate_name not in graph[current_crate_name]:
                    graph[current_crate_name].append(dep_crate_name)
                    reverse_graph[dep_crate_name].append(current_crate_name)

                if dep_crate_name not in visited:
                    visited.add(dep_crate_name)
                    crate_layers[dep_crate_name] = current_layer + 1
                    queue.append((dep_crate_name, current_layer + 1))
            else:
                # If a path dependency points to a directory that doesn't contain a known Cargo.toml
                # it might be an external dependency or a misconfigured path.
                # For now, we'll just note it.
                print(f"Warning: Dependency '{dep_name}' at '{resolved_dep_path}' for '{current_crate_name}' not found in crate_path_map.")
        
        # If a crate has no outgoing dependencies within the graph, it's a leaf
        # (or if it only has external dependencies not in our map)
        if is_leaf and not graph[current_crate_name]:
            # This check is not entirely accurate for leaf nodes, as a crate might have external dependencies
            # but no *path* dependencies within the analyzed set.
            # A better leaf check is to see if it has no outgoing edges in the constructed graph.
            pass # We'll determine leaf_crates later based on the final graph

    leaf_crates = {crate for crate in graph if not graph[crate]}
    
    return graph, crate_layers, leaf_crates

def main():
    project_root = "/data/data/com.termux.nix/files/home/experiments/pick-up-nix/vendor/nix/cargo2nix/"
    tools_dir = "/data/data/com.termux.nix/files/home/experiments/pick-up-nix/vendor/nix/cargo2nix/tools/"

    all_cargo_toml_files = []
    for root_dir in [project_root, tools_dir]:
        for dirpath, _, filenames in os.walk(root_dir):
            for filename in filenames:
                if filename == "Cargo.toml":
                    all_cargo_toml_files.append(os.path.join(dirpath, filename))

    crate_path_map = build_crate_path_map(all_cargo_toml_files)

    start_crate = "rustc-main"
    if start_crate not in crate_path_map:
        print(f"Error: Starting crate '{start_crate}' not found. Please ensure its Cargo.toml is present and correctly named.")
        return

    dependency_graph, crate_layers, leaf_crates = build_dependency_graph(start_crate, crate_path_map, all_cargo_toml_files)

    print("\n--- Dependency Graph Analysis ---")
    print(f"Starting from: {start_crate}")
    print("\nCrate Layers:")
    sorted_layers = sorted(crate_layers.items(), key=lambda item: item[1])
    for crate, layer in sorted_layers:
        print(f"  {crate}: Layer {layer}")

    print("\nLeaf Crates (crates with no internal path dependencies):")
    for leaf in sorted(list(leaf_crates)):
        print(f"  {leaf}")

    print("\nFull Dependency Graph:")
    for crate, deps in sorted(dependency_graph.items()):
        print(f"  {crate} -> {', '.join(deps)}")

if __name__ == "__main__":
    main()
