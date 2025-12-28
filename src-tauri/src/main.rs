// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod domain;
mod error;
mod persistence;
mod storage;

use std::sync::Mutex;
use storage::task_store::TaskStore;

use crate::commands::task_commands::{add_task, delete_task, list_tasks, update_task_status};
use crate::persistence::task_repository::TaskRepository;

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data dir");

            std::fs::create_dir_all(&data_dir)?;

            let repo = TaskRepository::new(data_dir.join("tasks.json"));
            let store = TaskStore::new(repo)?;

            app.manage(store);
            Ok(())
        })
        // TODO: CHECK IF THIS CAN BE PASSED AS WHOLE ... LIKE SO
        .invoke_handler(tauri::generate_handler![
            add_task,
            delete_task,
            list_tasks,
            update_task_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
