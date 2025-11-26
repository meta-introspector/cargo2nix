#include "solvers/monster_gurobi_solver.hh"
#include <gurobi_c++.h>

namespace MonsterGroup {

MonsterGurobiSolver::MonsterGurobiSolver() : env_(), model_(env_) {
    env_.set(GRB_IntParam_OutputFlag, 0);
}

MonsterGurobiSolver::~MonsterGurobiSolver() {}

std::vector<uint64_t> MonsterGurobiSolver::solve_trait_mapping(int num_traits) {
    try {
        // Variables: trait elements [0, MONSTER_ORDER-1]
        std::vector<GRBVar> traits(num_traits);
        for (int i = 0; i < num_traits; i++) {
            traits[i] = model_.addVar(0.0, static_cast<double>(MONSTER_ORDER - 1), 1.0, GRB_INTEGER);
        }
        
        // All-different constraints
        for (int i = 0; i < num_traits; i++) {
            for (int j = i + 1; j < num_traits; j++) {
                model_.addConstr(traits[i] - traits[j] >= 1);
                model_.addConstr(traits[j] - traits[i] >= 1);
            }
        }
        
        // Modular constraint: sum ≡ 0 (mod 24)
        GRBVar k = model_.addVar(0.0, GRB_INFINITY, 0.0, GRB_INTEGER);
        GRBLinExpr sum_expr = 0;
        for (int i = 0; i < num_traits; i++) {
            sum_expr += traits[i];
        }
        model_.addConstr(sum_expr - 24 * k == 0);
        
        // Objective: minimize sum
        GRBLinExpr obj = 0;
        for (int i = 0; i < num_traits; i++) {
            obj += traits[i];
        }
        model_.setObjective(obj, GRB_MINIMIZE);
        
        // Solve
        model_.optimize();
        
        std::vector<uint64_t> result;
        if (model_.get(GRB_IntAttr_Status) == GRB_OPTIMAL) {
            for (int i = 0; i < num_traits; i++) {
                result.push_back(static_cast<uint64_t>(traits[i].get(GRB_DoubleAttr_X) + 0.5));
            }
        }
        
        return result;
        
    } catch (GRBException& e) {
        return {};
    }
}

bool MonsterGurobiSolver::verify_solution(const std::vector<uint64_t>& elements) {
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
