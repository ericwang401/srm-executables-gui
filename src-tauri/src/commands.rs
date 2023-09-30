use std::io::Cursor;
use tauri::api::dialog::blocking::FileDialogBuilder;
use tokio::fs;

use reqwest::Client;

#[tauri::command]
pub async fn install_dependencies(app_handle: tauri::AppHandle) -> Result<(), String> {
    let dependencies_dir = app_handle.path_resolver().app_local_data_dir().unwrap().join("dependencies");

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
pub async fn select_data(app_handle: tauri::AppHandle, data_input_type: String) -> Result<Option<String>, String> {
    let data_dir = app_handle.path_resolver().app_local_data_dir().unwrap().join("data");

    match data_input_type.as_str() {
        "inputData" => {
            let file_path = FileDialogBuilder::new()
                .add_filter("Input Data File", &vec!["csv"])
                .pick_file();

            if let Some(file_path) = file_path {
                let file_name = file_path.file_name().unwrap();

                fs::copy(&file_path, format!("{}/Data.csv", data_dir.to_str().unwrap()))
                    .await
                    .map_err(|err| format!("Failed to save input: {err}"))?;

                return Ok(Some(file_name.to_string_lossy().into_owned()));
            }

            Ok(None)
        },
        "heavyWaterInputData" => {
            let file_path = FileDialogBuilder::new()
                .pick_file();

            if let Some(file_path) = file_path {
                let file_name = file_path.file_name().unwrap();

                fs::copy(&file_path, format!("{}/HeavyWater_Data.txt", data_dir.to_str().unwrap()))
                    .await
                    .map_err(|err| format!("Failed to save input: {err}"))?;

                return Ok(Some(file_name.to_string_lossy().into_owned()));
            }

            Ok(None)
        }
        _ => Err("Invalid data input type".into()),
    }
}

#[tauri::command]
pub async fn process_data(app_handle: tauri::AppHandle) -> Result<(), String> {
    let data_dir = app_handle.path_resolver().app_local_data_dir().unwrap().join("data");
    let dependencies_dir = app_handle.path_resolver().app_local_data_dir().unwrap().join("dependencies");

    let mut command = tokio::process::Command::new(format!(
        "{}/SRM_Rate.exe",
        dependencies_dir.to_str().unwrap()
    ));
    command.arg(format!(
        "{}/HeavyWater_Data.txt",
        data_dir.to_str().unwrap()
    ));
    command.arg(format!("{}/Data.csv", data_dir.to_str().unwrap()));

    let output = command
        .output()
        .await
        .map_err(|err| format!("Failed to run command: {err}"))?;

    if output.status.success() {
        let file_path = FileDialogBuilder::new()
            .set_file_name("Data.RateConst.csv")
            .add_filter("Output CSV File", &vec!["csv"])
            .save_file();

        if let Some(file_path) = file_path {
            fs::copy(format!("{}/Data.RateConst.csv", data_dir.to_str().unwrap()), file_path)
                .await
                .map_err(|err| format!("Failed to save output: {err}"))?;
        }

        Ok(())
    } else {
        Err("The command exited with non-zero status".to_string())
    }
}
