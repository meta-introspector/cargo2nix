#pragma once

#include "../monster_ast.hh"
#include <vector>
#include <set>

// Forward declaration
typedef struct xo_prob_struct* XPRSprob;

namespace MonsterGroup {

class MonsterXpressSolver {
private:
    XPRSprob prob_;
    
public:
    MonsterXpressSolver();
    ~MonsterXpressSolver();
    
    std::vector<uint64_t> solve_trait_mapping(int num_traits);
    bool verify_solution(const std::vector<uint64_t>& elements);
};

} // namespace MonsterGroup
