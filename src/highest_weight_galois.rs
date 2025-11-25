pub struct HighestWeight {
    pub weight: i64,
}

pub struct TwoAdicGalois {
    pub representation: Vec<i64>,
    pub prime: i64,
}

impl HighestWeight {
    pub fn new(weight: i64) -> Self {
        Self { weight }
    }
}

impl TwoAdicGalois {
    pub fn new(representation: Vec<i64>) -> Self {
        Self {
            representation,
            prime: 2,
        }
    }

    pub fn is_two_adic(&self) -> bool {
        self.prime == 2
    }

    pub fn highest_weight_action(&self, hw: &HighestWeight) -> i64 {
        self.representation.iter().sum::<i64>() * hw.weight
    }
}
