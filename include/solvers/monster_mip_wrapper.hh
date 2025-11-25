#pragma once

#include "monster_cplex_solver.hh"

extern "C" {
    void* monster_cplex_create();
    void monster_cplex_destroy(void* solver);
    int monster_cplex_solve(void* solver, int num_traits, uint64_t* result);
    bool monster_cplex_verify(void* solver, const uint64_t* elements, int count);
}
