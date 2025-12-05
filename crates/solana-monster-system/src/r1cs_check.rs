pub struct R1CSCheck {
    pub a: Vec<i64>,
    pub b: Vec<i64>,
    pub c: Vec<i64>,
}

impl R1CSCheck {
    pub fn new(a: Vec<i64>, b: Vec<i64>, c: Vec<i64>) -> Self {
        Self { a, b, c }
    }

    pub fn verify(&self, witness: &[i64]) -> bool {
        let a_val = dot_product(&self.a, witness);
        let b_val = dot_product(&self.b, witness);
        let c_val = dot_product(&self.c, witness);

        a_val * b_val == c_val
    }
}

fn dot_product(coeffs: &[i64], witness: &[i64]) -> i64 {
    coeffs.iter().zip(witness.iter()).map(|(a, w)| a * w).sum()
}
