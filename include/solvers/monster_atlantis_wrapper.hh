#pragma once

#include "monster_atlantis_solver.hh"

extern "C" {
    void* monster_atlantis_create();
    void monster_atlantis_destroy(void* solver);
    int monster_atlantis_solve(void* solver, int num_traits, uint64_t* result);
    bool monster_atlantis_verify(void* solver, const uint64_t* elements, int count);
}
