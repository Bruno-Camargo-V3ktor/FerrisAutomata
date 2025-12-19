use crate::{ machine::Machine, state::State, symbol::Symbol };

mod machine;
mod state;
mod symbol;
mod process;

fn main() {
    let input = "aaba";

    let q1 = State::new("q1", false);
    let q2 = State::new("q2", false);
    let q3 = State::new("q3", false);
    let q4 = State::new("q4", true);

    q1.add_transaction(Symbol::Letter('a'), &q2);

    let machine = Machine::new(&q1);
    machine.analyze(input);
}
