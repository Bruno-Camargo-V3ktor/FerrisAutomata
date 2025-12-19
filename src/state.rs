use crate::symbol::Symbol;
use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};

#[derive(Debug)]
pub struct State {
    pub name: String,
    pub transactions: HashMap<Symbol, Vec<Weak<State>>>,
    pub finishing: bool,
}

impl State {
    pub fn new(name: impl Into<String>, finishing: bool) -> Arc<Self> {
        let state = Self {
            name: name.into(),
            transactions: HashMap::new(),
            finishing,
        };

        Arc::new(state)
    }

    pub fn add_transaction(&mut self, symbol: Symbol, state: Weak<State>) {
        let value = self.transactions.entry(symbol).or_default();
        value.push(state);
    }
}
