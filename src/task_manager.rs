use crate::task::Task;

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

    pub fn list_tasks(&self) {
        for t in &self.tasks {
            println!("{:?}", t);
        }

    }
}
