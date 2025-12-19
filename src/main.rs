use crate::{ machine::Machine, state::State, symbol::Symbol };

mod machine;
mod state;
mod symbol;
mod process;

fn main() {
    let q0 = State::new("q0", false);
    let q1 = State::new("q1", false);
    let q2 = State::new("q2", false);
    let q3 = State::new("q3", true);

    q0.add_transaction(Symbol::Letter('a'), &q1);
    q0.add_transaction(Symbol::Letter('b'), &q2);

    q1.add_transaction(Symbol::Letter('a'), &q3);
    q1.add_transaction(Symbol::Letter('b'), &q2);

    q2.add_transaction(Symbol::Letter('a'), &q1);
    q2.add_transaction(Symbol::Letter('b'), &q3);

    q3.add_transaction(Symbol::Letter('a'), &q3);
    q3.add_transaction(Symbol::Letter('b'), &q3);

    let machine = Machine::new(&q0);

    let res = machine.analyze("aaba");
    println!("{res}");

    let res = machine.analyze("abab");
    println!("{res}");
}
