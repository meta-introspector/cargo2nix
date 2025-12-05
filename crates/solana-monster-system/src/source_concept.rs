pub struct SourceConcept {
    pub id: String,
    pub value: i64,
}

impl SourceConcept {
    pub fn new(id: String, value: i64) -> Self {
        Self { id, value }
    }

    pub fn encode(&self) -> i64 {
        self.value
    }
}
