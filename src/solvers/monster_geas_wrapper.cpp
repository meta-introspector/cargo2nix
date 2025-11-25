#include "solvers/monster_geas_wrapper.hh"

extern "C" {

void* monster_geas_create() {
    return new MonsterGroup::MonsterGeasSolver();
}

void monster_geas_destroy(void* solver) {
    delete static_cast<MonsterGroup::MonsterGeasSolver*>(solver);
}

int monster_geas_solve(void* solver, int num_traits, uint64_t* result) {
    auto* geas_solver = static_cast<MonsterGroup::MonsterGeasSolver*>(solver);
    auto solution = geas_solver->solve_trait_mapping(num_traits);
    
    if (solution.empty()) return 0;
    
    for (size_t i = 0; i < solution.size(); i++) {
        result[i] = solution[i];
    }
    
    return static_cast<int>(solution.size());
}

bool monster_geas_verify(void* solver, const uint64_t* elements, int count) {
    auto* geas_solver = static_cast<MonsterGroup::MonsterGeasSolver*>(solver);
    std::vector<uint64_t> vec(elements, elements + count);
    return geas_solver->verify_solution(vec);
}

} // extern "C"
