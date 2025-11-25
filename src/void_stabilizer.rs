pub struct VoidStabilizer {
    pub elements: Vec<i64>,
}

impl VoidStabilizer {
    pub fn new() -> Self {
        Self { elements: vec![] }
    }

    pub fn stabilizes_void(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn act_on_void(&self) -> Option<i64> {
        None // Void has no elements to act upon
    }
}
