#include "solvers/monster_fzn_instance.hh"

extern "C" {

void* monster_fzn_create() {
    return new MonsterGroup::MonsterFznSolver();
}

void monster_fzn_destroy(void* solver) {
    delete static_cast<MonsterGroup::MonsterFznSolver*>(solver);
}

int monster_fzn_solve(void* solver, int num_traits, uint64_t* result) {
    auto* fzn_solver = static_cast<MonsterGroup::MonsterFznSolver*>(solver);
    auto solution = fzn_solver->solve_trait_mapping(num_traits);
    
    if (solution.empty()) return 0;
    
    for (size_t i = 0; i < solution.size(); i++) {
        result[i] = solution[i];
    }
    
    return static_cast<int>(solution.size());
}

const char* monster_fzn_generate_model(void* solver, int num_traits) {
    auto* fzn_solver = static_cast<MonsterGroup::MonsterFznSolver*>(solver);
    static std::string model = fzn_solver->generate_fzn_model(num_traits);
    return model.c_str();
}

} // extern "C"
