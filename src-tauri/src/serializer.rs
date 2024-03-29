use std::collections::HashSet;
use std::path::{Path, PathBuf};

use csv::Writer;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use crate::aggregator::Calculation;

use crate::grouper::NAGroup;
use crate::parser::{Day, Label, Mouse, Peptide};

pub struct Dataset {
    pub spreadsheet: PathBuf,
    pub heavy_water: PathBuf,
    pub samples_removed: u64,
}


pub async fn serialize(
    should_remove_na_calculations: bool,
    path: &Path,
    days: Vec<Day>,
    mice: Vec<Mouse>,
    labels: Vec<Label>,
    groups: Vec<NAGroup>,
) -> Result<Vec<Dataset>, Box<dyn std::error::Error>> {
    let mut datasets = vec![];

    for group in groups {
        let (
            filtered_days,
            filtered_mice,
            filtered_labels,
            filtered_peptides,
            columns_removed
        ) = if should_remove_na_calculations {
            prepare_peptides(&days, &mice, &labels, group)
        } else {
            (
                days.clone(),
                mice.clone(),
                labels.clone(),
                group.peptides.clone(),
                0
            )
        };

        let peptides = serialize_peptides(path, &filtered_days, &filtered_mice, &filtered_labels, filtered_peptides)?;

        let heavy_water = serialize_heavy_water_file(path, &filtered_days, &filtered_labels).await?;

        datasets.push(Dataset {
            spreadsheet: peptides,
            heavy_water,
            samples_removed: columns_removed,
        });
    }

    Ok(datasets)
}

async fn serialize_heavy_water_file(path: &Path, days: &Vec<Day>, labels: &Vec<Label>) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Ensure the input vectors are of the same length
    if days.len() != labels.len() {
        return Err("Days and labels vectors must have the same length.".into());
    }

    // Generate a unique file name
    let file_name = format!("heavy_water_{}.txt", uuid::Uuid::new_v4());
    let file_path = path.join(file_name);

    // Create and open the file
    let mut file = File::create(&file_path).await?;

    // Write the header
    file.write_all(b"Experiment, Labeling\n").await?;

    // Write each day and label to the file
    for (day, label) in days.iter().zip(labels.iter()) {
        // Check if the label is numeric and replace non-numeric labels with "0"
        let numeric_label = if label.parse::<f64>().is_ok() {
            label
        } else if label.parse::<i64>().is_ok() {
            label
        } else {
            "0"
        };
        file.write_all(&format!("{}, {}\n", day, numeric_label).into_bytes()).await?;
    }

    Ok(file_path)
}

fn serialize_peptides(
    path: &Path,
    days: &Vec<Day>,
    mice: &Vec<Mouse>,
    labels: &Vec<Label>,
    peptides: Vec<Peptide>) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let file_path = path.join(format!("peptides_{}.csv", uuid::Uuid::new_v4()));
    let mut wtr = Writer::from_path(file_path.clone())?;

    let days_str = [
        vec!["Day".to_string(), "".to_string(), "".to_string()],
        days.iter().map(|day| day.to_string()).collect()
    ].concat();

    let mice_str = [
        vec!["Mouse".to_string(), "".to_string(), "".to_string()],
        mice.iter().map(|mouse| mouse.to_string()).collect()
    ].concat();

    let labels_str = [
        vec!["Body water enrichment".to_string(), "".to_string(), "".to_string()],
        labels.iter().map(|label| label.to_string()).collect()
    ].concat();

    let headers_str = [
        vec!["Protein".to_string(), "Peptide".to_string(), "Product Mz".to_string()],
        days.iter().enumerate().map(|(i, _)| i.to_string()).collect()
    ].concat();

    wtr.write_record(&days_str)?;
    wtr.write_record(vec![""; headers_str.len()])?;
    wtr.write_record(&mice_str)?;
    wtr.write_record(vec![""; headers_str.len()])?;
    wtr.write_record(&labels_str)?;
    wtr.write_record(vec![""; headers_str.len()])?;
    wtr.write_record(&headers_str)?;

    for peptide in peptides {
        let intensities = peptide
            .intensities
            .iter()
            .map(|i| {
                match i {
                    None => "#N/A".to_string(),
                    Some(i) => i.to_string(),
                }
            }).collect();

        let record = [
            vec![peptide.protein, peptide.name, peptide.mass_charge_ratio.to_string()],
            intensities
        ].concat();
        wtr.write_record(&record)?;
    }

    wtr.flush()?;

    Ok(file_path)
}

