use std::collections::HashMap;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::utili::ForeignString;

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub date: NaiveDate,
    pub progress: u32,
    pub goal: u32,
    pub portions: HashMap<String, Portion>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Portion {
    pub name: String,
    pub volume: u32,
}

impl State {
    pub fn reset_if_new_day(&mut self) -> Option<u32> {
        let today = chrono::Local::now().date_naive();
        if self.date == today {
            return None;
        }
        self.date = today;
        let old_progress = self.progress;
        self.progress = 0;
        return Some(old_progress);
    }

    pub fn add_portion(&mut self, portion: Portion) -> Option<Portion> {
        let id = portion.name.normalize_id();
        self.portions.insert(id, portion)
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            date: chrono::Local::now().date_naive(),
            progress: 0,
            goal: 0,
            portions: HashMap::new(),
        }
    }
}
