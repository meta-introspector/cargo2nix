pub struct DiracConsistency {
    pub analytical_index: i64,
    pub topological_index: i64,
}

impl DiracConsistency {
    pub fn new(analytical: i64, topological: i64) -> Self {
        Self {
            analytical_index: analytical,
            topological_index: topological,
        }
    }

    pub fn atiyah_singer_theorem(&self) -> bool {
        self.analytical_index == self.topological_index
    }

    pub fn index_difference(&self) -> i64 {
        self.analytical_index - self.topological_index
    }

    pub fn is_consistent(&self) -> bool {
        self.index_difference() == 0
    }
}
