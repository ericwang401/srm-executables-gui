use crate::processor::{
    aggregator::aggregate,
    executor::execute,
    helpers::{copy_input_files, prepare_data_folder},
    peptide_isolator::{get_na_peptides, isolate},
};
use csv::{Reader, ReaderBuilder, StringRecord, WriterBuilder};
use std::collections::HashMap;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::process::Command;
use uuid::Uuid;

mod aggregator;
mod executor;
mod helpers;
mod peptide_isolator;

pub async fn handle(
    should_remove_na_calculations: bool,
    data_dir: &Path,
    dependencies_dir: &Path,
    input_file_path: &Path,
    heavy_water_file_path: &Path,
) -> Result<(), String> {
    prepare_data_folder(data_dir).await?;

    let (input_file_path, heavy_water_file_path, input_file_uuid, heavy_water_file_uuid) =
        copy_input_files(data_dir, input_file_path, heavy_water_file_path).await?;
    let input_file_contents = fs::read(&input_file_path).await.unwrap();
    let heavy_water_contents = fs::read_to_string(&heavy_water_file_path).await.unwrap();

    let master_output_file = execute(
        dependencies_dir,
        data_dir,
        &input_file_path,
        &heavy_water_file_path,
        Some((&input_file_uuid, &heavy_water_file_uuid)),
    )
    .await?;
    let mut master_output_contents = fs::read(&master_output_file).await.unwrap();

    if (should_remove_na_calculations) {
        let peptides_with_na_samples = get_na_peptides(&input_file_contents)?;

        let mut peptide_files: Vec<(PathBuf, Uuid, PathBuf, Uuid)> = Vec::new();
        for (peptide, na_samples) in peptides_with_na_samples.iter() {
            peptide_files.push(isolate(
                &data_dir,
                &input_file_contents,
                &heavy_water_contents,
                peptide,
                na_samples,
            )?);
        }

        let mut peptide_output_files: Vec<PathBuf> = Vec::new();
        for (input_file_path, input_file_uuid, heavy_water_path, heavy_water_uuid) in peptide_files.iter() {
            peptide_output_files.push(
                execute(
                    dependencies_dir,
                    data_dir,
                    input_file_path,
                    heavy_water_path,
                    Some((input_file_uuid, heavy_water_uuid)),
                )
                .await?,
            );
        }

        for peptide_output_file in peptide_output_files.iter() {
            aggregate(&mut master_output_contents, peptide_output_file)?;
        }
    }

    Ok(())
}
