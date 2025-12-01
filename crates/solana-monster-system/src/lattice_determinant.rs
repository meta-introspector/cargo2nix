pub struct LatticeDeterminant {
    pub value: i64,
}

impl LatticeDeterminant {
    pub fn new(value: i64) -> Self {
        Self { value }
    }

    pub fn compute_2x2(a: i64, b: i64, c: i64, d: i64) -> Self {
        Self {
            value: a * d - b * c,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.value == 0
    }
}
