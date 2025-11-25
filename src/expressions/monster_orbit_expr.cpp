#include "../../include/monster_ast.hh"

extern "C" {

bool monster_orbit_check(uint64_t a, uint64_t b) {
    return ((a * b) % MonsterGroup::MONSTER_ORDER) == ((b * a) % MonsterGroup::MONSTER_ORDER);
}

}
