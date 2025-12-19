use crate::symbol::Symbol;
use std::{collections::HashMap, sync::Weak};

#[derive(Debug)]
pub struct State {
    pub name: String,
    pub transactions: HashMap<Symbol, Vec<Weak<State>>>,
}

impl State {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            transactions: HashMap::new(),
        }
    }

    pub fn add_transaction(&mut self, symbol: Symbol, state: Weak<State>) {
        let value = self.transactions.entry(symbol).or_default();
        value.push(state);
    }
}
