use std::{ sync::{ Arc, Weak }, thread::{ self, JoinHandle } };
use crate::{ state::State, symbol::Symbol };

pub struct Process {
    actual_state: Weak<State>,
    result: bool,
    sub_process: Vec<JoinHandle<Process>>,
    input: Arc<Vec<char>>,
    pos: usize,
}

impl Process {
    pub fn new(actual_state: Weak<State>, input: Arc<Vec<char>>, pos: usize) -> Self {
        Process { actual_state, result: false, sub_process: vec![], input, pos }
    }

    pub fn start(&mut self) {
        let state = self.actual_state.upgrade().unwrap();

        loop {
            if let Some(states) = state.clone().transactions.get(&Symbol::Empty) {
                for state in states {
                    self.create_subprocess(state.clone(), self.pos);
                }
            }

            if self.input.len() < self.pos {
                let actual_letter = Symbol::Letter(self.input[self.pos]);

                if let Some(states) = state.clone().transactions.get(&actual_letter) {
                    if let Some(state) = states.first() {
                        self.actual_state = state.clone();
                    } else {
                        self.result = false;
                        break;
                    }

                    for state in &states[1..] {
                        self.create_subprocess(state.clone(), self.pos + 1);
                    }
                } else {
                    self.result = state.finishing;
                    break;
                }

                self.pos += 1;
                continue;
            } else {
                self.result = state.finishing;
                break;
            }
        }

        self.compress_result();
    }

    pub fn create_subprocess(&mut self, state: Weak<State>, pos: usize) {
        let process = Process {
            actual_state: state,
            input: self.input.clone(),
            result: false,
            sub_process: vec![],
            pos,
        };

        let handle = thread::spawn(move || {
            let mut process = process;
            process.start();
            process
        });

        self.sub_process.push(handle);
    }

    pub fn compress_result(&mut self) {
        while self.sub_process.len() > 0 {
            let handle = self.sub_process.remove(0);
            match handle.join() {
                Ok(p) => {
                    self.result = self.result || p.result;
                }
                Err(_) => {}
            }
        }
    }
}
