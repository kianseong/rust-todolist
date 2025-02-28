mod task;
mod task_manager;

use task_manager::TaskManager;

fn main() {
    let mut manager = TaskManager::new();
    manager.add_task("This is the first task".to_string());

    manager.list_tasks();
    manager.mark_task_complete(1);
    println!("--- After Marking ---");
    manager.list_tasks();

    let filename = "tasks.json";
    if let Err(err) = manager.save_to_file(filename) {
        eprintln!("Failed to save tasks: {}", err);
    }
}
