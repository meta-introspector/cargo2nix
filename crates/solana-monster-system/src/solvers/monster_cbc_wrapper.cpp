#include "solvers/monster_cbc_wrapper.hh"

extern "C" {

void* monster_cbc_create() {
    return new MonsterGroup::MonsterOsiCbcSolver();
}

void monster_cbc_destroy(void* solver) {
    delete static_cast<MonsterGroup::MonsterOsiCbcSolver*>(solver);
}

int monster_cbc_solve(void* solver, int num_traits, uint64_t* result) {
    auto* cbc_solver = static_cast<MonsterGroup::MonsterOsiCbcSolver*>(solver);
    auto solution = cbc_solver->solve_trait_mapping(num_traits);
    
    if (solution.empty()) return 0;
    
    for (size_t i = 0; i < solution.size(); i++) {
        result[i] = solution[i];
    }
    
    return static_cast<int>(solution.size());
}

bool monster_cbc_verify(void* solver, const uint64_t* elements, int count) {
    auto* cbc_solver = static_cast<MonsterGroup::MonsterOsiCbcSolver*>(solver);
    std::vector<uint64_t> vec(elements, elements + count);
    return cbc_solver->verify_solution(vec);
}

} // extern "C"
