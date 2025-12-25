use crate::domain::task::{Task, TaskPriority};
use crate::storage::task_store::TaskStore;
use std::time::SystemTime;
use tauri::State;
#[tauri::command]
pub fn add_task(
    store: State<TaskStore>,
    title: String,
    description: String,
    priority: TaskPriority,
    due_at: Option<SystemTime>,
) {
    let task = Task::new(title, Some(description), priority, due_at);
    println!("Adding task: {:?}", task);
    store.add(task.clone());
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
