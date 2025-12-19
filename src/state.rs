use crate::symbol::Symbol;
use std::{ collections::HashMap, sync::{ Arc, RwLock, RwLockReadGuard, Weak } };

#[allow(dead_code)]
#[derive(Debug)]
pub struct State {
    pub name: String,
    pub transactions: RwLock<HashMap<Symbol, Vec<Weak<State>>>>,
    pub finishing: bool,
}

impl State {
    pub fn new(name: impl Into<String>, finishing: bool) -> Arc<Self> {
        let state = Self {
            name: name.into(),
            transactions: RwLock::new(HashMap::new()),
            finishing,
        };

        Arc::new(state)
    }

    pub fn add_transaction(&self, symbol: Symbol, state: &Arc<State>) {
        match self.transactions.write() {
            Ok(mut table) => {
                let list = table.entry(symbol).or_default();
                list.push(Arc::downgrade(state));
            }
            Err(err) => panic!("{err}"),
        };
    }

    pub fn get_transaction(&self) -> RwLockReadGuard<'_, HashMap<Symbol, Vec<Weak<State>>>> {
        match self.transactions.read() {
            Ok(table) => {
                return table;
            }
            Err(err) => panic!("{err}"),
        };
    }
}
