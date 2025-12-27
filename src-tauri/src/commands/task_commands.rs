use crate::domain::task::{Task, TaskPriority, TaskStatus};
use crate::storage::task_store::TaskStore;
use std::time::SystemTime;
use tauri::State;
use crate::error::TaskError;
use uuid::Uuid;

#[tauri::command]
pub fn add_task(
    store: State<TaskStore>,
    title: String,
    description: String,
    priority: TaskPriority,
    due_at: Option<SystemTime>,
) -> Result<Task, TaskError> {
    let task = Task::new(title, Some(description), priority, due_at)?;
    println!("Adding task: {:?}", task);
    store.add(task.clone());
    Ok(task)
}

#[tauri::command]
pub fn list_tasks(store: State<TaskStore>) -> Vec<Task> {
    store.list()
}

#[tauri::command]
pub fn delete_task(store: State<TaskStore>, id: String) -> bool {
    let uuid = id.parse().expect("Invalid UUID");
    store.delete(uuid)
}

pub fn update_task_status(store: State<TaskStore>, task_id: Uuid, status: TaskStatus) -> Result<Task, TaskError> {
    store.update_status(task_id, status)
}



#[tauri::command]
pub fn update_task(store: State<TaskStore>, id: Uuid, title: Option<String>, description: Option<Option<String>>) -> Result<Task, TaskError> {
    store.update_task(id, title, desciption)
}
