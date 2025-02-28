use serde::{Serialize};

#[derive(Debug, Serialize)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub completed: bool,
}

impl Task {
    pub fn new(id: u32, description: String) -> Self {
        Self {
            id,
            description,
            completed: false,
        }

    }

    pub fn mark_complete(&mut self) {
        self.completed = true;
    }
}
