use std::collections::HashMap;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::process::Command;
use uuid::Uuid;
use csv::Reader;

pub async fn handle(
    should_remove_na_calculations: bool,
    data_dir: &Path,
    dependencies_dir: &Path,
    input_file_path: &Path,
    heavy_water_file_path: &Path,
) -> Result<(), String> {
    prepare_data_folder(data_dir).await?;

    let (input_file_path, _, _) =
        copy_input_files(data_dir, input_file_path, heavy_water_file_path).await?;
    let input_file_contents = fs::read(&input_file_path)
        .await
        .unwrap();
    let mut rdr = Reader::from_reader(Cursor::new(input_file_contents));


    Ok(())
}

pub async fn process_data(
    dependencies_dir: &Path,
    data_dir: &Path,
    input_file_path: &Path,
    heavy_water_file_path: &Path,
) -> Result<PathBuf, String> {
    let (input_file_path, heavy_water_file_path, input_file_uuid) =
        copy_input_files(data_dir, input_file_path, heavy_water_file_path).await?;

    let mut command = Command::new(dependencies_dir.join("SRM_Rate.exe"));
    command
        .arg(heavy_water_file_path.to_str().unwrap())
        .arg(input_file_path.to_str().unwrap())
        // additional context here: https://stackoverflow.com/questions/60750113/how-do-i-hide-the-console-window-for-a-process-started-with-stdprocesscomman
        // CREATE_NO_WINDOW flag. See: https://learn.microsoft.com/en-us/windows/win32/procthread/process-creation-flags#CREATE_NO_WINDOW
        // CREATE_NO_WINDOW comes at a disadvantage of not being able to read the output of the process. DETACHED_PROCESS can be used instead if output is needed.
        .creation_flags(0x08000000);

    let output = command
        .output()
        .await
        .map_err(|err| format!("Command couldn't run: {err}"))?;

    if output.status.success() {
        Ok(data_dir.join(format!("{input_file_uuid}.RateConst.csv")))
    } else {
        Err("The command didn't complete successfully".to_string())
    }
}

async fn copy_input_files(
    data_dir: &Path,
    input_file_path: &Path,
    heavy_water_file_path: &Path,
) -> Result<(PathBuf, PathBuf, Uuid), String> {
    let input_file_uuid = Uuid::new_v4();
    let new_input_file_path = data_dir.join(format!("{input_file_uuid}.csv"));
    let new_heavy_water_file_path = data_dir.join(format!("{}.txt", Uuid::new_v4()));

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
    ))
}

async fn prepare_data_folder(data_dir: &Path) -> Result<(), String> {
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

async fn get_peptides_with_na_samples() -> Result<HashMap<String, Vec<u32>>, String> {
    /*
       peptides = {
           "peptide1": [0, 1], // 0 and 1 are the column index numbers. I chose column index numbers because sample names may sometimes conflict with each other and I don't want to deal with that mess.
       }
    */
    let mut peptides: HashMap<String, Vec<u32>> = HashMap::new();



    Ok(peptides)
}
