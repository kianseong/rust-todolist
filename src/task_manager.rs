use std::{fs, io::{self, Write}};

use crate::task::Task;
use serde_json;

pub struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new()
        }
    }

    pub fn add_task(&mut self, description: String) {
        let new_id = self.next_id();
        let task = Task::new(new_id, description);
        self.tasks.push(task);
    }

    // Find the highest id
    fn next_id(&self) -> u32 {
        self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1
    }

    pub fn mark_task_complete(&mut self, task_id: u32) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
            task.mark_complete();
        }
    }

    pub fn list_tasks(&self) {
        for t in &self.tasks {
            println!("{:?}", t);
        }
    }

    pub fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let json = serde_json::to_string_pretty(&self.tasks)?;
        let mut file = fs::File::create(filename)?;
        file.write_all(json.as_bytes())?;

        Ok(())
    }
}
