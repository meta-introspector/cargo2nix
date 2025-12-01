pub trait RegexMatcher: Send + Sync {
    fn new(re: &str) -> Result<Self, String>
    where
        Self: Sized;
    fn is_match(&self, text: &str) -> bool;
    fn captures<'t>(&'t self, text: &'t str) -> Option<Box<dyn RegexCaptures + 't>>;
}

pub trait RegexCaptures {
    fn get(&self, i: usize) -> Option<&str>;
    fn len(&self) -> usize;
}
