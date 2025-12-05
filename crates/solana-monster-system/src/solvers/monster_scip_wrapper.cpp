#include "solvers/monster_scip_wrapper.hh"

extern "C" {

void* monster_scip_create() {
    return new MonsterGroup::MonsterScipSolver();
}

void monster_scip_destroy(void* solver) {
    delete static_cast<MonsterGroup::MonsterScipSolver*>(solver);
}

int monster_scip_solve(void* solver, int num_traits, uint64_t* result) {
    auto* scip_solver = static_cast<MonsterGroup::MonsterScipSolver*>(solver);
    auto solution = scip_solver->solve_trait_mapping(num_traits);
    
    if (solution.empty()) return 0;
    
    for (size_t i = 0; i < solution.size(); i++) {
        result[i] = solution[i];
    }
    
    return static_cast<int>(solution.size());
}

bool monster_scip_verify(void* solver, const uint64_t* elements, int count) {
    auto* scip_solver = static_cast<MonsterGroup::MonsterScipSolver*>(solver);
    std::vector<uint64_t> vec(elements, elements + count);
    return scip_solver->verify_solution(vec);
}

} // extern "C"
