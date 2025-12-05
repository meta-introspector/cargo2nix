pub struct HiddenLatticeComponent {
    pub visible_dims: usize,
    pub hidden_dims: usize,
}

impl HiddenLatticeComponent {
    pub fn new(visible: usize, hidden: usize) -> Self {
        Self {
            visible_dims: visible,
            hidden_dims: hidden,
        }
    }

    pub fn total_dimensions(&self) -> usize {
        self.visible_dims + self.hidden_dims
    }

    pub fn is_hidden(&self, dim: usize) -> bool {
        dim >= self.visible_dims
    }
}
