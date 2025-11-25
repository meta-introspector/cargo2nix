#include "../../include/monster_ast.hh"

extern "C" {

bool monster_trait_is_valid(const char* trait_name) {
    return trait_name != nullptr && strlen(trait_name) > 0;
}

uint64_t monster_trait_get_element(const char* trait_name) {
    if (!monster_trait_is_valid(trait_name)) return 0;
    
    uint64_t hash = 0;
    for (const char* p = trait_name; *p; ++p) {
        hash = hash * 31 + static_cast<uint64_t>(*p);
    }
    return hash % MonsterGroup::MONSTER_ORDER;
}

int32_t monster_trait_get_hecke(const char* trait_name) {
    uint64_t element = monster_trait_get_element(trait_name);
    return (element % 2 == 0) ? MonsterGroup::HECKE_POS : MonsterGroup::HECKE_NEG;
}

}
