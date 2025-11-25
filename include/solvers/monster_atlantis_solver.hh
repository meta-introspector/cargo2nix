#pragma once

#include "../monster_ast.hh"
#include <vector>
#include <set>

namespace MonsterGroup {

class MonsterAtlantisSolver {
public:
    MonsterAtlantisSolver();
    
    std::vector<uint64_t> solve_trait_mapping(int num_traits);
    bool verify_solution(const std::vector<uint64_t>& elements);
};

} // namespace MonsterGroup
