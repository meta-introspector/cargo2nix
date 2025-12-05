pub struct ParenOne {
    pub result: i64,
}

impl ParenOne {
    pub fn new() -> Self {
        Self { result: 1 }
    }

    pub fn is_one(&self) -> bool {
        self.result == 1
    }
}
