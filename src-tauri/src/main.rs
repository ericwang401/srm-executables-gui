// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use app::commands;

#[tokio::main]
async fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![commands::install_dependencies, commands::process_data, commands::select_data])
    .setup(|app| {
      let data_dir = app.path_resolver().app_local_data_dir().unwrap().join("data");

      if data_dir.exists() {
        fs::remove_dir_all(&data_dir).unwrap();
      }

      fs::create_dir(&data_dir).unwrap();

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
