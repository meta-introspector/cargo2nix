pub struct ReflectionMatrix {
    pub matrix: [[i64; 3]; 3],
}

pub struct Vector {
    pub components: [i64; 3],
}

pub struct Invariance {
    pub value: i64,
}

impl ReflectionMatrix {
    pub fn new() -> Self {
        Self {
            matrix: [[-1, 0, 0], [0, -1, 0], [0, 0, -1]],
        }
    }

    pub fn multiply_vector(&self, vector: &Vector) -> Vector {
        let mut result = [0i64; 3];
        for i in 0..3 {
            for j in 0..3 {
                result[i] += self.matrix[i][j] * vector.components[j];
            }
        }
        Vector { components: result }
    }
}

impl Vector {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self {
            components: [x, y, z],
        }
    }

    pub fn dot(&self, other: &Vector) -> i64 {
        self.components
            .iter()
            .zip(other.components.iter())
            .map(|(a, b)| a * b)
            .sum()
    }
}

impl Invariance {
    pub fn new(value: i64) -> Self {
        Self { value }
    }

    pub fn check_zero(&self) -> bool {
        self.value == 0
    }
}

pub fn verify_reflection_invariance(
    matrix: &ReflectionMatrix,
    vector: &Vector,
    invariance: &Invariance,
) -> bool {
    let reflected = matrix.multiply_vector(vector);
    let result = reflected.dot(vector) * invariance.value;
    result == 0
}
