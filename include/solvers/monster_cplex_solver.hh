#pragma once

#include <ilcplex/ilocplex.h>
#include "../monster_ast.hh"
#include <vector>

ILOSTLBEGIN

namespace MonsterGroup {

class MonsterCplexSolver {
private:
    IloEnv env;
    IloModel model;
    
public:
    MonsterCplexSolver();
    ~MonsterCplexSolver();
    
    std::vector<uint64_t> solve_trait_mapping(int num_traits);
    bool verify_solution(const std::vector<uint64_t>& elements);
};

} // namespace MonsterGroup
