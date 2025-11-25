#include "solvers/monster_atlantis_wrapper.hh"

extern "C" {

void* monster_atlantis_create() {
    return new MonsterGroup::MonsterAtlantisSolver();
}

void monster_atlantis_destroy(void* solver) {
    delete static_cast<MonsterGroup::MonsterAtlantisSolver*>(solver);
}

int monster_atlantis_solve(void* solver, int num_traits, uint64_t* result) {
    auto* atlantis_solver = static_cast<MonsterGroup::MonsterAtlantisSolver*>(solver);
    auto solution = atlantis_solver->solve_trait_mapping(num_traits);
    
    if (solution.empty()) return 0;
    
    for (size_t i = 0; i < solution.size(); i++) {
        result[i] = solution[i];
    }
    
    return static_cast<int>(solution.size());
}

bool monster_atlantis_verify(void* solver, const uint64_t* elements, int count) {
    auto* atlantis_solver = static_cast<MonsterGroup::MonsterAtlantisSolver*>(solver);
    std::vector<uint64_t> vec(elements, elements + count);
    return atlantis_solver->verify_solution(vec);
}

} // extern "C"
