use crate::{ process::Process, state::State };
use std::sync::{ Arc, Weak };

pub struct Machine {
    pub initial: Weak<State>,
}

impl Machine {
    pub fn new(initial: &Arc<State>) -> Self {
        Self {
            initial: Arc::downgrade(initial),
        }
    }

    pub fn analyze(&self, input: impl Into<String>) -> bool {
        let input: Arc<Vec<char>> = Arc::new(input.into().chars().into_iter().collect());

        let mut process = Process::new(self.initial.clone(), input.clone(), 0);
        process.start();

        process.result
    }
}
