pub struct TopologicalInsulator {
    pub dimension: usize,
    pub ko_class: i64,
}

pub struct KOTheoryClassification {
    pub periodicity: usize, // 8-fold Bott periodicity
}

impl TopologicalInsulator {
    pub fn new(dimension: usize, ko_class: i64) -> Self {
        Self {
            dimension,
            ko_class,
        }
    }

    pub fn is_trivial(&self) -> bool {
        self.ko_class == 0
    }
}

impl KOTheoryClassification {
    pub fn new() -> Self {
        Self { periodicity: 8 }
    }

    pub fn classify(&self, insulator: &TopologicalInsulator) -> i64 {
        insulator.ko_class % (self.periodicity as i64)
    }

    pub fn is_protected(&self, insulator: &TopologicalInsulator) -> bool {
        !insulator.is_trivial()
    }
}
