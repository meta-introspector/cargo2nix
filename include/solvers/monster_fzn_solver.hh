#pragma once

#include "../monster_ast.hh"
#include <vector>
#include <set>
#include <string>

namespace MonsterGroup {

class MonsterFznSolver {
public:
    MonsterFznSolver();
    
    std::string generate_fzn_model(int num_traits);
    std::vector<uint64_t> solve_trait_mapping(int num_traits);
    bool verify_solution(const std::vector<uint64_t>& elements);
};

} // namespace MonsterGroup
