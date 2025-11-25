#pragma once

#include "monster_chuffed_solver.hh"

extern "C" {
    void* monster_chuffed_create();
    void monster_chuffed_destroy(void* solver);
    int monster_chuffed_solve(void* solver, int num_traits, uint64_t* result);
    bool monster_chuffed_verify(void* solver, const uint64_t* elements, int count);
}
