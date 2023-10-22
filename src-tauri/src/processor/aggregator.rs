use csv::{Reader, ReaderBuilder, StringRecord, WriterBuilder};
use std::collections::HashMap;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::process::Command;
use uuid::Uuid;

// stitches the isolated peptide results back into the master output file
pub fn aggregate(
    mut master_output_contents: &mut Vec<u8>,
    peptide_output_file: &Path,
) -> Result<(), String> {
    let mut master_reader = ReaderBuilder::new()
        .has_headers(false) // this is to prevent the first row from being separated from the rest of the spreadsheet when we are trying to take the first 7 rows
        .from_reader(Cursor::new(master_output_contents.clone()));
    let mut master_writer = WriterBuilder::new()
        .flexible(true) // NEEDED to write records with varying number of fields (https://docs.rs/csv/latest/csv/struct.WriterBuilder.html#method.flexible)
        .from_writer(&mut master_output_contents);
    let mut peptide_reader = ReaderBuilder::new()
        .has_headers(false)
        .from_path(peptide_output_file)
        .map_err(|err| format!("Failed to create CSV reader: {}", err))?;

    let master_header: StringRecord = master_reader
        .records()
        .nth(6)
        .unwrap()
        .map_err(|err| format!("Failed to read record: {}", err))?;

    let peptide_header: StringRecord = peptide_reader
        .records()
        .nth(6)
        .unwrap()
        .map_err(|err| format!("Failed to read record: {}", err))?;

    for result in peptide_reader.records() {
        dbg!(result);
    }

    Ok(())
}