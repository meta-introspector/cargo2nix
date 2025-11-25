#include "solvers/monster_geas_solver.hh"

namespace MonsterGroup {

MonsterGeasSolver::MonsterGeasSolver() {}

std::vector<uint64_t> MonsterGeasSolver::solve_trait_mapping(int num_traits) {
    // Create Geas solver instance
    geas::solver solver;
    
    // Variables: trait elements [0, MONSTER_ORDER-1]
    std::vector<geas::intvar> traits;
    for (int i = 0; i < num_traits; i++) {
        traits.push_back(solver.new_intvar(0, MONSTER_ORDER - 1));
    }
    
    // All-different constraint
    solver.post(geas::all_different(traits));
    
    // Modular constraint: sum ≡ 0 (mod 24)
    geas::intvar sum = solver.new_intvar(0, (MONSTER_ORDER - 1) * num_traits);
    solver.post(geas::int_linear_eq(traits, std::vector<int>(num_traits, 1), sum));
    
    // sum mod 24 = 0
    geas::intvar remainder = solver.new_intvar(0, 23);
    solver.post(geas::int_mod(sum, 24, remainder));
    solver.post(geas::int_eq(remainder, 0));
    
    // Search
    std::vector<uint64_t> result;
    if (solver.solve()) {
        for (int i = 0; i < num_traits; i++) {
            result.push_back(static_cast<uint64_t>(solver.get_model()[traits[i]]));
        }
    }
    
    return result;
}

bool MonsterGeasSolver::verify_solution(const std::vector<uint64_t>& elements) {
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
