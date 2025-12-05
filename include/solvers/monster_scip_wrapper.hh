#pragma once

#include "monster_scip_solver.hh"

extern "C" {
    void* monster_scip_create();
    void monster_scip_destroy(void* solver);
    int monster_scip_solve(void* solver, int num_traits, uint64_t* result);
    bool monster_scip_verify(void* solver, const uint64_t* elements, int count);
}
