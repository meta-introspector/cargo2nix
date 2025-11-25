#pragma once

#include "../../monster_ast.hh"
#include <vector>
#include <map>

namespace MonsterGroup {

enum class ConstraintType {
    EQUAL,
    LESS_EQUAL,
    GREATER_EQUAL
};

struct Variable {
    int id;
    double lower_bound;
    double upper_bound;
    bool is_integer;
};

struct Constraint {
    std::vector<int> variable_ids;
    std::vector<double> coefficients;
    double rhs;
    ConstraintType type;
};

struct Objective {
    std::vector<int> variable_ids;
    std::vector<double> coefficients;
    bool minimize;
};

class MonsterMipInstance {
private:
    std::map<int, Variable> variables_;
    std::vector<Constraint> constraints_;
    Objective objective_;
    
public:
    MonsterMipInstance();
    ~MonsterMipInstance();
    
    void add_variable(int id, double lb, double ub, bool integer = false);
    void add_constraint(const std::vector<int>& var_ids, 
                       const std::vector<double>& coeffs,
                       double rhs, ConstraintType type);
    void set_objective(const std::vector<int>& var_ids,
                      const std::vector<double>& coeffs,
                      bool minimize = true);
    
    std::vector<double> solve();
    bool is_feasible(const std::vector<double>& solution);
};

} // namespace MonsterGroup
