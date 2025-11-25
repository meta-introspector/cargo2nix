#include "../../include/monster_ast.hh"

extern "C" {

bool monster_element_is_valid(uint64_t element) {
    return element < MonsterGroup::MONSTER_ORDER;
}

uint64_t monster_element_normalize(uint64_t element) {
    return element % MonsterGroup::MONSTER_ORDER;
}

bool monster_element_equals(uint64_t a, uint64_t b) {
    return monster_element_normalize(a) == monster_element_normalize(b);
}

}
