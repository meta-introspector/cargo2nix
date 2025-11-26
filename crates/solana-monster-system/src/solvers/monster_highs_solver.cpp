#include "solvers/monster_highs_solver.hh"
#include <Highs.h>

namespace MonsterGroup {

MonsterHighsSolver::MonsterHighsSolver() {}

std::vector<uint64_t> MonsterHighsSolver::solve_trait_mapping(int num_traits) {
    Highs highs;
    highs.setOptionValue("output_flag", false);
    
    // Variables: trait elements [0, MONSTER_ORDER-1]
    std::vector<double> col_cost(num_traits, 1.0); // Minimize sum
    std::vector<double> col_lower(num_traits, 0.0);
    std::vector<double> col_upper(num_traits, static_cast<double>(MONSTER_ORDER - 1));
    
    highs.addCols(num_traits, col_cost.data(), col_lower.data(), col_upper.data());
    
    // Set integer variables
    HighsVarType var_type = HighsVarType::kInteger;
    for (int i = 0; i < num_traits; i++) {
        highs.changeColIntegrality(i, var_type);
    }
    
    // All-different constraints: x[i] - x[j] >= 1 for i != j
    for (int i = 0; i < num_traits; i++) {
        for (int j = i + 1; j < num_traits; j++) {
            std::vector<int> indices = {i, j};
            std::vector<double> values = {1.0, -1.0};
            highs.addRow(1.0, highs.getInfinity(), 2, indices.data(), values.data());
            
            values = {-1.0, 1.0};
            highs.addRow(1.0, highs.getInfinity(), 2, indices.data(), values.data());
        }
    }
    
    // Modular constraint: sum ≡ 0 (mod 24)
    // Add auxiliary variable k: sum - 24*k = 0
    highs.addCol(0.0, 0.0, highs.getInfinity()); // k variable
    highs.changeColIntegrality(num_traits, var_type);
    
    std::vector<int> mod_indices(num_traits + 1);
    std::vector<double> mod_values(num_traits + 1);
    for (int i = 0; i < num_traits; i++) {
        mod_indices[i] = i;
        mod_values[i] = 1.0;
    }
    mod_indices[num_traits] = num_traits;
    mod_values[num_traits] = -24.0;
    
    highs.addRow(0.0, 0.0, num_traits + 1, mod_indices.data(), mod_values.data());
    
    // Solve
    HighsStatus status = highs.run();
    
    std::vector<uint64_t> result;
    if (status == HighsStatus::kOptimal) {
        const HighsSolution& solution = highs.getSolution();
        for (int i = 0; i < num_traits; i++) {
            result.push_back(static_cast<uint64_t>(solution.col_value[i] + 0.5));
        }
    }
    
    return result;
}

bool MonsterHighsSolver::verify_solution(const std::vector<uint64_t>& elements) {
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
