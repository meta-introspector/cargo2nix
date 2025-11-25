#include "../../include/monster_ast.hh"

extern "C" {

bool monster_hecke_is_valid(int32_t value) {
    return value == MonsterGroup::HECKE_POS || value == MonsterGroup::HECKE_NEG;
}

int32_t monster_hecke_from_element(uint64_t element) {
    return (element % 2 == 0) ? MonsterGroup::HECKE_POS : MonsterGroup::HECKE_NEG;
}

bool monster_hecke_equals(int32_t a, int32_t b) {
    return monster_hecke_is_valid(a) && monster_hecke_is_valid(b) && a == b;
}

}
