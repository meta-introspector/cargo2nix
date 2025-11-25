pub struct TheTwo {
    pub value: i64,
}

impl TheTwo {
    pub fn new() -> Self {
        Self { value: 2 }
    }

    pub fn is_the_two(&self) -> bool {
        self.value == 2
    }
}
