use std::path::Path;
use anyhow::anyhow;
use tokio::fs::create_dir;
use crate::aggregator::aggregate;
use crate::analyzer::analyze_all;
use crate::commands::{ProgressCallback, ProgressUpdate};
use crate::grouper::{group_by_na_columns, group_by_peptides};
use crate::parser::{InputFile, parse};
use crate::serializer::{serialize, serialize_calculations};

pub async fn process_file(
    deps_dir: &Path,
    should_remove_na_calculations: bool,
    tolerance_multiplier: f64,
    input_file_path: InputFile,
    progress_callback: ProgressCallback,
) -> anyhow::Result<()> {
    dbg!("Run 2");
    progress_callback(ProgressUpdate::Set {
        iterations: 0,
        total_iterations: Some(100),
    });

    let temp_dir = tempfile::tempdir().map_err(|e| anyhow!(e.to_string()))?;
    let data_dir = temp_dir.path().join("data");

    create_dir(&data_dir).await.map_err(|e| anyhow!(e.to_string()))?;

    let input_file_path = Path::new(&input_file_path.path);

    let (
        days,
        mice,
        labels,
        peptides
    ) = parse(input_file_path).await?;

    let groups = group_by_na_columns(group_by_peptides(tolerance_multiplier, peptides));
    progress_callback(ProgressUpdate::Set {
        iterations: 0,
        total_iterations: Some(groups.len() * 2 + groups.len() / 10),
    });

    let datasets = serialize(
        should_remove_na_calculations,
        &data_dir,
        days,
        mice,
        labels,
        groups,
        &progress_callback,
    ).await.unwrap();

    let calculations = analyze_all(&deps_dir, &data_dir, &datasets, &progress_callback).await?;
    let calculations = aggregate(&calculations).await.map_err(|e| anyhow!(e.to_string()))?;

    let input_file_name = input_file_path
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .into_owned();

    let file_path = input_file_path.parent().unwrap().join(format!("{}.RateConst.csv", input_file_name));

    serialize_calculations(&file_path, &calculations)?;

    progress_callback(ProgressUpdate::Set {
        iterations: 100,
        total_iterations: Some(100),
    });

    temp_dir.close().map_err(|e| anyhow!(e.to_string()))?;

    Ok(())
}