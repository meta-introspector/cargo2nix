#include "solvers/monster_scip_solver.hh"
#include <scip/scip.h>
#include <scip/scipdefplugins.h>

namespace MonsterGroup {

MonsterScipSolver::MonsterScipSolver() : scip_(nullptr) {
    SCIPcreate(&scip_);
    SCIPincludeDefaultPlugins(scip_);
    SCIPcreateProb(scip_, "monster_traits", nullptr, nullptr, nullptr, nullptr, nullptr, nullptr, nullptr);
    SCIPsetIntParam(scip_, "display/verblevel", 0);
}

MonsterScipSolver::~MonsterScipSolver() {
    if (scip_) {
        SCIPfree(&scip_);
    }
}

std::vector<uint64_t> MonsterScipSolver::solve_trait_mapping(int num_traits) {
    std::vector<SCIP_VAR*> vars(num_traits);
    
    // Create variables: trait elements [0, MONSTER_ORDER-1]
    for (int i = 0; i < num_traits; i++) {
        SCIPcreateVarBasic(scip_, &vars[i], nullptr, 0.0, 
                          static_cast<double>(MONSTER_ORDER - 1), 1.0, SCIP_VARTYPE_INTEGER);
        SCIPaddVar(scip_, vars[i]);
    }
    
    // All-different constraints
    for (int i = 0; i < num_traits; i++) {
        for (int j = i + 1; j < num_traits; j++) {
            SCIP_CONS* cons;
            SCIP_VAR* diff_vars[2] = {vars[i], vars[j]};
            SCIP_Real coeffs[2] = {1.0, -1.0};
            
            SCIPcreateConsLinear(scip_, &cons, nullptr, 2, diff_vars, coeffs, 
                               1.0, SCIPinfinity(scip_), TRUE, TRUE, TRUE, TRUE, TRUE, FALSE, FALSE, FALSE, FALSE, FALSE);
            SCIPaddCons(scip_, cons);
            SCIPreleaseCons(scip_, &cons);
            
            coeffs[0] = -1.0; coeffs[1] = 1.0;
            SCIPcreateConsLinear(scip_, &cons, nullptr, 2, diff_vars, coeffs, 
                               1.0, SCIPinfinity(scip_), TRUE, TRUE, TRUE, TRUE, TRUE, FALSE, FALSE, FALSE, FALSE, FALSE);
            SCIPaddCons(scip_, cons);
            SCIPreleaseCons(scip_, &cons);
        }
    }
    
    // Modular constraint: sum ≡ 0 (mod 24)
    SCIP_VAR* k_var;
    SCIPcreateVarBasic(scip_, &k_var, nullptr, 0.0, SCIPinfinity(scip_), 0.0, SCIP_VARTYPE_INTEGER);
    SCIPaddVar(scip_, k_var);
    
    std::vector<SCIP_VAR*> mod_vars(vars);
    mod_vars.push_back(k_var);
    std::vector<SCIP_Real> mod_coeffs(num_traits, 1.0);
    mod_coeffs.push_back(-24.0);
    
    SCIP_CONS* mod_cons;
    SCIPcreateConsLinear(scip_, &mod_cons, nullptr, num_traits + 1, mod_vars.data(), mod_coeffs.data(),
                        0.0, 0.0, TRUE, TRUE, TRUE, TRUE, TRUE, FALSE, FALSE, FALSE, FALSE, FALSE);
    SCIPaddCons(scip_, mod_cons);
    SCIPreleaseCons(scip_, &mod_cons);
    
    // Solve
    SCIPsolve(scip_);
    
    std::vector<uint64_t> result;
    if (SCIPgetNSols(scip_) > 0) {
        SCIP_SOL* sol = SCIPgetBestSol(scip_);
        for (int i = 0; i < num_traits; i++) {
            result.push_back(static_cast<uint64_t>(SCIPgetSolVal(scip_, sol, vars[i]) + 0.5));
        }
    }
    
    // Release variables
    for (auto var : vars) {
        SCIPreleaseVar(scip_, &var);
    }
    SCIPreleaseVar(scip_, &k_var);
    
    return result;
}

bool MonsterScipSolver::verify_solution(const std::vector<uint64_t>& elements) {
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
