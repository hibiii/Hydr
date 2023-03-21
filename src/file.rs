use std::{fs, path::Path};

use crate::state::State;

pub fn read_or_default(file_path: &str) -> Result<State, Error> {
    if !Path::new(file_path).exists() {
        return Ok(State::default());
    }
    let contents = fs::read_to_string(file_path).map_err(|_| Error::IoError)?;
    let state = serde_yaml::from_str(&contents).map_err(|_| Error::SerdeError)?;
    Ok(state)
}

pub fn save(state: &State, file_path: &str) -> Result<(), Error> {
    let contents = serde_yaml::to_string(&state).unwrap();
    fs::write(file_path, contents).map_err(|_| Error::IoError)
}

pub enum Error {
    IoError,
    SerdeError,
}
