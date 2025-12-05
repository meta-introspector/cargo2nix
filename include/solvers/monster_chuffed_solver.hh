#pragma once

#include "../monster_ast.hh"
#include <vector>
#include <set>

namespace MonsterGroup {

class MonsterChuffedSolver {
public:
    MonsterChuffedSolver();
    
    std::vector<uint64_t> solve_trait_mapping(int num_traits);
    bool verify_solution(const std::vector<uint64_t>& elements);
    
private:
    bool search_with_backtracking(std::vector<uint64_t>& solution, int index);
    bool is_partial_consistent(const std::vector<uint64_t>& solution, int length);
};

} // namespace MonsterGroup
