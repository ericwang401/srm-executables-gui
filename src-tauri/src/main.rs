// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod parser;
mod grouper;
mod serializer;
mod analyzer;
mod aggregator;
mod processor;
mod lib;

#[tokio::main]
async fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![commands::process_data])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
