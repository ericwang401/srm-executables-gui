use csv::{Reader, ReaderBuilder, StringRecord, WriterBuilder};
use std::collections::HashMap;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::process::Command;
use uuid::Uuid;

pub fn isolate<C: AsRef<Vec<u8>>, H: AsRef<str>, P: AsRef<str>, N: AsRef<[u32]>>(
    data_dir: &Path,
    csv_contents: C,
    heavy_water_contents: H,
    peptide: P,
    na_samples: N,
) -> Result<(PathBuf, Uuid, PathBuf, Uuid), String> {
    let input_file_uuid = Uuid::new_v4();
    let input_file_path = data_dir.join(format!("{input_file_uuid}.csv"));

    let heavy_water_uuid = Uuid::new_v4();
    let heavy_water_file_path = data_dir.join(format!("{heavy_water_uuid}.txt"));

    let mut reader = ReaderBuilder::new()
        .has_headers(false) // this is to prevent the first row from being separated from the rest of the spreadsheet when we are trying to take the first 7 rows
        .from_reader(Cursor::new(csv_contents.as_ref()));
    let mut writer = WriterBuilder::new()
        .flexible(true) // NEEDED to write records with varying number of fields (https://docs.rs/csv/latest/csv/struct.WriterBuilder.html#method.flexible)
        .from_path(&input_file_path)
        .map_err(|err| format!("Failed to create CSV writer: {}", err))?;

    // Copy over the first 7 headers (rows)
    for record in reader.records().take(7) {
        let header = record.map_err(|err| format!("Failed to read record: {}", err))?;
        let header: Vec<String> = header
            .iter()
            .enumerate()
            .filter(|(index, _)| {
                // Filter out the N/A columns
                !na_samples.as_ref().contains(&(*index as u32))
            })
            .map(|(_, value)| value.to_string())
            .collect();

        writer
            .write_record(header.iter())
            .map_err(|err| format!("Failed to write header: {}", err))?;
    }

    // Pluck out the peptide of interest
    for result in reader.records() {
        let record = result.map_err(|err| format!("Failed to read record: {}", err))?;
        let current_peptide = record.get(1).unwrap(); // Assuming the peptide is at index 1
        if current_peptide == peptide.as_ref() {
            // Filter out the N/A samples
            let filtered_record: Vec<String> = record
                .iter()
                .enumerate()
                .filter(|(index, _)| !na_samples.as_ref().contains(&(*index as u32)))
                .map(|(_, value)| value.to_string())
                .collect();

            // Write the filtered record to the new CSV file
            writer
                .write_record(filtered_record)
                .map_err(|err| format!("Failed to write record: {}", err))?;
        }
    }

    writer
        .flush()
        .map_err(|err| format!("Failed to flush CSV writer: {}", err))?;

    Ok((input_file_path, input_file_uuid, heavy_water_file_path, heavy_water_uuid))
}

pub fn get_na_peptides<T: AsRef<Vec<u8>>>(
    csv_contents: T,
) -> Result<HashMap<String, Vec<u32>>, String> {
    let mut reader = Reader::from_reader(Cursor::new(csv_contents.as_ref()));
    /*
       peptides = {
           "peptide1": [0, 1], // 0 and 1 are the column index numbers. I chose column index numbers because sample names may sometimes conflict with each other and I don't want to deal with that mess.
       }
    */
    let mut peptides: HashMap<String, Vec<u32>> = HashMap::new();

    // find peptides (2nd column) that have NA samples (4th column and beyond) and list the column index numbers of the NA samples
    for result in reader.records() {
        let record = result.map_err(|err| format!("Failed to read record: {err}"))?;
        let peptide = record.get(1).unwrap();
        let mut na_samples: Vec<u32> = Vec::new();

        for (index, value) in record.iter().enumerate() {
            if index > 2 && value == "#N/A" {
                na_samples.push(index as u32);
            }
        }

        if na_samples.len() > 0 {
            peptides.insert(peptide.to_string(), na_samples);
        }
    }

    Ok(peptides)
}