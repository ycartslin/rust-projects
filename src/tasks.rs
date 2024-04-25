use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub description: String,
    pub completed: bool,
}

pub fn add_task(mut tasks: Vec<Task>, description: String) -> Vec<Task> {
    tasks.push(Task {
        description,
        completed: false,
    });
    tasks
}

pub fn complete_task(mut tasks: Vec<Task>, index: usize) -> Vec<Task> {
    if let Some(task) = tasks.get_mut(index) {
        task.completed = true;
    }
    tasks
}

pub fn save_tasks(tasks: &Vec<Task>, path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let tasks_json = serde_json::to_string(&tasks)?;
    fs::write(path, tasks_json)?;
    Ok(())
}

pub fn load_tasks(path: PathBuf) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let tasks_file = fs::read_to_string(path)?;
    let tasks: Vec<Task> = serde_json::from_str(&tasks_file)?;
    Ok(tasks)
}
