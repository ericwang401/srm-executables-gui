use std::io::Cursor;
use std::path::Path;

use csv::{ReaderBuilder, WriterBuilder};

// stitches the isolated peptide results back into the master output file
pub fn aggregate(
    master_output_contents: &mut Vec<u8>,
    peptide_output_file: &Path,
    samples_omitted: &u32,
) -> Result<(), String> {
    let mut master_reader = ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_reader(Cursor::new(master_output_contents.clone()));
    let mut master_writer = WriterBuilder::new().flexible(true).from_writer(Vec::new()); // NEEDED to write records with varying number of fields (https://docs.rs/csv/latest/csv/struct.WriterBuilder.html#method.flexible)

    let mut peptide_reader = ReaderBuilder::new().from_path(peptide_output_file).unwrap();

    let peptide = &peptide_reader.records().nth(0).unwrap().unwrap();

    for record in master_reader.records() {
        let mut record = record.unwrap();
        if record.get(1).unwrap() == peptide.get(1).unwrap() {
            record = peptide.to_owned();
            if *samples_omitted == 1 {
                record.push_field(&format!("{samples_omitted} sample was omitted"));
            } else {
                record.push_field(&format!("{samples_omitted} samples were omitted"));
            }
        }
        master_writer.write_record(record.iter()).unwrap();
    }

    master_writer.flush().unwrap();

    master_output_contents.clear();
    master_output_contents.append(&mut master_writer.into_inner().unwrap());

    Ok(())
}
