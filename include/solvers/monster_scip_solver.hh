#pragma once

#include "../monster_ast.hh"
#include <vector>
#include <set>

// Forward declaration
typedef struct SCIP SCIP;

namespace MonsterGroup {

class MonsterScipSolver {
private:
    SCIP* scip_;
    
public:
    MonsterScipSolver();
    ~MonsterScipSolver();
    
    std::vector<uint64_t> solve_trait_mapping(int num_traits);
    bool verify_solution(const std::vector<uint64_t>& elements);
};

} // namespace MonsterGroup
