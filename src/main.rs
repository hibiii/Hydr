mod file;
mod state;
mod ui;
mod utili;

use std::{env, process::ExitCode};

#[cfg(not(debug_assertions))]
const FILE_PATH: &str = "~/.config/hydr.yaml";
#[cfg(debug_assertions)]
const FILE_PATH: &str = "./testing.yaml";

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage();
        return ExitCode::FAILURE;
    }
    // it should be established that args.len() >= 2
    let command = &args[1];
    if command == "version" {
        version();
        return ExitCode::SUCCESS;
    }
    let mut state = match file::read_or_default(FILE_PATH) {
        Err(file::Error::IoError) => {
            return die(&format!(
                "could not open state file, can you open \"{FILE_PATH}\"?"
            ))
        }
        Err(file::Error::SerdeError) => {
            return die(&format!(
                "could not parse state file, check \"{FILE_PATH}\""
            ))
        }
        Ok(state) => state,
    };

    if let Some(old_progress) = state.reset_if_new_day() {
        println!(
            "Reset progress (was {old_progress}, or {}%) because it is a new day.",
            (old_progress as f32 * 100f32) / state.goal as f32
        )
    }

    let mut state_modified = false;
    match command.as_str() {
        "progress" => ui::display_progress(&state),
        "add" => {
            let qty = ui::parse_millis(args.get(2));
            if let Err(reason) = qty {
                return die(reason);
            }
            state.drink_millis(qty.unwrap());
            ui::display_progress(&state);
            state_modified = true;
        }
        &_ => return die(&format!("unknown command: {command}")),
    }

    if !state_modified {
        return ExitCode::SUCCESS;
    }
    match file::save(&state, FILE_PATH) {
        Ok(_) => ExitCode::SUCCESS,
        Err(_) => return die("could not save state file, changes are gone"),
    }
}

fn usage() {
    println!(concat!(
        "Usage:\n",
        "hydr progress\n",
        "hydr add <millis>\n",
        "hydr version",
    ));
}

fn version() {
    println!(concat!("hydr ", env!("CARGO_PKG_VERSION"),));
}

fn die(reason: &str) -> ExitCode {
    eprintln!("Error: {}", reason);
    ExitCode::FAILURE
}
