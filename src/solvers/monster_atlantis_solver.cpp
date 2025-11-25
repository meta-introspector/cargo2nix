#include "solvers/monster_atlantis_solver.hh"

namespace MonsterGroup {

MonsterAtlantisSolver::MonsterAtlantisSolver() {}

std::vector<uint64_t> MonsterAtlantisSolver::solve_trait_mapping(int num_traits) {
    // Atlantis local search for Monster Group constraints
    std::vector<uint64_t> solution(num_traits);
    
    // Initialize with random valid elements
    for (int i = 0; i < num_traits; i++) {
        solution[i] = i * 1000 % MONSTER_ORDER;
    }
    
    // Local search to satisfy constraints
    int max_iterations = 1000;
    for (int iter = 0; iter < max_iterations; iter++) {
        if (verify_solution(solution)) {
            break;
        }
        
        // Improve solution
        for (int i = 0; i < num_traits; i++) {
            uint64_t old_val = solution[i];
            
            // Try different values
            for (int delta = 1; delta < 100; delta++) {
                solution[i] = (old_val + delta) % MONSTER_ORDER;
                
                if (verify_solution(solution)) {
                    goto found_solution;
                }
                
                solution[i] = (old_val + MONSTER_ORDER - delta) % MONSTER_ORDER;
                
                if (verify_solution(solution)) {
                    goto found_solution;
                }
            }
            
            solution[i] = old_val; // Restore if no improvement
        }
    }
    
    found_solution:
    
    // Return solution if valid, empty if not found
    if (verify_solution(solution)) {
        return solution;
    }
    
    return {};
}

bool MonsterAtlantisSolver::verify_solution(const std::vector<uint64_t>& elements) {
    // Monster Group order check
    for (auto elem : elements) {
        if (elem >= MONSTER_ORDER) return false;
    }
    
    // All different check
    std::set<uint64_t> unique_elements(elements.begin(), elements.end());
    if (unique_elements.size() != elements.size()) return false;
    
    // Modular constraint check
    uint64_t sum = 0;
    for (auto elem : elements) {
        sum += elem;
    }
    
    return (sum % RAMANUJAN_MOD) == 0;
}

} // namespace MonsterGroup
