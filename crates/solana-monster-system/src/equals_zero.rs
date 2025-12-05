pub struct EqualsZero {
    pub result: i64,
}

impl EqualsZero {
    pub fn new() -> Self {
        Self { result: 0 }
    }

    pub fn verify(&self) -> bool {
        self.result == 0
    }
}
