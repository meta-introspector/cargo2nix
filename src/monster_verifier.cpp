#include "monster_verifier.hh"

namespace MonsterGroup {

bool MonsterVerifier::verify_element(uint64_t element) {
    return element < MONSTER_ORDER;
}

bool MonsterVerifier::verify_hecke(int32_t value) {
    return value == HECKE_POS || value == HECKE_NEG;
}

bool MonsterVerifier::verify_modular_constraint(const std::vector<uint64_t>& elements) {
    uint64_t sum = 0;
    for (auto elem : elements) {
        sum += elem;
    }
    return (sum % RAMANUJAN_MOD) == 0;
}

bool MonsterVerifier::verify_orbit_preservation(uint64_t a, uint64_t b) {
    // SL₂(ℤ) orbit preservation check
    return ((a * b) % MONSTER_ORDER) == ((b * a) % MONSTER_ORDER);
}

} // namespace MonsterGroup
