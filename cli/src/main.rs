use automata::state::State;
use automata::symbol::Symbol;
use colorz::Colorize;
use std::io::{Read, Result as IOResult};
use std::sync::Arc;
use std::{env::args, fs::File, process::ExitCode};

fn main() -> ExitCode {
    let args: Vec<String> = args().skip(1).collect();

    if args.len() < 2 {
        eprintln!("{}", "Invalid Arguments...".red());
        eprintln!("Usage: {}", "automata file.txt words...".yellow());
        return ExitCode::FAILURE;
    }

    let lines: Vec<Vec<String>> = match open_file(&args[0]) {
        Ok(lines) => lines,
        Err(err) => {
            eprintln!(
                "{} {:?}",
                "File Invalid...\nError:".red(),
                err.kind().yellow()
            );
            return ExitCode::FAILURE;
        }
    }
    .into_iter()
    .map(|str| {
        str.split(";")
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    })
    .collect();

    if lines.len() < 2 {
        eprintln!("{}", "Format from file invalid".red());
        return ExitCode::FAILURE;
    }

    let mut symbols: Vec<Symbol> = lines[0]
        .iter()
        .skip(1)
        .map(|s| {
            let letter = match s.parse::<char>() {
                Ok(c) => c,
                Err(_) => {
                    panic!("{} {}", "Invalid Symbol:".red(), s.yellow());
                }
            };

            Symbol::Letter(letter)
        })
        .collect();
    symbols.push(Symbol::Empty);

    let mut states: Vec<Arc<State>> = Vec::with_capacity(lines.len());
    for col in 0..=lines[0].len() {
        for line in 1..lines.len() {
            if col == lines[0].len() && lines[1].len() <= col {
                break;
            }

            if col == 0 {
                let mut name = lines[line][col].clone();
                let finishing = if name.starts_with(':') {
                    name = name.strip_prefix(":").unwrap().to_string();
                    true
                } else {
                    false
                };

                let state = State::new(name, finishing);
                states.push(state);
                continue;
            }

            let names = lines[line][col].clone();
            for name in names.split(",") {
                if let Some(state) = states.iter().find(|state| state.name == name) {
                    states[line - 1].add_transaction(symbols[col - 1].clone(), state);
                }
            }
        }
    }

    ExitCode::SUCCESS
}

pub fn open_file(path: impl Into<String>) -> IOResult<Vec<String>> {
    let mut file = File::open(path.into())?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;
    let lines = buffer.lines();

    Ok(lines.into_iter().map(|str| str.to_string()).collect())
}
