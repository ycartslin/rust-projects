use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub description: String,
    pub completed: bool,
}

pub async fn save_tasks(tasks: &[Task], path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let tasks_json = serde_json::to_string(tasks)?;
    fs::write(path, tasks_json).await?;
    Ok(())
}

pub async fn load_tasks(path: PathBuf) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let tasks_file = fs::read_to_string(path).await?;
    let tasks: Vec<Task> = serde_json::from_str(&tasks_file)?;
    Ok(tasks)
}
