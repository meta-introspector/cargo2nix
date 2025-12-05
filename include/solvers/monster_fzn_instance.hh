#pragma once

#include "monster_fzn_solver.hh"

extern "C" {
    void* monster_fzn_create();
    void monster_fzn_destroy(void* solver);
    int monster_fzn_solve(void* solver, int num_traits, uint64_t* result);
    const char* monster_fzn_generate_model(void* solver, int num_traits);
}
