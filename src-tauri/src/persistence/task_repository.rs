use std::path::PathBuf;
use crate::domain::Task;
use std::fs;
use crate::error:TaskError;


#[derive(Debug)]
pub struct TaskRepository {
    file_path: PathBuf,
    }

impl TaskRepository {
    pub fn new(file_path: PathBuf) -> Self {
        Self {
            file_path
        }
    }

    pub fn save(&self, tasks: &Vec<Task>) -> Result<(), TaskError> {
        let json = serde_json::to_string_pretty(tasks).
            map_err( |_| TaskError::Persistence)?;

        fs::write(&self.file_path, json)
            .map_err( |_| TaskError::Persistence)?;

        Ok(())
    }

    pub fn load(&self) -> Result<Vec<Task>>, TaskError> {
        if !self.file_path.exists() {
            return Ok(Vec::new());
        }

        let data = fs::read_to_string(&self.file_path)
            .map_err( |_| TaskError::Persistence)?;

        let tasks = serde_json::from_str(&data)
            .map_err( |_| TaskError::Persistence)?;

        Ok(tasks)
    }
}
