pub struct FermionState {
    pub occupation: i64,
}

pub struct KTheoryIndex {
    pub index: i64,
}

impl FermionState {
    pub fn new(occupation: i64) -> Self {
        Self { occupation }
    }
}

impl KTheoryIndex {
    pub fn new(index: i64) -> Self {
        Self { index }
    }

    pub fn compute_with_fermion(&self, fermion: &FermionState) -> i64 {
        fermion.occupation * self.index
    }

    pub fn is_unit_index(&self, fermion: &FermionState) -> bool {
        self.compute_with_fermion(fermion) == 1
    }
}
