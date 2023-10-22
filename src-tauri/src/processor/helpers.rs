


use std::path::{Path, PathBuf};
use tokio::fs;

use uuid::Uuid;

pub async fn copy_input_files(
    data_dir: &Path,
    input_file_path: &Path,
    heavy_water_file_path: &Path,
) -> Result<(PathBuf, PathBuf, Uuid, Uuid), String> {
    let input_file_uuid = Uuid::new_v4();
    let heavy_water_file_uuid = Uuid::new_v4();
    let new_input_file_path = data_dir.join(format!("{input_file_uuid}.csv"));
    let new_heavy_water_file_path = data_dir.join(format!("{heavy_water_file_uuid}.txt"));

    fs::copy(input_file_path, &new_input_file_path)
        .await
        .map_err(|err| format!("Failed to save input file: {err}"))?;

    fs::copy(heavy_water_file_path, &new_heavy_water_file_path)
        .await
        .map_err(|err| format!("Failed to save heavy water file: {err}"))?;

    Ok((
        new_input_file_path,
        new_heavy_water_file_path,
        input_file_uuid,
        heavy_water_file_uuid,
    ))
}

pub async fn clear_data_folder(data_dir: &Path) -> Result<(), String> {
    if data_dir.exists() {
        fs::remove_dir_all(&data_dir).await.map_err(|err| {
            format!(
                "Failed to remove existing data folder: {err}",
                err = err.to_string()
            )
        })?;
    }

    fs::create_dir(&data_dir)
        .await
        .map_err(|err| format!("Failed to create data folder: {err}", err = err.to_string()))?;

    Ok(())
}