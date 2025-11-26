pub struct DiracOperator {
    pub index: i64,
}

pub struct TopologicalIntegral {
    pub value: i64,
}

impl DiracOperator {
    pub fn new(index: i64) -> Self {
        Self { index }
    }
}

impl TopologicalIntegral {
    pub fn new(value: i64) -> Self {
        Self { value }
    }

    pub fn compute_with_index(&self, dirac: &DiracOperator) -> i64 {
        dirac.index * self.value
    }

    pub fn is_zero_integral(&self, dirac: &DiracOperator) -> bool {
        self.compute_with_index(dirac) == 0
    }
}
