use chrono::Local;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub date: String,
    pub name: String,
    pub text: String,
}

impl Note {
    pub fn new(name: String, text: String) -> Note {
        let now = Local::now();
        let formatted = now.format("%d/%m/%y").to_string();

        Note {
            date: formatted,
            name,
            text,
        }
    }
}
