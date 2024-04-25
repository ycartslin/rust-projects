mod tasks;
use clap::{App, Arg, SubCommand};
use tasks::{add_task, complete_task, load_tasks, save_tasks};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Rust Task Manager")
        .version("1.0")
        .author("Your Name")
        .about("Manages a task list")
        .subcommand(
            SubCommand::with_name("add")
                .about("Adds a task")
                .arg(Arg::with_name("description").help("Task description").required(true)),
        )
        .subcommand(
            SubCommand::with_name("done")
                .about("Marks a task as done")
                .arg(Arg::with_name("index").help("Task index").required(true).takes_value(true)),
        )
        .subcommand(SubCommand::with_name("list").about("Lists all tasks"))
        .get_matches();

    let mut task_path = PathBuf::from(".tasks.json");
    let mut tasks = load_tasks(task_path.clone()).unwrap_or_else(|_| vec![]);

    match matches.subcommand() {
        ("add", Some(add_matches)) => {
            tasks = add_task(tasks, add_matches.value_of("description").unwrap().to_string());
            save_tasks(&tasks, task_path)?;
        }
        ("done", Some(done_matches)) => {
            let index: usize = done_matches.value_of("index").unwrap().parse().expect("Invalid index");
            tasks = complete_task(tasks, index);
            save_tasks(&tasks, task_path)?;
        }
        ("list", Some(_)) => {
            for (index, task) in tasks.iter().enumerate() {
                println!("{}: {} [{}]", index, task.description, if task.completed { "done" } else { "pending" });
            }
        }
        _ => unreachable!(), // clap will handle if no subcommand is passed
    }

    Ok(())
}
