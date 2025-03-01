mod task;
mod task_manager;

use task_manager::TaskManager;

use std::{io, process};

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

    loop {
        let mut input = String::new();
        println!("What do you want to do?\n1. Add a new task\n2. List tasks\n3. Exit\n");

        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<u32>() {
            Ok(number) => {
                match number {
                    1 => {
                        println!("Adding a task.");
                        println!("Please enter the description.");

                        let mut task_desc = String::new();
                        io::stdin().read_line(&mut task_desc).unwrap();

                        manager.add_task(task_desc.trim().to_string());
                        continue;
                    },
                    2 => {
                        manager.list_tasks();
                        continue;
                    },
                    3 => {
                        println!("Exiting...");
                        process::exit(0);
                    },
                    _ => eprintln!("Invalid choice. Please choose again!"),
                }
                break;
            },
            Err(_) => println!("Invalid choice. Please choose an option!"),
        }
    }

    if let Err(err) = manager.save_to_file(filename) {
        eprintln!("Failed to save tasks: {}", err);
    }
}
