use crate::commands::{ProgressCallback, ProgressUpdate};
use crate::serializer::Dataset;
use anyhow::anyhow;
use std::path::{Path, PathBuf};
use tokio::fs::remove_file;
use tokio::process::Command;

pub async fn analyze_all(
    deps_dir: &Path,
    data_dir: &Path,
    datasets: &Vec<Dataset>,
    progress_callback: &ProgressCallback,
) -> anyhow::Result<Vec<(PathBuf, u64)>> {
    let mut results = vec![];

    for dataset in datasets {
        let result = analyze(deps_dir, data_dir, dataset).await?;
        results.push(result);
        progress_callback(ProgressUpdate::Increment { iterations: 1 });
    }

    Ok(results)
}

async fn analyze(
    deps_dir: &Path,
    data_dir: &Path,
    dataset: &Dataset,
) -> anyhow::Result<(PathBuf, u64)> {
    let mut command = Command::new(deps_dir.join("SRM_Rate.exe")); // TODO: figure out lifetimes here

    command
        .arg(dataset.heavy_water.to_str().unwrap())
        .arg(dataset.spreadsheet.to_str().unwrap());

    let input_file_name = dataset.spreadsheet.file_stem().unwrap().to_str().unwrap();

    let output = command
        .output()
        .await
        .map_err(|err| anyhow!(format!("Command couldn't run: {err}")))?;

    remove_file(&dataset.heavy_water)
        .await
        .map_err(|err| anyhow!(format!("Couldn't delete heavy water file: {err}")))?;
    remove_file(&dataset.spreadsheet)
        .await
        .map_err(|err| anyhow!(format!("Couldn't delete spreadsheet file: {err}")))?;

    if output.status.success() {
        Ok((
            data_dir.join(format!("{input_file_name}.RateConst.csv")),
            dataset.samples_removed,
        ))
    } else {
        Err(anyhow!(format!(
            "The command didn't complete successfully: {}",
            String::from_utf8_lossy(&output.stderr)
        )))
    }
}
