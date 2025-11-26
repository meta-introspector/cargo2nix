pub struct GriessAlgebra {
    pub dimension: usize,
    pub is_associative: bool,
}

impl GriessAlgebra {
    pub fn new() -> Self {
        Self {
            dimension: 196884, // Griess algebra dimension
            is_associative: false,
        }
    }

    pub fn is_non_associative(&self) -> bool {
        !self.is_associative
    }

    pub fn monster_connection(&self) -> bool {
        self.dimension == 196884 && self.is_non_associative()
    }
}
