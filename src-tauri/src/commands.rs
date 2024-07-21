use std::io::Cursor;
use std::path::Path;
use futures::future::join_all;
use reqwest::Client;
use tauri::api::dialog::blocking::FileDialogBuilder;
use tokio::fs;
use tokio::fs::create_dir;
use crate::aggregator::aggregate;
use crate::analyzer::analyze_all;

use crate::grouper::{group_by_na_columns, group_by_peptides};
use crate::parser::{EngineType, InputFile, parse};
use crate::processor::process_file;
use crate::serializer::{serialize, serialize_calculations};
use tokio::task::{JoinHandle, JoinSet};


#[tauri::command]
pub async fn install_dependencies(app_handle: tauri::AppHandle) -> Result<(), String> {
    let dependencies_dir = app_handle
        .path_resolver()
        .app_local_data_dir()
        .unwrap()
        .join("dependencies");

    if fs::try_exists(&dependencies_dir).await.unwrap() == true {
        fs::remove_dir_all(&dependencies_dir)
            .await
            .map_err(|err| format!("Failed to remove existing dependencies: {err}"))?;
    }

    let client = Client::new();
    let response = client
        .get("https://github.com/rgsadygov/SRM_executables/archive/refs/heads/main.zip")
        .send()
        .await
        .map_err(|err| format!("Failed to download dependencies: {err}"))?
        .bytes()
        .await
        .unwrap();

    zip_extract::extract(Cursor::new(response), &dependencies_dir, true)
        .map_err(|err| format!("Failed to extract dependencies: {err}"))?;

    Ok(())
}

#[tauri::command]
pub async fn process_data(
    app: tauri::AppHandle,
    engine_type: EngineType,
    should_remove_na_calculations: bool,
    tolerance_multiplier: f64,
    input_files: Vec<InputFile>,
) -> Result<(), String> {
    // TODO: https://tauri.app/v1/guides/features/events/
    dbg!("is this running?");

    let deps_path = match engine_type {
        EngineType::Single =>
            app.path_resolver()
                .resolve_resource("assets").unwrap().join("single-timepoint-engine"),
        EngineType::Multi => app.path_resolver()
            .resolve_resource("assets").unwrap().join("multi-timepoint-engine"),
    };

    let mut tasks: Vec<JoinHandle<anyhow::Result<()>>> = vec![];
    dbg!("wtf");


    for input_file in input_files {
        let deps_path = deps_path.clone();

        let task = tokio::spawn(async move {
            process_file(
                &deps_path,
                should_remove_na_calculations,
                tolerance_multiplier,
                input_file,
            ).await?;

            dbg!("hello?");

            Ok(())
        });

        tasks.push(task);
    }

    join_all(tasks).await;

    Ok(())
}