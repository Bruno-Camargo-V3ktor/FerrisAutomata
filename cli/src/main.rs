use colorz::Colorize;
use std::io::{Read, Result as IOResult};
use std::{env::args, fs::File, process::ExitCode};

fn main() -> ExitCode {
    let args: Vec<String> = args().skip(1).collect();

    if args.len() < 2 {
        eprintln!("{}", "Invalid Arguments...".red());
        eprintln!("Usage: {}", "automata file.txt words...".yellow());
        return ExitCode::FAILURE;
    }

    let lines = match open_file(&args[0]) {
        Ok(lines) => lines,
        Err(err) => {
            eprintln!("{} {:?}", "File Invalid...\n Error:".red(), err);
            return ExitCode::FAILURE;
        }
    };

    ExitCode::SUCCESS
}

pub fn open_file(path: impl Into<String>) -> IOResult<Vec<String>> {
    let mut file = File::open(path.into())?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;
    let lines = buffer.lines();

    Ok(lines.into_iter().map(|str| str.to_string()).collect())
}
