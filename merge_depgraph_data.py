import sys
import subprocess
from collections import defaultdict

def read_layer_data(process_depgraph_script_path):
    """
    Runs process_depgraph.py and parses its output for layer information.
    Returns a dictionary: {crate_name: layer_number}
    """
    layer_data = {}
    try:
        # Execute process_depgraph.py and capture its stdout
        result = subprocess.run(
            ['python', process_depgraph_script_path],
            capture_output=True,
            text=True,
            check=True
        )
        
        for line in result.stdout.splitlines():
            parts = line.strip().split(': Layer ')
            if len(parts) == 2:
                crate_name = parts[0].strip()
                try:
                    layer = int(parts[1].strip())
                    layer_data[crate_name] = layer
                except ValueError:
                    sys.stderr.write(f"Warning: Could not parse layer number for '{crate_name}' in line: {line.strip()}\n")
            else:
                sys.stderr.write(f"Warning: Unexpected line format from process_depgraph.py: {line.strip()}\n")
    except subprocess.CalledProcessError as e:
        sys.stderr.write(f"Error running process_depgraph.py: {e}\n")
        sys.stderr.write(f"Stdout: {e.stdout}\n")
        sys.stderr.write(f"Stderr: {e.stderr}\n")
    except FileNotFoundError:
        sys.stderr.write(f"Error: process_depgraph.py not found at {process_depgraph_script_path}\n")
    return layer_data

def read_usage_counts(usage_file_path):
    """
    Reads usage counts from a file (e.g., non_vendored_modules.txt).
    Returns a dictionary: {crate_name: usage_count}
    """
    usage_counts = defaultdict(int)
    try:
        with open(usage_file_path, 'r') as f:
            for line in f:
                parts = line.strip().split()
                if len(parts) == 2:
                    crate_name = parts[0]
                    try:
                        count = int(parts[1])
                        usage_counts[crate_name] = count
                    except ValueError:
                        sys.stderr.write(f"Warning: Could not parse usage count for '{crate_name}' in line: {line.strip()}\n")
                elif len(parts) == 1:
                    usage_counts[parts[0]] = 1 # Default to 1 if no count is specified
                else:
                    sys.stderr.write(f"Warning: Unexpected line format in usage file: {line.strip()}\n")
    except FileNotFoundError:
        sys.stderr.write(f"Warning: Usage file not found at {usage_file_path}. Usage counts will not be available.\n")
    return usage_counts

def main():
    process_depgraph_script_path = './process_depgraph.py' # Relative path
    usage_file_path = '.cargo/non_vendored_modules.txt' # Relative path
    
    layer_data = read_layer_data(process_depgraph_script_path)
    usage_counts = read_usage_counts(usage_file_path)

    # Merge data
    merged_data = defaultdict(lambda: {'layer': -1, 'usage_count': 0})
    all_crates = set(layer_data.keys()).union(set(usage_counts.keys()))

    for crate in all_crates:
        if crate in layer_data:
            merged_data[crate]['layer'] = layer_data[crate]
        if crate in usage_counts:
            merged_data[crate]['usage_count'] = usage_counts[crate]

    # Filter out crates that don't have layer information (i.e., not in depgraph)
    # Or decide how to handle them. For now, let's only show crates with layer info.
    final_crates = {crate: data for crate, data in merged_data.items() if data['layer'] != -1}

    # Order by layer (0-N)
    sys.stdout.write("Crates ordered by Layer (0-N) with Usage Counts:\n")
    sorted_by_layer_asc = sorted(final_crates.items(), key=lambda item: (item[1]['layer'], item[0]))
    for crate, data in sorted_by_layer_asc:
        sys.stdout.write(f"  {crate}: Layer {data['layer']}, Usage Count: {data['usage_count']}\n")

    # Cluster by Layer (0-N)
    clusters_asc = defaultdict(list)
    for crate, data in sorted_by_layer_asc:
        clusters_asc[data['layer']].append((crate, data['usage_count']))

    sys.stdout.write("\nClustered by Layer (0-N) with Usage Counts:\n")
    for layer_num in sorted(clusters_asc.keys()):
        sys.stdout.write(f"Layer {layer_num} (Total modules: {len(clusters_asc[layer_num])}):\n")
        for crate, count in sorted(clusters_asc[layer_num], key=lambda item: item[0]):
            sys.stdout.write(f"  - {crate}, Usage Count: {count}\n")

    # Cluster by Layer (N-0)
    sys.stdout.write("\nClustered by Layer (N-0) with Usage Counts:\n")
    sorted_layers_desc = sorted(clusters_asc.keys(), reverse=True)
    for layer_num in sorted(sorted_layers_desc): # Sort again to ensure consistent order of layers if there are multiple layers with same number
        sys.stdout.write(f"Layer {layer_num} (Total modules: {len(clusters_asc[layer_num])}):\n")
        for crate, count in sorted(clusters_asc[layer_num], key=lambda item: item[0]):
            sys.stdout.write(f"  - {crate}, Usage Count: {count}\n")

if __name__ == "__main__":
    main()
