pub struct Volume {
    pub value: f64,
}

pub struct Complement {
    pub total: f64,
}

impl Volume {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl Complement {
    pub fn new(total: f64) -> Self {
        Self { total }
    }

    pub fn compute(&self, volume: &Volume) -> f64 {
        self.total - volume.value
    }
}
