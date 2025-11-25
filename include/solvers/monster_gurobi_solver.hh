#pragma once

#include "../monster_ast.hh"
#include <vector>
#include <set>

// Forward declarations
class GRBEnv;
class GRBModel;

namespace MonsterGroup {

class MonsterGurobiSolver {
private:
    GRBEnv env_;
    GRBModel model_;
    
public:
    MonsterGurobiSolver();
    ~MonsterGurobiSolver();
    
    std::vector<uint64_t> solve_trait_mapping(int num_traits);
    bool verify_solution(const std::vector<uint64_t>& elements);
};

} // namespace MonsterGroup
