#include "../../include/monster_ast.hh"

extern "C" {

void monster_map_traits_to_elements(const char** trait_names, size_t count, uint64_t* elements) {
    for (size_t i = 0; i < count; i++) {
        elements[i] = monster_trait_get_element(trait_names[i]);
    }
}

void monster_map_elements_to_hecke(const uint64_t* elements, size_t count, int32_t* hecke_values) {
    for (size_t i = 0; i < count; i++) {
        hecke_values[i] = monster_hecke_from_element(elements[i]);
    }
}

}
