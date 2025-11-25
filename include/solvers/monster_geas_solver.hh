#pragma once

#include "../monster_ast.hh"
#include <vector>
#include <set>

// Mock Geas namespace (would normally include geas headers)
namespace geas {
    class solver;
    class intvar;
}

namespace MonsterGroup {

class MonsterGeasSolver {
public:
    MonsterGeasSolver();
    
    std::vector<uint64_t> solve_trait_mapping(int num_traits);
    bool verify_solution(const std::vector<uint64_t>& elements);
};

} // namespace MonsterGroup
