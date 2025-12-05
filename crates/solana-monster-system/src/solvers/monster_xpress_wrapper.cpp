#include "solvers/monster_xpress_wrapper.hh"

extern "C" {

void* monster_xpress_create() {
    return new MonsterGroup::MonsterXpressSolver();
}

void monster_xpress_destroy(void* solver) {
    delete static_cast<MonsterGroup::MonsterXpressSolver*>(solver);
}

int monster_xpress_solve(void* solver, int num_traits, uint64_t* result) {
    auto* xpress_solver = static_cast<MonsterGroup::MonsterXpressSolver*>(solver);
    auto solution = xpress_solver->solve_trait_mapping(num_traits);
    
    if (solution.empty()) return 0;
    
    for (size_t i = 0; i < solution.size(); i++) {
        result[i] = solution[i];
    }
    
    return static_cast<int>(solution.size());
}

bool monster_xpress_verify(void* solver, const uint64_t* elements, int count) {
    auto* xpress_solver = static_cast<MonsterGroup::MonsterXpressSolver*>(solver);
    std::vector<uint64_t> vec(elements, elements + count);
    return xpress_solver->verify_solution(vec);
}

} // extern "C"
