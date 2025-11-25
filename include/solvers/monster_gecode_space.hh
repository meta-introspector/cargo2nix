#pragma once

#include <gecode/int.hh>
#include <gecode/search.hh>
#include "../monster_ast.hh"

using namespace Gecode;

namespace MonsterGroup {

class MonsterSpace : public Space {
public:
  IntVarArray traits;
  IntVarArray hecke_values;
  
  MonsterSpace(int num_traits);
  MonsterSpace(MonsterSpace& s);
  
  virtual Space* copy();
  void print() const;
};

} // namespace MonsterGroup
