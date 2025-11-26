#include "../../include/monster_ast.hh"

extern "C" {

bool monster_modular_check(const uint64_t* elements, size_t count) {
    uint64_t sum = 0;
    for (size_t i = 0; i < count; i++) {
        sum += elements[i];
    }
    return (sum % MonsterGroup::RAMANUJAN_MOD) == 0;
}

uint64_t monster_modular_sum(const uint64_t* elements, size_t count) {
    uint64_t sum = 0;
    for (size_t i = 0; i < count; i++) {
        sum += elements[i];
    }
    return sum;
}

uint32_t monster_modular_remainder(const uint64_t* elements, size_t count) {
    return monster_modular_sum(elements, count) % MonsterGroup::RAMANUJAN_MOD;
}

}
