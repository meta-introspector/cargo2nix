#include "../../include/monster_ast.hh"
#include <vector>
#include <algorithm>

namespace MonsterGroup {

struct Edge {
    int from, to;
    uint64_t capacity;
};

class MonsterMinCut {
private:
    std::vector<std::vector<int>> adj_;
    std::vector<Edge> edges_;
    
public:
    MonsterMinCut(int n) : adj_(n) {}
    
    void add_edge(int from, int to, uint64_t capacity) {
        adj_[from].push_back(edges_.size());
        edges_.push_back({from, to, capacity});
        adj_[to].push_back(edges_.size());
        edges_.push_back({to, from, 0}); // Reverse edge
    }
    
    uint64_t max_flow(int source, int sink) {
        // Simple Ford-Fulkerson for Monster Group trait dependencies
        uint64_t total_flow = 0;
        
        while (true) {
            std::vector<int> parent(adj_.size(), -1);
            std::vector<bool> visited(adj_.size(), false);
            
            // BFS to find augmenting path
            std::vector<int> queue = {source};
            visited[source] = true;
            
            bool found_path = false;
            for (size_t i = 0; i < queue.size() && !found_path; i++) {
                int u = queue[i];
                
                for (int edge_id : adj_[u]) {
                    Edge& e = edges_[edge_id];
                    if (!visited[e.to] && e.capacity > 0) {
                        visited[e.to] = true;
                        parent[e.to] = edge_id;
                        queue.push_back(e.to);
                        
                        if (e.to == sink) {
                            found_path = true;
                            break;
                        }
                    }
                }
            }
            
            if (!found_path) break;
            
            // Find minimum capacity along path
            uint64_t path_flow = MONSTER_ORDER;
            for (int v = sink; v != source; ) {
                int edge_id = parent[v];
                path_flow = std::min(path_flow, edges_[edge_id].capacity);
                v = edges_[edge_id].from;
            }
            
            // Update capacities
            for (int v = sink; v != source; ) {
                int edge_id = parent[v];
                edges_[edge_id].capacity -= path_flow;
                edges_[edge_id ^ 1].capacity += path_flow;
                v = edges_[edge_id].from;
            }
            
            total_flow += path_flow;
        }
        
        return total_flow;
    }
};

uint64_t compute_monster_min_cut(const std::vector<std::pair<int, int>>& trait_dependencies) {
    if (trait_dependencies.empty()) return 0;
    
    int max_node = 0;
    for (const auto& [u, v] : trait_dependencies) {
        max_node = std::max(max_node, std::max(u, v));
    }
    
    MonsterMinCut graph(max_node + 1);
    
    for (const auto& [u, v] : trait_dependencies) {
        graph.add_edge(u, v, 1); // Unit capacity for trait dependencies
    }
    
    return graph.max_flow(0, max_node);
}

} // namespace MonsterGroup
