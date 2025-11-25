#include "solvers/monster_fzn_solver.hh"
#include <sstream>
#include <fstream>

namespace MonsterGroup {

MonsterFznSolver::MonsterFznSolver() {}

std::string MonsterFznSolver::generate_fzn_model(int num_traits) {
    std::ostringstream fzn;
    
    // Variable declarations
    for (int i = 0; i < num_traits; i++) {
        fzn << "var 0.." << (MONSTER_ORDER - 1) << ": trait_" << i << ";\n";
    }
    
    // All different constraint
    fzn << "constraint all_different([";
    for (int i = 0; i < num_traits; i++) {
        if (i > 0) fzn << ", ";
        fzn << "trait_" << i;
    }
    fzn << "]);\n";
    
    // Modular constraint: sum mod 24 = 0
    fzn << "constraint (";
    for (int i = 0; i < num_traits; i++) {
        if (i > 0) fzn << " + ";
        fzn << "trait_" << i;
    }
    fzn << ") mod " << RAMANUJAN_MOD << " = 0;\n";
    
    // Solve statement
    fzn << "solve satisfy;\n";
    
    // Output
    fzn << "output [";
    for (int i = 0; i < num_traits; i++) {
        if (i > 0) fzn << ", ";
        fzn << "show(trait_" << i << ")";
    }
    fzn << "];\n";
    
    return fzn.str();
}

std::vector<uint64_t> MonsterFznSolver::solve_trait_mapping(int num_traits) {
    // Generate FlatZinc model
    std::string model = generate_fzn_model(num_traits);
    
    // Write to temporary file
    std::ofstream file("monster_traits.fzn");
    file << model;
    file.close();
    
    // Mock solution (would normally call external FlatZinc solver)
    std::vector<uint64_t> result;
    for (int i = 0; i < num_traits; i++) {
        result.push_back((i * 24) % MONSTER_ORDER);
    }
    
    // Verify and adjust if needed
    if (!verify_solution(result)) {
        result.clear();
    }
    
    return result;
}

bool MonsterFznSolver::verify_solution(const std::vector<uint64_t>& elements) {
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
