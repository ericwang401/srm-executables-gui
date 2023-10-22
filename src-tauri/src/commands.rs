use crate::processor;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;
use std::io::Cursor;
use std::path::Path;
use tauri::api::dialog::blocking::FileDialogBuilder;
use tokio::fs;

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

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    name: String,
    path: String,
}

#[tauri::command]
pub async fn select_data(data_input_type: String) -> Result<Option<File>, String> {
    match data_input_type.as_str() {
        "inputData" => {
            let file_path = FileDialogBuilder::new()
                .add_filter("Input Data File", &vec!["csv"])
                .pick_file();

            if let Some(file_path) = file_path {
                let file_name = file_path
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned();

                return Ok(Some(File {
                    name: file_name,
                    path: file_path.to_string_lossy().into_owned(),
                }));
            }

            Ok(None)
        }
        "heavyWaterInputData" => {
            let file_path = FileDialogBuilder::new()
                .add_filter("Heavy Water File", &vec!["txt"])
                .pick_file();

            if let Some(file_path) = file_path {
                let file_name = file_path
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned();

                return Ok(Some(File {
                    name: file_name,
                    path: file_path.to_string_lossy().into_owned(),
                }));
            }

            Ok(None)
        }
        _ => Err("Invalid data input type".into()),
    }
}

#[tauri::command]
pub async fn process_data(
    app_handle: tauri::AppHandle,
    should_remove_na_calculations: bool,
    input_file_path: String,
    heavy_water_file_path: String,
) -> Result<(), String> {
    let data_dir = app_handle
        .path_resolver()
        .app_local_data_dir()
        .unwrap()
        .join("data");
    let dependencies_dir = app_handle
        .path_resolver()
        .app_local_data_dir()
        .unwrap()
        .join("dependencies");
    let input_file_path = Path::new(&input_file_path);
    let heavy_water_file_path = Path::new(&heavy_water_file_path);

    let output_contents = processor::handle(
        should_remove_na_calculations,
        &data_dir,
        &dependencies_dir,
        &input_file_path,
        heavy_water_file_path,
    )
    .await?;

    let input_file_name = input_file_path
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .into_owned();

    let file_path = FileDialogBuilder::new()
            .set_file_name(&format!("{input_file_name}.RateConst.csv"))
            .add_filter("Output CSV File", &vec!["csv"])
            .save_file();

    if let Some(file_path) = file_path {
        // overwrite file if it already exists
        fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(&file_path)
            .await
            .map_err(|err| format!("Failed to write output file: {err}"))?
            .write(&output_contents)
            .await
            .map_err(|err| format!("Failed to write output file: {err}"))?;
    }

    Ok(())
}
