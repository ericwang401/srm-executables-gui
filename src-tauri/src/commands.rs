use std::io::Cursor;
use std::path::Path;

use reqwest::Client;
use tauri::api::dialog::blocking::FileDialogBuilder;
use tokio::fs;
use tokio::fs::create_dir;
use crate::aggregator::aggregate;
use crate::analyzer::analyze;

use crate::grouper::{group_by_na_columns, group_by_peptides};
use crate::parser::parse;
use crate::serializer::{serialize, serialize_calculations};

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
    app_handle: tauri::AppHandle,
    should_remove_na_calculations: bool,
    tolerance_multiplier: f64,
    input_files: Vec<String>,
) -> Result<(), String> {}

#[tauri::command]
pub async fn old_process_data(
    app_handle: tauri::AppHandle,
    should_remove_na_calculations: bool,
    tolerance_multiplier: f64,
    input_file_path: String,
) -> Result<(), String> {
    let dependencies_dir = app_handle
        .path_resolver()
        .app_local_data_dir()
        .unwrap()
        .join("dependencies");

    let temp_dir = tempfile::tempdir().map_err(|e| e.to_string())?;
    let data_dir = temp_dir.path().join("data");
    create_dir(&data_dir).await.map_err(|e| e.to_string())?;


    let input_file_path = Path::new(&input_file_path);

    let (
        days,
        mice,
        labels,
        peptides
    ) = parse(input_file_path).await?;

    let groups = group_by_na_columns(group_by_peptides(tolerance_multiplier, peptides));

    let datasets = serialize(
        should_remove_na_calculations,
        &data_dir,
        days,
        mice,
        labels,
        groups,
    ).await.unwrap();

    let calculations = analyze(&dependencies_dir, &data_dir, &datasets).await?;
    let calculations = aggregate(&calculations).await.map_err(|e| e.to_string())?;

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
        serialize_calculations(&file_path, &calculations).map_err(|e| e.to_string())?;
    }

    temp_dir.close().map_err(|e| e.to_string())?;
    Ok(())
}
