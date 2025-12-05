#pragma once

#include "monster_geas_solver.hh"

extern "C" {
    void* monster_geas_create();
    void monster_geas_destroy(void* solver);
    int monster_geas_solve(void* solver, int num_traits, uint64_t* result);
    bool monster_geas_verify(void* solver, const uint64_t* elements, int count);
}
