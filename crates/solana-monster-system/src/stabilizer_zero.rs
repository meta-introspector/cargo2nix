pub struct StabilizerGroup {
    pub elements: Vec<i64>,
}

pub struct ZeroVector {
    pub components: Vec<i64>,
}

impl StabilizerGroup {
    pub fn new(elements: Vec<i64>) -> Self {
        Self { elements }
    }

    pub fn act_on_zero(&self, zero_vector: &ZeroVector) -> i64 {
        self.elements.iter().zip(zero_vector.components.iter())
            .map(|(g, v)| g * v)
            .sum()
    }
}

impl ZeroVector {
    pub fn new(size: usize) -> Self {
        Self { components: vec![0; size] }
    }

    pub fn is_zero(&self) -> bool {
        self.components.iter().all(|&x| x == 0)
    }
}

pub fn verify_stabilizer_zero(stabilizer: &StabilizerGroup, zero_vector: &ZeroVector) -> bool {
    stabilizer.act_on_zero(zero_vector) == 0
}
