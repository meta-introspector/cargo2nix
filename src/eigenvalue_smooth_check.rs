pub struct Eigenvalue {
    pub value: f64,
}

pub struct SmoothCheck {
    pub threshold: f64,
}

impl Eigenvalue {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl SmoothCheck {
    pub fn new(threshold: f64) -> Self {
        Self { threshold }
    }

    pub fn check(&self, eigenvalue: &Eigenvalue) -> f64 {
        eigenvalue.value * self.threshold
    }

    pub fn is_smooth_at_two(&self, eigenvalue: &Eigenvalue) -> bool {
        self.check(eigenvalue) == 2.0
    }
}
