use crate::domain::task::{Task, TaskPriority, TaskStatus};
use crate::error::TaskError;
use crate::storage::task_store::TaskStore;
use std::sync::Mutex;
use std::time::SystemTime;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub fn add_task(
    store: State<TaskStore>,
    title: String,
    description: Option<String>,
    priority: TaskPriority,
    due_at: Option<SystemTime>,
) -> Result<Task, TaskError> {
    let task = Task::new(title, description, priority, due_at)?;
    store.add(task.clone())?;
    Ok(task)
}

#[tauri::command]
pub fn list_tasks(store: State<Mutex<TaskStore>>) -> Result<Vec<Task>, TaskError> {
    let store = store.lock().unwrap();
    Ok(store.list())
}

#[tauri::command]
pub fn delete_task(store: State<Mutex<TaskStore>>, id: String) -> Result<bool, TaskError> {
    let uuid = id.parse().map_err(|_| TaskError::InvalidUuid)?;
    let store = store.lock().unwrap();
    store.delete(uuid)
}

#[tauri::command]
pub fn update_task_status(
    store: State<Mutex<TaskStore>>,
    task_id: String,
    status: TaskStatus,
) -> Result<Task, TaskError> {
    let uuid = task_id.parse().map_err(|_| TaskError::InvalidUuid)?;
    let store = store.lock().unwrap();
    store.update_status(uuid, status)
}

#[tauri::command]
pub fn update_task(
    store: State<Mutex<TaskStore>>,
    id: String,
    title: Option<String>,
    description: Option<Option<String>>,
) -> Result<Task, TaskError> {
    let uuid = id.parse().map_err(|_| TaskError::InvalidUuid)?;
    let store = store.lock().unwrap();
    store.update(uuid, title, description)
}
