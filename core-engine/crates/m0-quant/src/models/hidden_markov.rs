
#[derive(Debug, Clone)]
pub struct HiddenMarkov {
    pub states: usize,
}

impl HiddenMarkov {
    pub fn new(states: usize) -> Self {
        Self { states: states.max(2) }
    }
}
