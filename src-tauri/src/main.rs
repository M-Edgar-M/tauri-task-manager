// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod domain;
mod storage;
mod commands;

use storage::task_store::TaskStore;
use commands::{add_task, list_tasks, delete_task};

fn main() {
    tauri::Builder::default()
        .manage(TaskStore::new())
        .invoke_handler(tauri::generate_handler![
            add_task,
            list_tasks,
            delete_task
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
