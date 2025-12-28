use crate::domain::task::{Task, TaskStatus};
use crate::error::TaskError;
use crate::persistence::task_repository::TaskRepository;
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Debug)]
pub struct TaskStore {
    tasks: Mutex<Vec<Task>>,
    repo: TaskRepository,
}

impl TaskStore {
    pub fn new(repo: TaskRepository) -> Result<Self, TaskError> {
        let tasks = repo.load()?;
        Ok(Self {
            tasks: Mutex::new(tasks),
            repo,
        })
    }

    pub fn add(&self, task: Task) -> Result<(), TaskError> {
        let mut tasks = self.tasks.lock().unwrap();
        tasks.push(task);
        self.repo.save(&tasks)
    }

    pub fn list(&self) -> Vec<Task> {
        let tasks = self.tasks.lock().expect("TaskStore mutex poisoned");
        tasks.clone()
    }

    pub fn delete(&self, id: Uuid) -> Result<bool, TaskError> {
        let mut tasks = self.tasks.lock().unwrap();
        let old_len = tasks.len();

        tasks.retain(|t| t.id != id);
        let deleted = old_len != tasks.len();

        if deleted {
            self.repo.save(&tasks)?;
        }

        Ok(deleted)
    }

    pub fn update_status(&self, id: Uuid, status: TaskStatus) -> Result<Task, TaskError> {
        let mut tasks = self.tasks.lock().unwrap();

        let task = {
            let task = tasks
                .iter_mut()
                .find(|t| t.id == id)
                .ok_or(TaskError::NotFound)?;

            task.update_status(status);
            task.clone()
        };
        self.repo.save(&tasks)?;

        Ok(task)
    }

    pub fn update(
        &self,
        id: Uuid,
        title: Option<String>,
        description: Option<Option<String>>,
    ) -> Result<Task, TaskError> {
        let mut tasks = self.tasks.lock().unwrap();

        let task = {
            let task = tasks
                .iter_mut()
                .find(|t| t.id == id)
                .ok_or(TaskError::NotFound)?;

            task.update(title, description)?;
            task.clone()
        };
        self.repo.save(&tasks)?;

        Ok(task)
    }
}
