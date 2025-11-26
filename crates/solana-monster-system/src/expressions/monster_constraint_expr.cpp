#include "../../include/monster_ast.hh"

extern "C" {

bool monster_constraint_all_different(const uint64_t* elements, size_t count) {
    for (size_t i = 0; i < count; i++) {
        for (size_t j = i + 1; j < count; j++) {
            if (elements[i] == elements[j]) return false;
        }
    }
    return true;
}

bool monster_constraint_all_valid(const uint64_t* elements, size_t count) {
    for (size_t i = 0; i < count; i++) {
        if (!monster_element_is_valid(elements[i])) return false;
    }
    return true;
}

}
