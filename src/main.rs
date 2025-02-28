mod task;
mod task_manager;

use task_manager::TaskManager;

fn main() {
    let mut manager = TaskManager::new();
    manager.add_task("This is the first task".to_string());

    manager.list_tasks();
}
