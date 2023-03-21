mod file;
mod state;

use std::process::ExitCode;

use crate::state::{Portion, State};

#[cfg(not(debug_assertions))]
const FILE_PATH: &str = "~/.config/hydr.yaml";
#[cfg(debug_assertions)]
const FILE_PATH: &str = "./testing.yaml";

fn main() -> ExitCode {
    let state = match file::read_or_default(FILE_PATH) {
        Err(file::Error::IoError) => {
            eprintln!("Error: could not open state file, can you open \"{FILE_PATH}\"?");
            return ExitCode::from(2);
        }
        Err(file::Error::SerdeError) => {
            eprintln!("Error: could not parse state file, check \"{FILE_PATH}\"");
            return ExitCode::from(3);
        }
        Ok(state) => state,
    };

    println!("{state:?}");

    let state = {
        let mut o = State::default();
        o.goal = 2500;
        o.progress = 420;
        o.portions.push(Portion {
            name: "Glass".to_string(),
            volume: 200,
        });
        o
    };

    match file::save(&state, FILE_PATH) {
        Ok(_) => ExitCode::SUCCESS,
        Err(_) => {
            eprintln!("Error: could not save state file, changes are gone");
            ExitCode::from(4)
        }
    }
}
