#include "solvers/monster_mip_wrapper.hh"

extern "C" {

void* monster_cplex_create() {
    return new MonsterGroup::MonsterCplexSolver();
}

void monster_cplex_destroy(void* solver) {
    delete static_cast<MonsterGroup::MonsterCplexSolver*>(solver);
}

int monster_cplex_solve(void* solver, int num_traits, uint64_t* result) {
    auto* cplex_solver = static_cast<MonsterGroup::MonsterCplexSolver*>(solver);
    auto solution = cplex_solver->solve_trait_mapping(num_traits);
    
    if (solution.empty()) return 0;
    
    for (size_t i = 0; i < solution.size(); i++) {
        result[i] = solution[i];
    }
    
    return static_cast<int>(solution.size());
}

bool monster_cplex_verify(void* solver, const uint64_t* elements, int count) {
    auto* cplex_solver = static_cast<MonsterGroup::MonsterCplexSolver*>(solver);
    std::vector<uint64_t> vec(elements, elements + count);
    return cplex_solver->verify_solution(vec);
}

} // extern "C"
