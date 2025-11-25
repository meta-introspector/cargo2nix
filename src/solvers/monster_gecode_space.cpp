#include "solvers/monster_gecode_space.hh"

namespace MonsterGroup {

MonsterSpace::MonsterSpace(int num_traits) 
  : traits(*this, num_traits, 0, MONSTER_ORDER - 1),
    hecke_values(*this, num_traits, 0, 1) {
  
  // All traits must map to distinct Monster Group elements
  distinct(*this, traits);
  
  // Modular constraint: sum of elements ≡ 0 (mod 24)
  IntVar sum(*this, 0, (MONSTER_ORDER - 1) * num_traits);
  linear(*this, traits, IRT_EQ, sum);
  rel(*this, sum % 24, IRT_EQ, 0);
  
  // Hecke eigenvalue constraints
  for (int i = 0; i < num_traits; i++) {
    rel(*this, hecke_values[i], IRT_GQ, 0);
    rel(*this, hecke_values[i], IRT_LQ, 1);
  }
  
  // Branching strategy
  branch(*this, traits, INT_VAR_SIZE_MIN(), INT_VAL_MIN());
}

MonsterSpace::MonsterSpace(MonsterSpace& s) 
  : Space(s) {
  traits.update(*this, s.traits);
  hecke_values.update(*this, s.hecke_values);
}

Space* MonsterSpace::copy() {
  return new MonsterSpace(*this);
}

void MonsterSpace::print() const {
  std::cout << "Monster elements: " << traits << std::endl;
  std::cout << "Hecke indices: " << hecke_values << std::endl;
}

} // namespace MonsterGroup
