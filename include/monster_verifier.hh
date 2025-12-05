#pragma once

#include "monster_ast.hh"

namespace MonsterGroup {

class MonsterVerifier {
public:
    static bool verify_element(uint64_t element);
    static bool verify_hecke(int32_t value);
    static bool verify_modular_constraint(const std::vector<uint64_t>& elements);
    static bool verify_orbit_preservation(uint64_t a, uint64_t b);
};

} // namespace MonsterGroup
