#include "trait_mapper.hh"
#include <functional>

namespace MonsterGroup {

TraitMapper::TraitMapper() {}

uint64_t TraitMapper::map_trait_to_element(const std::string& trait_name) {
    std::hash<std::string> hasher;
    uint64_t hash = hasher(trait_name);
    return hash % MONSTER_ORDER;
}

int32_t TraitMapper::assign_hecke_value(uint64_t element) {
    return (element % 2 == 0) ? HECKE_POS : HECKE_NEG;
}

bool TraitMapper::verify_trait_consistency(const std::vector<TraitNode>& traits) {
    std::vector<uint64_t> elements;
    for (const auto& trait : traits) {
        elements.push_back(trait.monster_element);
    }
    return MonsterVerifier::verify_modular_constraint(elements);
}

} // namespace MonsterGroup
