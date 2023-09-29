// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::commands;

#[tokio::main]
async fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![commands::install_dependencies, commands::process_data])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
