use colorz::Colorize;
use std::{env::args, process::ExitCode};

fn main() -> ExitCode {
    let args: Vec<String> = args().skip(1).collect();

    if args.len() < 2 {
        eprintln!("{}", "Invalid Arguments...".red());
        eprintln!("Usage: {}", "automata file.txt words...".yellow());
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
