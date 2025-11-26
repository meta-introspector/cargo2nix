#include "solvers/monster_highs_wrapper.hh"

extern "C" {

void* monster_highs_create() {
    return new MonsterGroup::MonsterHighsSolver();
}

void monster_highs_destroy(void* solver) {
    delete static_cast<MonsterGroup::MonsterHighsSolver*>(solver);
}

int monster_highs_solve(void* solver, int num_traits, uint64_t* result) {
    auto* highs_solver = static_cast<MonsterGroup::MonsterHighsSolver*>(solver);
    auto solution = highs_solver->solve_trait_mapping(num_traits);
    
    if (solution.empty()) return 0;
    
    for (size_t i = 0; i < solution.size(); i++) {
        result[i] = solution[i];
    }
    
    return static_cast<int>(solution.size());
}

bool monster_highs_verify(void* solver, const uint64_t* elements, int count) {
    auto* highs_solver = static_cast<MonsterGroup::MonsterHighsSolver*>(solver);
    std::vector<uint64_t> vec(elements, elements + count);
    return highs_solver->verify_solution(vec);
}

} // extern "C"
