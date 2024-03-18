use std::io::Cursor;
use std::path::Path;
use csv::{Reader, ReaderBuilder};
use tokio::fs;
use tokio::fs::File;
use tokio::io::{AsyncSeekExt, AsyncWriteExt};


pub type Day = u64;

pub type Mouse = String;

pub type Label = String;

#[derive(Debug)]
pub struct Peptide {
    pub name: String,
    pub protein: String,
    pub charge_mass_ratio: f64,
    pub intensities: Vec<Option<u64>>,
}

pub async fn parse(spreadsheet: &Path)
                   -> Result<
                       (Vec<Day>, Vec<Mouse>, Vec<Label>, Vec<Peptide>),
                       Box<dyn std::error::Error>
                   >
{
    let contents = fs::read(spreadsheet).await?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(Cursor::new(contents));

    let (days, mice, labels) = extract_headers(&mut rdr)?;
    let peptides = extract_peptides(&mut rdr)?;

    Ok((days, mice, labels, peptides))
}

fn extract_peptides(rdr: &mut Reader<Cursor<Vec<u8>>>) -> Result<Vec<Peptide>, Box<dyn std::error::Error>> {
    let mut peptides = vec![];

    for result in rdr.records().skip(1) {
        let record = result?;
        let name = record[0].to_string();
        let protein = record[1].to_string();
        let charge_mass_ratio = record[2].parse::<f64>()?;
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
            charge_mass_ratio,
            intensities,
        });
    }

    Ok(peptides)
}

fn extract_headers(rdr: &mut Reader<Cursor<Vec<u8>>>) -> Result<(Vec<Day>, Vec<Mouse>, Vec<Label>), Box<dyn std::error::Error>> {
    let mut non_empty_row_count = 0;
    let mut days = vec![];
    let mut mice = vec![];
    let mut labels = vec![];

    for row in rdr.records() {
        let record = row?;
        if record.iter().any(|field| !field.is_empty()) {
            non_empty_row_count += 1;

            if non_empty_row_count == 1 {
                days = record
                    .iter()
                    .skip(3)
                    .map(|col| col.to_string().parse::<Day>().unwrap())
                    .collect::<Vec<_>>();
            }

            if non_empty_row_count == 2 {
                mice = record
                    .iter()
                    .skip(3)
                    .map(|col| col.to_string().parse::<Mouse>().unwrap())
                    .collect::<Vec<_>>();
            }

            if non_empty_row_count == 3 {
                labels = record
                    .iter()
                    .skip(3)
                    .map(|col| col.to_string().parse::<Label>().unwrap())
                    .collect::<Vec<_>>();
            }

            if non_empty_row_count == 4 {
                break;
            }
        }
    }

    Ok((days, mice, labels))
}