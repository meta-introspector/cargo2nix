#pragma once

#include "../monster_ast.hh"
#include <string>

namespace MonsterGroup {

class MonsterMznSolver {
public:
    MonsterMznSolver();
    std::string generate_mzn_model(int num_traits);
};

} // namespace MonsterGroup
