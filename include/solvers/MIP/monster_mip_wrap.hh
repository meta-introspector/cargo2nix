#pragma once

#include "monster_mip_instance.hh"

extern "C" {
    void* monster_mip_create();
    void monster_mip_destroy(void* instance);
    void monster_mip_add_var(void* instance, int id, double lb, double ub, int integer);
    void monster_mip_add_constraint(void* instance, const int* var_ids, const double* coeffs, 
                                   int count, double rhs, int type);
    void monster_mip_set_objective(void* instance, const int* var_ids, const double* coeffs, 
                                  int count, int minimize);
    int monster_mip_solve(void* instance, double* solution);
    int monster_mip_check_feasible(void* instance, const double* solution, int count);
}
