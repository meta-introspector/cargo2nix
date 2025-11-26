#include "solvers/monster_gecode_solver.hh"

namespace MonsterGroup {

MonsterGecodeSolver::MonsterGecodeSolver() {}

std::vector<uint64_t> MonsterGecodeSolver::solve_trait_mapping(int num_traits) {
  MonsterSpace* space = new MonsterSpace(num_traits);
  
  DFS<MonsterSpace> engine(space);
  delete space;
  
  std::vector<uint64_t> result;
  
  if (MonsterSpace* solution = engine.next()) {
    for (int i = 0; i < num_traits; i++) {
      result.push_back(solution->traits[i].val());
    }
    delete solution;
  }
  
  return result;
}

bool MonsterGecodeSolver::verify_solution(const std::vector<uint64_t>& elements) {
  // Check Monster Group constraints
  for (auto elem : elements) {
    if (elem >= MONSTER_ORDER) return false;
  }
  
  // Check modular constraint
  uint64_t sum = 0;
  for (auto elem : elements) {
    sum += elem;
  }
  
  return (sum % RAMANUJAN_MOD) == 0;
}

} // namespace MonsterGroup
