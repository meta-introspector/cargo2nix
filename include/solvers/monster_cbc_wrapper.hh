#pragma once

#include "monster_osicbc_solver.hh"

extern "C" {
    void* monster_cbc_create();
    void monster_cbc_destroy(void* solver);
    int monster_cbc_solve(void* solver, int num_traits, uint64_t* result);
    bool monster_cbc_verify(void* solver, const uint64_t* elements, int count);
}
