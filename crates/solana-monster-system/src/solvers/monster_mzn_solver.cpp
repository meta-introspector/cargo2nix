#include "solvers/monster_mzn_solver.hh"

namespace MonsterGroup {

MonsterMznSolver::MonsterMznSolver() {}

std::string MonsterMznSolver::generate_mzn_model(int num_traits) {
    return "% Monster Group MiniZinc model\n"
           "include \"globals.mzn\";\n"
           "array[1.." + std::to_string(num_traits) + "] of var 0.." + 
           std::to_string(MONSTER_ORDER - 1) + ": traits;\n"
           "constraint all_different(traits);\n"
           "constraint sum(traits) mod 24 = 0;\n"
           "solve satisfy;\n";
}

} // namespace MonsterGroup
