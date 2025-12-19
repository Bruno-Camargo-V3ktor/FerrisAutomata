use crate::state::State;
use std::sync::{Arc, Weak};

pub struct Machine {
    pub states: Vec<Arc<State>>,
    pub input: Vec<char>,
    pub initial: Weak<State>,
}

impl Machine {
    pub fn new(states: Vec<Arc<State>>, input: impl Into<String>, initial_pos: usize) -> Self {
        let ref_state = Arc::downgrade(states.get(initial_pos).unwrap());
        Self {
            states,
            initial: ref_state,
            input: input.into().chars().into_iter().collect(),
        }
    }
}
