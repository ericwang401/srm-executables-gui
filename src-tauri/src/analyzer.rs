use std::path::{Path, PathBuf};
use tokio::process::Command;
use crate::serializer::Dataset;


pub async fn analyze(deps_dir: &Path, data_dir: &Path, datasets: &Vec<Dataset>) -> Result<Vec<(PathBuf, u64)>, String> {
    let mut results = vec![];

    for dataset in datasets {
        let result = analyze_single(deps_dir, data_dir, dataset).await?;
        results.push(result);
    }

    Ok(results)
}

async fn analyze_single(deps_dir: &Path, data_dir: &Path, dataset: &Dataset) -> Result<(PathBuf, u64), String> {
    let mut command = Command::new(deps_dir.join("SRM_Rate.exe")); // TODO: figure out lifetimes here

    command.arg(dataset.heavy_water.to_str().unwrap())
        .arg(dataset.spreadsheet.to_str().unwrap());

    let input_file_name = dataset.spreadsheet.file_stem().unwrap().to_str().unwrap();

    let output = command
        .output()
        .await
        .map_err(|err| format!("Command couldn't run: {err}"))?;

    if output.status.success() {
        Ok((data_dir.join(format!("{input_file_name}.RateConst.csv")), dataset.samples_removed))
    } else {
        Err(format!("The command didn't complete successfully: {}", String::from_utf8_lossy(&output.stdout)))
    }
}