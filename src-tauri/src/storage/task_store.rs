use crate::domain::task::Task;
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Debug)]
pub struct TaskStore {
    tasks: Mutex<Vec<Task>>,
}

impl TaskStore {
    pub fn new() -> Self {
        Self {
            tasks: Mutex::new(Vec::new())
        }
    }

    pub fn add(&self, task: Task) {
        let mut tasks = self.tasks.lock().expect("TaskStore mutex poisoned");
        tasks.push(task);
    }

    pub fn list(&self) -> Vec<Task> {
        let tasks = self.tasks.lock().expect("TaskStore mutex poisoned");
        tasks.clone()
    }

    pub fn delete(&self, id: Uuid) -> bool {
        let mut tasks = self.tasks.lock().expect("TaskStore mutex poisoned");
        let tasks_old_len = tasks.len();
        tasks.retain( |t| t.id != id);
        tasks_old_len != tasks.len()
    }

}
