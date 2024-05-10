use std::path::Path;
use tauri::api::dialog::blocking::FileDialogBuilder;
use tokio::fs::create_dir;
use crate::aggregator::aggregate;
use crate::analyzer::analyze;
use crate::grouper::{group_by_na_columns, group_by_peptides};
use crate::parser::parse;
use crate::serializer::{serialize, serialize_calculations};

pub async fn process_file(
    deps_dir: &Path,
    should_remove_na_calculations: bool,
    tolerance_multiplier: f64,
    input_file_path: String,
) -> Result<(), String> {
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

    let calculations = analyze(&deps_dir, &data_dir, &datasets).await?;
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