pub struct ConwayGroup {
    pub order: u64,
    pub leech_lattice: bool,
}

impl ConwayGroup {
    pub fn co0() -> Self {
        Self {
            order: 8315553613086720000, // |Co0|
            leech_lattice: true,
        }
    }

    pub fn co1() -> Self {
        Self {
            order: 4157776806543360000, // |Co1|
            leech_lattice: true,
        }
    }

    pub fn is_sporadic(&self) -> bool {
        self.leech_lattice
    }
}
