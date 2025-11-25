#include "solvers/monster_cplex_solver.hh"

namespace MonsterGroup {

MonsterCplexSolver::MonsterCplexSolver() : env(), model(env) {}

MonsterCplexSolver::~MonsterCplexSolver() {
    env.end();
}

std::vector<uint64_t> MonsterCplexSolver::solve_trait_mapping(int num_traits) {
    try {
        // Variables: trait elements in Monster Group
        IloIntVarArray traits(env, num_traits, 0, MONSTER_ORDER - 1);
        
        // Constraint: all different
        for (int i = 0; i < num_traits; i++) {
            for (int j = i + 1; j < num_traits; j++) {
                model.add(traits[i] != traits[j]);
            }
        }
        
        // Constraint: modular sum ≡ 0 (mod 24)
        IloExpr sum(env);
        for (int i = 0; i < num_traits; i++) {
            sum += traits[i];
        }
        model.add(IloMod(sum, RAMANUJAN_MOD) == 0);
        sum.end();
        
        // Objective: minimize sum (for deterministic solution)
        IloExpr obj(env);
        for (int i = 0; i < num_traits; i++) {
            obj += traits[i];
        }
        model.add(IloMinimize(env, obj));
        obj.end();
        
        // Solve
        IloCplex cplex(model);
        cplex.setOut(env.getNullStream());
        
        std::vector<uint64_t> result;
        if (cplex.solve()) {
            for (int i = 0; i < num_traits; i++) {
                result.push_back(static_cast<uint64_t>(cplex.getValue(traits[i])));
            }
        }
        
        return result;
        
    } catch (IloException& e) {
        return {};
    }
}

bool MonsterCplexSolver::verify_solution(const std::vector<uint64_t>& elements) {
    // Monster Group order check
    for (auto elem : elements) {
        if (elem >= MONSTER_ORDER) return false;
    }
    
    // Modular constraint check
    uint64_t sum = 0;
    for (auto elem : elements) {
        sum += elem;
    }
    
    return (sum % RAMANUJAN_MOD) == 0;
}

} // namespace MonsterGroup
