// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod domain;
mod storage;
mod commands;
mod error;
mod persistence;

use storage::task_store::TaskStore;
use commands::{add_task, list_tasks, delete_task, update_task_status};
use tauri::api::path::app_data_dir;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let data_dir = app_data_dir(app.config())
                .expect("Failed to get app data dir");

            std::fs::create_dir_all(&data_dir)?;

            let repo = TaskRepository::new(data_dir.join("tasks.json"));
            let store = TaskStore::new(repo)?;

            app.manage(store);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![...])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

