use csv::{Reader, WriterBuilder};
use std::collections::HashMap;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::process::Command;
use uuid::Uuid;

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
    let input_file_contents = fs::read(&input_file_path).await.unwrap();

    let peptides_with_na_samples = get_peptides_with_na_samples(&input_file_contents)?;

    if let Some((peptide, na_samples)) = peptides_with_na_samples.iter().next() {
        isolate_peptide_into_new_csv(&data_dir, &input_file_contents, peptide, na_samples)?;
    }

    dbg!(peptides_with_na_samples);

    Ok(())
}

async fn process_data(
    dependencies_dir: &Path,
    data_dir: &Path,
    input_file_path: &Path,
    heavy_water_file_path: &Path,
    uuid: Option<Uuid>,
) -> Result<PathBuf, String> {
    let (input_file_path, heavy_water_file_path, input_file_uuid) = match uuid {
        Some(id) => (
            data_dir.join(format!("{}.input", id)),
            data_dir.join(format!("{}.heavy", id)),
            id,
        ),
        None => copy_input_files(data_dir, input_file_path, heavy_water_file_path).await?,
    };

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

// make csv_contents a vector that contains unsigned integers of any size
fn get_peptides_with_na_samples<T: AsRef<Vec<u8>>>(
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

// isolate a troublesome peptide into its own file without the N/A sample
fn isolate_peptide_into_new_csv<C: AsRef<Vec<u8>>, P: AsRef<str>, N: AsRef<[u32]>>(
    data_dir: &Path,
    csv_contents: C,
    peptide: P,
    na_samples: N,
) -> Result<PathBuf, String> {
    let mut reader = Reader::from_reader(Cursor::new(csv_contents.as_ref()));
    let output_file_name = data_dir.join(format!("{uuid}.csv", uuid = Uuid::new_v4()));
    let mut writer = WriterBuilder::new()
        .flexible(true) // NEEDED to write records with different number of fields (https://docs.rs/csv/latest/csv/struct.WriterBuilder.html#method.flexible)
        .from_path(&output_file_name)
        .map_err(|err| format!("Failed to create CSV writer: {}", err))?;

    // Store the first 7 rows from the original csv in a vector
    for record in reader.records().take(7) {
        let record = record.map_err(|err| format!("Failed to read record: {}", err))?;
        writer
            .write_record(record.iter())
            .map_err(|err| format!("Failed to write record: {}", err))?;
    }

    // Iterate over the records and filter based on the provided peptide and NA samples
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
    Ok(output_file_name) // Return the path to the new CSV file
}
