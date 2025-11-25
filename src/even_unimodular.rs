pub struct EvenUnimodular {
    pub matrix: [[i64; 2]; 2],
    pub determinant: i64,
}

impl EvenUnimodular {
    pub fn new(a: i64, b: i64, c: i64, d: i64) -> Self {
        let matrix = [[a, b], [c, d]];
        let determinant = a * d - b * c;
        Self { matrix, determinant }
    }

    pub fn is_unimodular(&self) -> bool {
        self.determinant.abs() == 1
    }

    pub fn is_even(&self) -> bool {
        self.matrix.iter().flatten().all(|&x| x % 2 == 0)
    }

    pub fn second_layer_symmetry(&self) -> bool {
        self.is_unimodular() && self.is_even()
    }
}
