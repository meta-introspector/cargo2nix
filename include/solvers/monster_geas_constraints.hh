#pragma once

#include "../monster_ast.hh"

namespace MonsterGroup {

bool post_monster_constraints(void* solver, int num_traits);
bool post_all_different_constraint(void* solver, void** vars, int count);
bool post_modular_constraint(void* solver, void** vars, int count, int modulus);

} // namespace MonsterGroup
