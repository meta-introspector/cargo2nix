#pragma once

#include "monster_gecode_space.hh"
#include <vector>

namespace MonsterGroup {

class MonsterGecodeSolver {
public:
  MonsterGecodeSolver();
  
  std::vector<uint64_t> solve_trait_mapping(int num_traits);
  bool verify_solution(const std::vector<uint64_t>& elements);
};

} // namespace MonsterGroup
