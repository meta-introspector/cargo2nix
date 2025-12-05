#pragma once

#include "monster_gurobi_solver.hh"

extern "C" {
    void* monster_gurobi_create();
    void monster_gurobi_destroy(void* solver);
    int monster_gurobi_solve(void* solver, int num_traits, uint64_t* result);
    bool monster_gurobi_verify(void* solver, const uint64_t* elements, int count);
}
