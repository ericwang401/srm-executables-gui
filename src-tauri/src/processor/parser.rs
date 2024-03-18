use std::path::Path;
use csv::ReaderBuilder;
use tokio::fs;
use tokio::fs::File;
use tokio::io::{AsyncSeekExt, AsyncWriteExt};


pub type Day = u64;

pub type Label = String;

pub struct Protein {
    pub name: String,
    pub peptide: String,
    pub charge_mass_ratio: f64,
    pub insensities: Vec<Option<u64>>,
}

pub async fn parse(spreadsheet: &Path)
                   -> Result<
                       (Vec<Day>, Vec<Label>, Vec<Protein>),
                       Box<dyn std::error::Error>
                   >
{
    let mut file = File::open(spreadsheet).await?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file.seek(1));


}