#include "solvers/monster_mzn_instance.hh"

extern "C" {

void* monster_mzn_create() {
    return new MonsterGroup::MonsterMznSolver();
}

void monster_mzn_destroy(void* solver) {
    delete static_cast<MonsterGroup::MonsterMznSolver*>(solver);
}

const char* monster_mzn_generate_model(void* solver, int num_traits) {
    auto* mzn_solver = static_cast<MonsterGroup::MonsterMznSolver*>(solver);
    static std::string model = mzn_solver->generate_mzn_model(num_traits);
    return model.c_str();
}

} // extern "C"
