pub struct EvenCheck {
    pub result: i64,
}

impl EvenCheck {
    pub fn new(value: i64) -> Self {
        Self {
            result: if value % 2 == 0 { 1 } else { 0 },
        }
    }

    pub fn is_one(&self) -> bool {
        self.result == 1
    }
}
