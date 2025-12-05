#include "../../include/monster_ast.hh"

extern "C" {

bool monster_verify_full(const uint64_t* elements, size_t count) {
    return monster_constraint_all_valid(elements, count) &&
           monster_constraint_all_different(elements, count) &&
           monster_modular_check(elements, count);
}

}
