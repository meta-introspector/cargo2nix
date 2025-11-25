pub struct KTheoryEquivalence {
    pub x: i64,
    pub ko_x: i64,
}

impl KTheoryEquivalence {
    pub fn new(x: i64) -> Self {
        Self { x, ko_x: x }
    }

    pub fn is_equivalent(&self) -> bool {
        self.x == self.ko_x
    }
}
