#include "solvers/monster_osicbc_solver.hh"
#include <CbcModel.hpp>
#include <OsiClpSolverInterface.hpp>

namespace MonsterGroup {

MonsterOsiCbcSolver::MonsterOsiCbcSolver() {}

std::vector<uint64_t> MonsterOsiCbcSolver::solve_trait_mapping(int num_traits) {
    OsiClpSolverInterface solver;
    
    // Variables: trait elements [0, MONSTER_ORDER-1]
    std::vector<double> col_lb(num_traits, 0.0);
    std::vector<double> col_ub(num_traits, static_cast<double>(MONSTER_ORDER - 1));
    std::vector<double> obj(num_traits, 1.0); // Minimize sum
    
    // Add variables
    for (int i = 0; i < num_traits; i++) {
        solver.addCol(0, nullptr, nullptr, col_lb[i], col_ub[i], obj[i]);
        solver.setInteger(i);
    }
    
    // All-different constraints (simplified as inequalities)
    for (int i = 0; i < num_traits; i++) {
        for (int j = i + 1; j < num_traits; j++) {
            // x[i] - x[j] >= 1 OR x[j] - x[i] >= 1
            std::vector<int> indices = {i, j};
            std::vector<double> coeffs = {1.0, -1.0};
            solver.addRow(2, indices.data(), coeffs.data(), 1.0, solver.getInfinity());
            
            coeffs = {-1.0, 1.0};
            solver.addRow(2, indices.data(), coeffs.data(), 1.0, solver.getInfinity());
        }
    }
    
    // Modular constraint: sum ≡ 0 (mod 24)
    // Implemented as: sum = 24*k for some integer k
    std::vector<int> sum_indices(num_traits);
    std::vector<double> sum_coeffs(num_traits, 1.0);
    for (int i = 0; i < num_traits; i++) {
        sum_indices[i] = i;
    }
    
    // Add auxiliary variable for k
    solver.addCol(0, nullptr, nullptr, 0.0, solver.getInfinity(), 0.0);
    solver.setInteger(num_traits);
    
    // sum - 24*k = 0
    sum_indices.push_back(num_traits);
    sum_coeffs.push_back(-24.0);
    solver.addRow(num_traits + 1, sum_indices.data(), sum_coeffs.data(), 0.0, 0.0);
    
    // Solve with CBC
    CbcModel model(solver);
    model.setLogLevel(0);
    model.branchAndBound();
    
    std::vector<uint64_t> result;
    if (model.isProvenOptimal()) {
        const double* solution = model.getColSolution();
        for (int i = 0; i < num_traits; i++) {
            result.push_back(static_cast<uint64_t>(solution[i] + 0.5));
        }
    }
    
    return result;
}

bool MonsterOsiCbcSolver::verify_solution(const std::vector<uint64_t>& elements) {
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
