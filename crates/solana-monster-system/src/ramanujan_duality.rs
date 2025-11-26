pub struct RamanujanDuality {
    pub coefficient: i64,
}

impl RamanujanDuality {
    pub fn new() -> Self {
        Self { coefficient: 2049 }
    }

    pub fn is_ramanujan_factor(&self) -> bool {
        self.coefficient == 2049
    }

    pub fn duality_transform(&self, input: i64) -> i64 {
        input * self.coefficient
    }
}
