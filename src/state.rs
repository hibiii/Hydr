use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub date: NaiveDate,
    pub progress: u32,
    pub goal: u32,
    pub portions: Vec<Portion>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Portion {
    pub name: String,
    pub volume: u32,
}

impl Default for State {
    fn default() -> Self {
        State {
            date: chrono::Local::now().date_naive(),
            progress: 0,
            goal: 0,
            portions: Vec::new(),
        }
    }
}
