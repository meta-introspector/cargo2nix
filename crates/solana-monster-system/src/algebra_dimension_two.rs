pub struct AlgebraDimension {
    pub dimension: i64,
}

impl AlgebraDimension {
    pub fn new(dimension: i64) -> Self {
        Self { dimension }
    }

    pub fn times_two(&self) -> i64 {
        self.dimension * 2
    }
}
