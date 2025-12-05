#pragma once

#include "monster_ast.hh"
#include "monster_verifier.hh"

namespace MonsterGroup {

class TraitMapper {
public:
    TraitMapper();
    
    uint64_t map_trait_to_element(const std::string& trait_name);
    int32_t assign_hecke_value(uint64_t element);
    bool verify_trait_consistency(const std::vector<TraitNode>& traits);
};

} // namespace MonsterGroup
