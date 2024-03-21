use std::io::Cursor;
use std::path::PathBuf;
use csv::ReaderBuilder;
use tokio::fs;

pub struct Calculation {
    pub protein: String,
    pub peptide: String,
    pub neh: String,
    pub charge: String,
    pub mean: String,
    pub n_ret_1: String,
    pub mpe_0: String,
    pub mpe_1: String,
    pub two_sd_minus: String,
    pub n_ret_2: String,
    pub two_sd_plus: String,
    pub n_ret_3: String,
    pub samples_omitted: u64,
}

pub async fn aggregate(spreadsheets: &Vec<(PathBuf, u64)>) -> Result<Vec<Calculation>, String> {
    let mut calculations = vec![];

    for spreadsheet in spreadsheets {
        let mut spreadsheet_calculations = parse_calculations(spreadsheet).await?;
        calculations.append(&mut spreadsheet_calculations);
    }

    Ok(calculations)
}

async fn parse_calculations(spreadsheet: &(PathBuf, u64)) -> Result<Vec<Calculation>, String> {
    let contents = fs::read(&spreadsheet.0).await.map_err(|err| format!("Error reading calculations file: {}", err))?;
    let mut rdr = ReaderBuilder::new()
        .from_reader(Cursor::new(contents));

    let mut calculations = vec![];

    for result in rdr.records() {
        let record = result.map_err(|err| format!("Error reading record: {}", err))?;
        let calculation = Calculation {
            protein: record[0].to_string(),
            peptide: record[1].trim().to_string(),
            neh: record[2].to_string(),
            charge: record[3].to_string(),
            mean: record[4].to_string(),
            n_ret_1: record[5].to_string(),
            mpe_0: record[6].to_string(),
            mpe_1: record[7].to_string(),
            two_sd_minus: record[8].to_string(),
            n_ret_2: record[9].to_string(),
            two_sd_plus: record[10].to_string(),
            n_ret_3: record[11].to_string(),
            samples_omitted: spreadsheet.1,
        };
        calculations.push(calculation);
    }

    Ok(calculations)
}