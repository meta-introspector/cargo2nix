pub struct CodeWord {
    pub bits: Vec<i64>,
}

pub struct HammingDistance {
    pub distance: usize,
}

impl CodeWord {
    pub fn new(bits: Vec<i64>) -> Self {
        Self { bits }
    }
}

impl HammingDistance {
    pub fn compute(word1: &CodeWord, word2: &CodeWord) -> Self {
        let distance = word1
            .bits
            .iter()
            .zip(word2.bits.iter())
            .map(|(a, b)| if a != b { 1 } else { 0 })
            .sum();
        Self { distance }
    }

    pub fn min_distance_check(&self) -> bool {
        self.distance >= 8
    }
}
