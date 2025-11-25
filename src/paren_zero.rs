pub struct ParenZero {
    pub result: i64,
}

impl ParenZero {
    pub fn new() -> Self {
        Self { result: 0 }
    }

    pub fn is_zero(&self) -> bool {
        self.result == 0
    }
}
