use std::io::Cursor;
use tauri::api::dialog::FileDialogBuilder;
use tokio::fs;

use reqwest::Client;

#[tauri::command]
pub async fn install_dependencies(app_handle: tauri::AppHandle) -> Result<(), String> {
    let mut dependencies_dir = app_handle.path_resolver().app_local_data_dir().unwrap();
    dependencies_dir.push("dependencies");

    if fs::try_exists(&dependencies_dir).await.unwrap() == true {
        fs::remove_dir_all(&dependencies_dir)
            .await
            .map_err(|_| "Failed to remove existing dependencies")?;
    }

    let client = Client::new();
    let response = client
        .get("https://github.com/rgsadygov/SRM_executables/archive/refs/heads/main.zip")
        .send()
        .await
        .map_err(|_| "Failed to download dependencies")?
        .bytes()
        .await
        .unwrap();

    zip_extract::extract(Cursor::new(response), &dependencies_dir, true)
        .map_err(|_| "Failed to extract dependencies")?;

    Ok(())
}

#[tauri::command]
pub async fn process_data(app_handle: tauri::AppHandle) -> Result<(), String> {
    let mut data_dir = app_handle.path_resolver().app_local_data_dir().unwrap();
    data_dir.push("data");
    let mut dependencies_dir = app_handle.path_resolver().app_local_data_dir().unwrap();
    dependencies_dir.push("dependencies");

    let mut command = tokio::process::Command::new(format!(
        "{}/SRM_Rate.exe",
        dependencies_dir.to_str().unwrap()
    ));
    command.arg(format!(
        "{}/HeavyWater_Data.txt",
        data_dir.to_str().unwrap()
    ));
    command.arg(format!("{}/Data.csv", data_dir.to_str().unwrap()));

    let output = command.output().await.unwrap();

    if output.status.success() {
        FileDialogBuilder::new()
            .set_file_name("Data.RateConst.csv")
            .add_filter("Output CSV File", &vec!["csv"])
            .save_file(move |file_path| {
                std::fs::copy(
                    format!("{}/Data.RateConst.csv", data_dir.to_str().unwrap()),
                    file_path.unwrap(),
                ).unwrap();
            });

        Ok(())
    } else {
        Err("SRM_executables failed".to_string())
    }
}
