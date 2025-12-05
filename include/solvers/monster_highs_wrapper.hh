#pragma once

#include "monster_highs_solver.hh"

extern "C" {
    void* monster_highs_create();
    void monster_highs_destroy(void* solver);
    int monster_highs_solve(void* solver, int num_traits, uint64_t* result);
    bool monster_highs_verify(void* solver, const uint64_t* elements, int count);
}
