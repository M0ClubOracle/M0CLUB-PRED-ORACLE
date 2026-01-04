
use std::collections::HashSet;

#[derive(Debug)]
pub struct DedupeSet {
    seen: HashSet<String>,
    max: usize,
}

impl DedupeSet {
    pub fn new(max: usize) -> Self {
        Self { seen: HashSet::new(), max }
    }

    pub fn insert(&mut self, key: String) -> bool {
        if self.seen.len() >= self.max {
            // Simple reset policy for skeleton.
            self.seen.clear();
        }
        self.seen.insert(key)
    }
}
