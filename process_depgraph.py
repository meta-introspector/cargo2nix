import re
import sys
from collections import defaultdict

def parse_dot_file(dot_file_path):
    graph = defaultdict(list)
    nodes = set()
    node_id_to_name = {}
    
    with open(dot_file_path, 'r') as f:
        for line in f:
            line = line.strip()
            
            # Parse node definitions: ID [ label = "crate_name" ... ]
            node_match = re.match(r'(\d+)\s*\[\s*label\s*=\s*"([^"]+)"', line)
            if node_match:
                node_id, node_name = node_match.groups()
                # Remove version numbers from node_name for cleaner processing
                node_name = re.sub(r'\s+\d+\.\d+\.\d+.*', '', node_name).strip()
                node_id_to_name[node_id] = node_name
                nodes.add(node_name)
                continue # Move to next line after processing node definition

            # Parse edge definitions: ID -> ID [ ... ]
            edge_match = re.match(r'(\d+)\s*->\s*(\d+)', line)
            if edge_match:
                source_id, target_id = edge_match.groups()
                source_name = node_id_to_name.get(source_id)
                target_name = node_id_to_name.get(target_id)
                
                if source_name and target_name:
                    graph[source_name].append(target_name)
                    nodes.add(source_name)
                    nodes.add(target_name)
    return graph, nodes

def reverse_graph(graph, nodes):
    """
    Reverses the direction of edges in the graph.
    Returns a new graph: {target_node: [source_node, ...]}.
    """
    reversed_graph = defaultdict(list)
    for node in nodes:
        # Ensure all nodes are in the reversed_graph, even if they have no incoming edges
        if node not in reversed_graph:
            reversed_graph[node] = []

    for source, targets in graph.items():
        for target in targets:
            reversed_graph[target].append(source)
    return reversed_graph

def calculate_layers(graph, nodes):
    """
    Calculates the layer for each node in the graph.
    Layer 0 nodes are those with no outgoing edges in the original graph (i.e., depend on nothing).
    """
    layers = {}
    
    # Initialize all nodes with an undefined layer
    for node in nodes:
        layers[node] = -1

    # Nodes with no outgoing edges are layer 0
    current_layer = 0
    nodes_in_current_layer = set(node for node in nodes if not graph[node])
    
    # Keep track of processed nodes to avoid infinite loops in case of cycles
    processed_nodes = set()

    while nodes_in_current_layer:
        next_layer_nodes = set()
        for node in nodes_in_current_layer:
            if node not in processed_nodes:
                layers[node] = current_layer
                processed_nodes.add(node)
        
        # Find nodes for the next layer
        # A node belongs to the next layer if all its dependencies are in the current or previous layers
        for potential_next_layer_node in nodes:
            if potential_next_layer_node not in processed_nodes:
                all_deps_assigned = True
                if potential_next_layer_node in graph:
                    for dep in graph[potential_next_layer_node]:
                        if layers.get(dep, -1) == -1: # If a dependency has no layer yet
                            all_deps_assigned = False
                            break
                else: # Node has no dependencies, should have been caught in layer 0
                    all_deps_assigned = False # This should not happen if layer 0 is correctly initialized
                
                if all_deps_assigned:
                    next_layer_nodes.add(potential_next_layer_node)
        
        if not next_layer_nodes and len(processed_nodes) < len(nodes):
            # This indicates a cycle or unreachable nodes not handled by initial layer 0 assignment
            # For now, let's break to avoid infinite loop and assign remaining to a high layer
            break

        nodes_in_current_layer = next_layer_nodes
        current_layer += 1
        
    # Handle any remaining unassigned nodes (e.g., due to cycles)
    for node in nodes:
        if layers[node] == -1:
            layers[node] = current_layer # Assign to a "cycle" layer

    return layers

def cluster_by_layer(layers):
    """
    Clusters nodes by their assigned layer.
    Returns a dictionary: {layer_number: [node1, node2, ...]}.
    """
    clusters = defaultdict(list)
    for node, layer in layers.items():
        clusters[layer].append(node)
    return clusters

def main():
    dot_file_path = 'depgraph.dot'
    
    graph, nodes = parse_dot_file(dot_file_path)
    layers = calculate_layers(graph, nodes)
    
    # Output layers in a parsable format: "crate_name: Layer layer_number"
    for crate, layer in sorted(layers.items(), key=lambda item: (item[1], item[0])):
        sys.stdout.write(f"{crate}: Layer {layer}\n")

if __name__ == "__main__":
    main()
