#pragma once

#include "monster_mzn_solver.hh"

extern "C" {
    void* monster_mzn_create();
    void monster_mzn_destroy(void* solver);
    const char* monster_mzn_generate_model(void* solver, int num_traits);
}
