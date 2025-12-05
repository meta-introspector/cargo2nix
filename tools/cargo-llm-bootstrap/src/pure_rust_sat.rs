/// Pure Rust SAT Solver for Monster Group Constraints
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Variable(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Literal {
    pub var: Variable,
    pub negated: bool,
}

#[derive(Debug, Clone)]
pub struct Clause {
    pub literals: Vec<Literal>,
}

#[derive(Debug)]
pub struct SATSolver {
    pub clauses: Vec<Clause>,
    pub variables: HashSet<Variable>,
    pub assignment: HashMap<Variable, bool>,
}

impl Literal {
    pub fn new(var: Variable, negated: bool) -> Self {
        Self { var, negated }
    }
    
    pub fn pos(var: Variable) -> Self {
        Self::new(var, false)
    }
    
    pub fn neg(var: Variable) -> Self {
        Self::new(var, true)
    }
    
    pub fn eval(&self, assignment: &HashMap<Variable, bool>) -> Option<bool> {
        assignment.get(&self.var).map(|&val| if self.negated { !val } else { val })
    }
}

impl Clause {
    pub fn new(literals: Vec<Literal>) -> Self {
        Self { literals }
    }
    
    pub fn is_satisfied(&self, assignment: &HashMap<Variable, bool>) -> bool {
        self.literals.iter().any(|lit| lit.eval(assignment) == Some(true))
    }
    
    pub fn is_unit(&self, assignment: &HashMap<Variable, bool>) -> Option<Literal> {
        let mut unassigned = None;
        let mut satisfied = false;
        
        for &lit in &self.literals {
            match lit.eval(assignment) {
                Some(true) => { satisfied = true; break; }
                Some(false) => continue,
                None => {
                    if unassigned.is_some() { return None; }
                    unassigned = Some(lit);
                }
            }
        }
        
        if satisfied { None } else { unassigned }
    }
}

impl SATSolver {
    pub fn new() -> Self {
        Self {
            clauses: Vec::new(),
            variables: HashSet::new(),
            assignment: HashMap::new(),
        }
    }
    
    pub fn add_clause(&mut self, clause: Clause) {
        for lit in &clause.literals {
            self.variables.insert(lit.var);
        }
        self.clauses.push(clause);
    }
    
    /// DPLL algorithm with unit propagation
    pub fn solve(&mut self) -> bool {
        self.assignment.clear();
        self.dpll()
    }
    
    fn dpll(&mut self) -> bool {
        // Unit propagation
        loop {
            let mut propagated = false;
            for i in 0..self.clauses.len() {
                if let Some(unit_lit) = self.clauses[i].is_unit(&self.assignment) {
                    self.assignment.insert(unit_lit.var, !unit_lit.negated);
                    propagated = true;
                }
            }
            if !propagated { break; }
        }
        
        // Check for conflicts
        for clause in &self.clauses {
            if !clause.is_satisfied(&self.assignment) && 
               clause.literals.iter().all(|lit| lit.eval(&self.assignment).is_some()) {
                return false; // Conflict
            }
        }
        
        // Check if all clauses satisfied
        if self.clauses.iter().all(|c| c.is_satisfied(&self.assignment)) {
            return true;
        }
        
        // Choose unassigned variable
        let unassigned_var = self.variables.iter()
            .find(|&&var| !self.assignment.contains_key(&var))
            .copied();
            
        if let Some(var) = unassigned_var {
            // Try true
            let mut assignment_backup = self.assignment.clone();
            self.assignment.insert(var, true);
            if self.dpll() { return true; }
            
            // Try false
            self.assignment = assignment_backup;
            self.assignment.insert(var, false);
            self.dpll()
        } else {
            true // All variables assigned
        }
    }
    
    pub fn get_assignment(&self) -> &HashMap<Variable, bool> {
        &self.assignment
    }
}

/// Monster Group SAT encoding
pub struct MonsterSATEncoder {
    solver: SATSolver,
    prime_vars: HashMap<u64, Variable>,
    module_vars: HashMap<String, Variable>,
    next_var_id: u32,
}

impl MonsterSATEncoder {
    pub fn new() -> Self {
        Self {
            solver: SATSolver::new(),
            prime_vars: HashMap::new(),
            module_vars: HashMap::new(),
            next_var_id: 1,
        }
    }
    
    fn new_var(&mut self) -> Variable {
        let var = Variable(self.next_var_id);
        self.next_var_id += 1;
        var
    }
    
    /// Encode Monster Group constraint: rustc ≡ M
    pub fn encode_monster_constraint(&mut self, assignments: &[(String, u64, u32)]) {
        // assignments: (module_name, prime, exponent)
        
        // Create variables for each prime and module
        for (module, prime, _) in assignments {
            if !self.prime_vars.contains_key(prime) {
                self.prime_vars.insert(*prime, self.new_var());
            }
            if !self.module_vars.contains_key(module) {
                self.module_vars.insert(module.clone(), self.new_var());
            }
            
            // Constraint: module assigned → prime used
            let module_var = self.module_vars[module];
            let prime_var = self.prime_vars[prime];
            
            self.solver.add_clause(Clause::new(vec![
                Literal::neg(module_var),
                Literal::pos(prime_var)
            ]));
        }
        
        // All modules must be assigned
        for module_var in self.module_vars.values() {
            self.solver.add_clause(Clause::new(vec![Literal::pos(*module_var)]));
        }
    }
    
    pub fn solve(&mut self) -> bool {
        self.solver.solve()
    }
    
    pub fn get_solution(&self) -> HashMap<String, bool> {
        let mut result = HashMap::new();
        let assignment = self.solver.get_assignment();
        
        for (module, &var) in &self.module_vars {
            if let Some(&value) = assignment.get(&var) {
                result.insert(module.clone(), value);
            }
        }
        
        result
    }
}
