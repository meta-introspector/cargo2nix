pub struct PermutationSize {
    pub size: usize,
}

pub struct MinimalFaithfulBound {
    pub bound: usize,
}

impl PermutationSize {
    pub fn new(size: usize) -> Self {
        Self { size }
    }
}

impl MinimalFaithfulBound {
    pub fn new(bound: usize) -> Self {
        Self { bound }
    }

    pub fn compute_product(&self, perm: &PermutationSize) -> usize {
        perm.size * self.bound
    }

    pub fn is_bounded_by(&self, perm: &PermutationSize, m: usize) -> bool {
        self.compute_product(perm) <= m
    }
}
