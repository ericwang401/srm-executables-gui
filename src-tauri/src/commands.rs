use std::io::Cursor;
use std::path::Path;

use reqwest::Client;
use tauri::api::dialog::blocking::FileDialogBuilder;
use tokio::fs;
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
    input_file_path: String,
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

    let (days, mice, labels, peptides) = parse(input_file_path).await.unwrap();

    let groups = group_by_peptides(peptides);
    let groups = group_by_na_columns(groups);

    let datasets = serialize(
        &data_dir,
        days,
        mice,
        labels,
        groups,
    ).await.unwrap();

    let calculations = analyze(&dependencies_dir, &data_dir, &datasets).await.unwrap();
    let calculations = aggregate(&calculations).await.unwrap();

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
        serialize_calculations(&file_path, &calculations).unwrap();
    }


    Ok(())
}
