mod task;
mod task_manager;

use task_manager::TaskManager;

fn main() {
    let mut manager = TaskManager::new();

    let filename = "tasks.json";

    manager = match TaskManager::load_from_file(filename) {
        Ok(loaded_manager) => loaded_manager,
        Err(err) => {
            eprintln!("Failed to load tasks: {}", err);
            manager
        }
    };

    manager.list_tasks();
    if let Err(err) = manager.save_to_file(filename) {
        eprintln!("Failed to save tasks: {}", err);
    }
}
