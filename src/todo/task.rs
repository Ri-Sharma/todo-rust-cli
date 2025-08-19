use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub note: String,
    pub completed: bool,
}

impl Task {
    pub fn new(note: &str) -> Self {
        Task {
            note: note.to_string(), 
            completed: false
        }
    }
}