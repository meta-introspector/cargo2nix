pub struct KTheoryDimension {
    pub dimension: i64,
}

impl KTheoryDimension {
    pub fn new(dimension: i64) -> Self {
        Self { dimension }
    }

    pub fn mod8(&self) -> i64 {
        self.dimension % 8
    }

    pub fn is_zero_mod8(&self) -> bool {
        self.mod8() == 0
    }
}
