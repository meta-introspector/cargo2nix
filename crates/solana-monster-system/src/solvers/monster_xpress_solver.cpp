#include "solvers/monster_xpress_solver.hh"
#include <xprs.h>

namespace MonsterGroup {

MonsterXpressSolver::MonsterXpressSolver() : prob_(nullptr) {
    XPRSinit(nullptr);
    XPRScreateprob(&prob_);
    XPRSsetintcontrol(prob_, XPRS_OUTPUTLOG, 0);
}

MonsterXpressSolver::~MonsterXpressSolver() {
    if (prob_) {
        XPRSdestroyprob(prob_);
    }
    XPRSfree();
}

std::vector<uint64_t> MonsterXpressSolver::solve_trait_mapping(int num_traits) {
    // Variables: trait elements [0, MONSTER_ORDER-1]
    std::vector<double> obj(num_traits, 1.0);
    std::vector<double> lb(num_traits, 0.0);
    std::vector<double> ub(num_traits, static_cast<double>(MONSTER_ORDER - 1));
    std::vector<char> vtype(num_traits, 'I');
    
    XPRSaddcols(prob_, num_traits, obj.data(), nullptr, nullptr, nullptr, lb.data(), ub.data());
    
    for (int i = 0; i < num_traits; i++) {
        XPRSchgcoltype(prob_, 1, &i, &vtype[i]);
    }
    
    // All-different constraints
    for (int i = 0; i < num_traits; i++) {
        for (int j = i + 1; j < num_traits; j++) {
            int indices[2] = {i, j};
            double coeffs[2] = {1.0, -1.0};
            XPRSaddrows(prob_, 1, 2, nullptr, indices, coeffs, nullptr, &coeffs[0], nullptr);
            
            coeffs[0] = -1.0; coeffs[1] = 1.0;
            XPRSaddrows(prob_, 1, 2, nullptr, indices, coeffs, nullptr, &coeffs[0], nullptr);
        }
    }
    
    // Modular constraint: sum ≡ 0 (mod 24)
    double k_obj = 0.0, k_lb = 0.0, k_ub = XPRS_PLUSINFINITY;
    char k_type = 'I';
    XPRSaddcols(prob_, 1, &k_obj, nullptr, nullptr, nullptr, &k_lb, &k_ub);
    XPRSchgcoltype(prob_, 1, &num_traits, &k_type);
    
    std::vector<int> mod_indices(num_traits + 1);
    std::vector<double> mod_coeffs(num_traits + 1);
    for (int i = 0; i < num_traits; i++) {
        mod_indices[i] = i;
        mod_coeffs[i] = 1.0;
    }
    mod_indices[num_traits] = num_traits;
    mod_coeffs[num_traits] = -24.0;
    
    double rhs = 0.0;
    XPRSaddrows(prob_, 1, num_traits + 1, nullptr, mod_indices.data(), mod_coeffs.data(), 
                nullptr, &rhs, nullptr);
    
    // Solve
    XPRSmipoptimize(prob_, "");
    
    std::vector<uint64_t> result;
    int status;
    XPRSgetintattrib(prob_, XPRS_MIPSTATUS, &status);
    
    if (status == XPRS_MIP_OPTIMAL) {
        std::vector<double> solution(num_traits);
        XPRSgetmipsol(prob_, solution.data(), nullptr);
        
        for (int i = 0; i < num_traits; i++) {
            result.push_back(static_cast<uint64_t>(solution[i] + 0.5));
        }
    }
    
    return result;
}

bool MonsterXpressSolver::verify_solution(const std::vector<uint64_t>& elements) {
    // Monster Group order check
    for (auto elem : elements) {
        if (elem >= MONSTER_ORDER) return false;
    }
    
    // All different check
    std::set<uint64_t> unique_elements(elements.begin(), elements.end());
    if (unique_elements.size() != elements.size()) return false;
    
    // Modular constraint check
    uint64_t sum = 0;
    for (auto elem : elements) {
        sum += elem;
    }
    
    return (sum % RAMANUJAN_MOD) == 0;
}

} // namespace MonsterGroup
