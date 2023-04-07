mod file;
mod state;
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

    match command.as_str() {
        "progress" => {
            println!(
                "Progress: {} of {} ({}%)",
                state.progress,
                state.goal,
                (state.progress as f32 * 100f32) / state.goal as f32
            )
        }
        "add" => {
            if let Some(qty) = args.get(2) {
                if let Ok(qty) = qty.parse::<u32>() {
                    state.progress += qty;
                } else {
                    return die("expected a positive number but I don't know what is that");
                }
            } else {
                return die("expected a number but you didn't give me any");
            }
        }
        &_ => return die(&format!("unknown command: {command}")),
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
