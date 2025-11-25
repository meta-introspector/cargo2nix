#pragma once

#include "monster_xpress_solver.hh"

extern "C" {
    void* monster_xpress_create();
    void monster_xpress_destroy(void* solver);
    int monster_xpress_solve(void* solver, int num_traits, uint64_t* result);
    bool monster_xpress_verify(void* solver, const uint64_t* elements, int count);
}
