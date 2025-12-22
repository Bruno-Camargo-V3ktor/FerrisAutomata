use automata::state::State;
use automata::symbol::Symbol;
use colorz::Colorize;
use std::collections::HashMap;
use std::io::{Read, Result as IOResult};
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

    let mut states: HashMap<String, State> = HashMap::with_capacity(lines.len());

    ExitCode::SUCCESS
}

pub fn open_file(path: impl Into<String>) -> IOResult<Vec<String>> {
    let mut file = File::open(path.into())?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;
    let lines = buffer.lines();

    Ok(lines.into_iter().map(|str| str.to_string()).collect())
}
