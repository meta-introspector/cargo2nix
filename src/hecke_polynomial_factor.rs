pub struct HeckePolynomial {
    pub coefficients: Vec<i64>,
}

pub struct Factor {
    pub expression: i64,
}

impl HeckePolynomial {
    pub fn new(coefficients: Vec<i64>) -> Self {
        Self { coefficients }
    }
}

impl Factor {
    pub fn one_plus_two() -> Self {
        Self { expression: 1 + 2 }
    }

    pub fn compute_with_hecke(&self, hecke: &HeckePolynomial) -> i64 {
        hecke.coefficients.iter().sum::<i64>() * self.expression
    }

    pub fn is_three(&self) -> bool {
        self.expression == 3
    }
}
