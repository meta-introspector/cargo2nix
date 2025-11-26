#include "../include/monster_ast.hh"
#include "../include/monster_verifier.hh"
#include "../include/trait_mapper.hh"

extern "C" {

// Environment management
void* monster_env_new() {
    return new MonsterGroup::TraitMapper();
}

void monster_env_free(void* env) {
    delete static_cast<MonsterGroup::TraitMapper*>(env);
}

// Trait operations
uint64_t monster_trait_to_element(const char* trait_name) {
    return monster_trait_get_element(trait_name);
}

int32_t monster_trait_to_hecke(const char* trait_name) {
    return monster_trait_get_hecke(trait_name);
}

// Element verification
int monster_element_verify(uint64_t element) {
    return MonsterGroup::MonsterVerifier::verify_element(element) ? 1 : 0;
}

int monster_hecke_verify(int32_t value) {
    return MonsterGroup::MonsterVerifier::verify_hecke(value) ? 1 : 0;
}

// Constraint verification
int monster_verify_modular(const uint64_t* elements, size_t count) {
    std::vector<uint64_t> vec(elements, elements + count);
    return MonsterGroup::MonsterVerifier::verify_modular_constraint(vec) ? 1 : 0;
}

int monster_verify_orbit(uint64_t a, uint64_t b) {
    return MonsterGroup::MonsterVerifier::verify_orbit_preservation(a, b) ? 1 : 0;
}

// Batch operations
void monster_map_traits_batch(const char** trait_names, size_t count, uint64_t* elements) {
    monster_map_traits_to_elements(trait_names, count, elements);
}

void monster_map_hecke_batch(const uint64_t* elements, size_t count, int32_t* hecke_values) {
    monster_map_elements_to_hecke(elements, count, hecke_values);
}

// Full verification
int monster_verify_complete(const uint64_t* elements, size_t count) {
    return monster_verify_full(elements, count) ? 1 : 0;
}

// Version info
const char* monster_get_version() {
    return "Monster Group Cargo2Nix v1.0.0";
}

} // extern "C"
