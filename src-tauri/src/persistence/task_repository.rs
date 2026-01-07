use crate::domain::task::Task;
use crate::error::TaskError;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug)]
pub struct TaskRepository {
    file_path: PathBuf,
}

impl TaskRepository {
    pub fn new(file_path: PathBuf) -> Self {
        Self { file_path }
    }

    pub fn save(&self, tasks: &Vec<Task>) -> Result<(), TaskError> {
        let json = serde_json::to_string_pretty(tasks).map_err(|_| TaskError::Persistence)?;

        let tmp_path = self.file_path.with_extension("tmp");

        let mut file = File::create(&tmp_path).map_err(|_| TaskError::Persistence)?;

        file.write_all(json.as_bytes())
            .map_err(|_| TaskError::Persistence)?;

        file.sync_all().map_err(|_| TaskError::Persistence)?;

        fs::rename(tmp_path, &self.file_path).map_err(|_| TaskError::Persistence)?;

        Ok(())
    }

    pub fn load(&self) -> Result<Vec<Task>, TaskError> {
        let tmp = self.file_path.with_extension("tmp");

        if tmp.exists() {
            fs::remove_file(tmp).ok();
        }

        if !self.file_path.exists() {
            return Ok(Vec::new());
        }

        let data = fs::read_to_string(&self.file_path).map_err(|_| TaskError::Persistence)?;

        let tasks = serde_json::from_str(&data).map_err(|_| TaskError::Persistence)?;

        Ok(tasks)
    }
}