fn prepare_peptides(
    days: &Vec<Day>,
    mice: &Vec<Mouse>,
    labels: &Vec<Label>,
    group: NAGroup,
) -> (Vec<Day>, Vec<Mouse>, Vec<Label>, Vec<Peptide>, u64) {
    // Determine which columns are NA across all groups in the NAGroup
    let mut na_columns = HashSet::new();
    for (i, &is_na) in group.na_columns.iter().enumerate() {
        if is_na {
            na_columns.insert(i);
        }
    }

    let original_columns_count = days.len().max(mice.len()).max(labels.len());

    // Filter out NA columns from days, mice, labels, and peptide intensities
    let filtered_days: Vec<Day> = days.into_iter().enumerate()
        .filter(|(i, _)| !na_columns.contains(i))
        .map(|(_, day)| day.clone())
        .collect();

    let filtered_mice = mice.into_iter().enumerate()
        .filter(|(i, _)| !na_columns.contains(i))
        .map(|(_, mouse)| mouse.clone())
        .collect();

    let filtered_labels = labels.into_iter().enumerate()
        .filter(|(i, _)| !na_columns.contains(i))
        .map(|(_, label)| label.clone())
        .collect();

    let filtered_peptides = group.peptides.iter().map(|peptide| {
        let filtered_intensities = peptide.intensities.iter().enumerate()
            .filter(|(i, _)| !na_columns.contains(i))
            .map(|(_, &intensity)| intensity)
            .collect();

        Peptide {
            name: peptide.name.clone(),
            protein: peptide.protein.clone(),
            mass_charge_ratio: peptide.mass_charge_ratio,
            intensities: filtered_intensities,
        }
    }).collect();

    let columns_removed = original_columns_count - filtered_days.len();

    (filtered_days, filtered_mice, filtered_labels, filtered_peptides, columns_removed as u64)
}

pub fn serialize_calculations(path: &Path, calculations: &Vec<Calculation>) -> Result<(), Box<dyn std::error::Error>> {
    let mut wtr = Writer::from_path(path)?;
    wtr.write_record(&[
        "Protein",
        "Peptide",
        "NEH",
        "Charge",
        "Mean",
        "nRet",
        "MPE_0",
        "MPE_1",
        "Two_SD_Minus",
        "nRet",
        "Two_SD_Plus",
        "nRet",
        "",
    ]).map_err(|e| e.to_string())?;

    for calculation in calculations {
        let samples_omitted = if calculation.samples_omitted == 0 {
            "".to_string()
        } else {
            format!("{} sample{} omitted", calculation.samples_omitted, if calculation.samples_omitted > 1 { "s" } else { "" })
        };

        wtr.write_record(&[
            calculation.protein.clone(),
            calculation.peptide.clone(),
            calculation.neh.to_string(),
            calculation.charge.to_string(),
            calculation.mean.to_string(),
            calculation.n_ret_1.to_string(),
            calculation.mpe_0.to_string(),
            calculation.mpe_1.to_string(),
            calculation.two_sd_minus.to_string(),
            calculation.n_ret_2.to_string(),
            calculation.two_sd_plus.to_string(),
            calculation.n_ret_3.to_string(),
            samples_omitted,
        ])?;
    }

    wtr.flush()?;


    Ok(())
}