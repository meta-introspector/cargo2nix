#[derive(Debug, Clone)]
pub struct Constraint {
    pub id: usize,
    pub a: i64,
    pub b: i64,
    pub c: i64,
}

#[derive(Debug, Clone)]
pub struct Constraints {
    pub list: Vec<Constraint>,
}

impl Constraint {
    pub fn new(id: usize, a: i64, b: i64, c: i64) -> Self {
        Self { id, a, b, c }
    }

    pub fn verify(&self, witness: i64) -> bool {
        // Placeholder for actual verification logic
        // For now, let's say it always passes if witness is positive
        witness > 0
    }
}

impl Constraints {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub fn add(&mut self, constraint: Constraint) {
        self.list.push(constraint);
    }

    pub fn verify_all(&self, witness: i64) -> bool {
        self.list.iter().all(|c| c.verify(witness))
    }
}
