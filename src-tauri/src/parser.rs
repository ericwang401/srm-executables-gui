use std::io::Cursor;
use std::path::{Path, PathBuf};
use anyhow::anyhow;
use csv::{Reader, ReaderBuilder};
use serde::Deserialize;
use tokio::fs;
use crate::lib::serde::deserialize_path;

#[derive(Debug, Deserialize, Clone)]
pub struct InputFile {
    pub uuid: String,
    #[serde(deserialize_with = "deserialize_path")]
    pub path: PathBuf,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EngineType {
    Single,
    Multi,
}

pub type Day = u64;

pub type Mouse = String;

pub type Label = String;

#[derive(Debug, Clone)]
pub struct Peptide {
    pub name: String,
    pub protein: String,
    pub mass_charge_ratio: f64,
    pub intensities: Vec<Option<u64>>,
}

pub async fn parse(spreadsheet: &Path)
                   -> anyhow::Result<
                       (Vec<Day>, Vec<Mouse>, Vec<Label>, Vec<Peptide>)>
{
    let contents = fs::read(spreadsheet).await.map_err(|_| anyhow!("Failed to read file"))?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(Cursor::new(contents));

    let (days, mice, labels) = extract_headers(&mut rdr)?;
    let peptides = extract_peptides(&mut rdr)?;

    Ok((days, mice, labels, peptides))
}

fn extract_peptides(rdr: &mut Reader<Cursor<Vec<u8>>>) -> anyhow::Result<Vec<Peptide>> {
    let mut peptides = vec![];

    for result in rdr.records() {
        let record = result.map_err(|_| anyhow!("Failed to read row from spreadsheet"))?;
        let protein = record[0].to_string();
        let name = record[1].to_string();
        let charge_mass_ratio = record[2].parse::<f64>().map_err(|_| anyhow!("Failed to parse charge/mass ratio"))?;
        let intensities = record.iter().skip(3).map(|value| {
            if value == "#N/A" {
                None
            } else {
                value.parse::<u64>().ok()
            }
        })
            .collect::<Vec<Option<u64>>>();

        peptides.push(Peptide {
            name,
            protein,
            mass_charge_ratio: charge_mass_ratio,
            intensities,
        });
    }

    Ok(peptides)
}

fn extract_headers(rdr: &mut Reader<Cursor<Vec<u8>>>) -> anyhow::Result<(Vec<Day>, Vec<Mouse>, Vec<Label>)> {
    let mut non_empty_row_count = 0;
    let mut days = vec![];
    let mut mice = vec![];
    let mut labels = vec![];

    for row in rdr.records() {
        let record = row.map_err(|_| anyhow!("Failed to read row from spreadsheet"))?;
        if record.iter().any(|field| !field.is_empty()) {
            non_empty_row_count += 1;

            if non_empty_row_count == 1 {
                for col in record.iter().skip(3) {
                    let day: Day = col.to_string().parse().map_err(|_| anyhow!("Failed to parse day"))?;
                    days.push(day);
                }
            } else if non_empty_row_count == 2 {
                for col in record.iter().skip(3) {
                    let mouse: Mouse = col.to_string().parse().map_err(|_| anyhow!("Failed to parse mouse"))?;
                    mice.push(mouse);
                }
            } else if non_empty_row_count == 3 {
                for col in record.iter().skip(3) {
                    let label: Label = col.to_string().parse().map_err(|_| anyhow!("Failed to parse label"))?;
                    labels.push(label);
                }
            }

            if non_empty_row_count >= 4 {
                break;
            }
        }
    }

    Ok((days, mice, labels))
}
