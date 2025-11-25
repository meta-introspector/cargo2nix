#include "solvers/monster_chuffed_wrapper.hh"

extern "C" {

void* monster_chuffed_create() {
    return new MonsterGroup::MonsterChuffedSolver();
}

void monster_chuffed_destroy(void* solver) {
    delete static_cast<MonsterGroup::MonsterChuffedSolver*>(solver);
}

int monster_chuffed_solve(void* solver, int num_traits, uint64_t* result) {
    auto* chuffed_solver = static_cast<MonsterGroup::MonsterChuffedSolver*>(solver);
    auto solution = chuffed_solver->solve_trait_mapping(num_traits);
    
    if (solution.empty()) return 0;
    
    for (size_t i = 0; i < solution.size(); i++) {
        result[i] = solution[i];
    }
    
    return static_cast<int>(solution.size());
}

bool monster_chuffed_verify(void* solver, const uint64_t* elements, int count) {
    auto* chuffed_solver = static_cast<MonsterGroup::MonsterChuffedSolver*>(solver);
    std::vector<uint64_t> vec(elements, elements + count);
    return chuffed_solver->verify_solution(vec);
}

} // extern "C"
