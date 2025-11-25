#include "solvers/monster_gurobi_wrapper.hh"

extern "C" {

void* monster_gurobi_create() {
    return new MonsterGroup::MonsterGurobiSolver();
}

void monster_gurobi_destroy(void* solver) {
    delete static_cast<MonsterGroup::MonsterGurobiSolver*>(solver);
}

int monster_gurobi_solve(void* solver, int num_traits, uint64_t* result) {
    auto* gurobi_solver = static_cast<MonsterGroup::MonsterGurobiSolver*>(solver);
    auto solution = gurobi_solver->solve_trait_mapping(num_traits);
    
    if (solution.empty()) return 0;
    
    for (size_t i = 0; i < solution.size(); i++) {
        result[i] = solution[i];
    }
    
    return static_cast<int>(solution.size());
}

bool monster_gurobi_verify(void* solver, const uint64_t* elements, int count) {
    auto* gurobi_solver = static_cast<MonsterGroup::MonsterGurobiSolver*>(solver);
    std::vector<uint64_t> vec(elements, elements + count);
    return gurobi_solver->verify_solution(vec);
}

} // extern "C"
