use std::collections::HashMap;
use crate::parser::Peptide;

#[derive(Debug, Clone)]
pub struct PeptideGroup {
    pub peptides: Vec<Peptide>,
    pub na_columns: Vec<bool>,
}

pub fn group_by_peptides(peptides: Vec<Peptide>) -> (Vec<PeptideGroup>, Vec<NAGroup>) {
    let mut sorted_peptides = peptides;
    sorted_peptides.sort_by(|a, b| {
        a.name.cmp(&b.name).then_with(|| {
            a.mass_charge_ratio.partial_cmp(&b.mass_charge_ratio).unwrap()
        })
    });

    let mut groups: Vec<PeptideGroup> = Vec::new();
    let mut separated_groups: Vec<NAGroup> = Vec::new();
    let mut current_group: Vec<Peptide> = Vec::new();

    for peptide in sorted_peptides {
        if current_group.is_empty() {
            current_group.push(peptide);
        } else if peptide.name == current_group.last().unwrap().name {
            // Dynamically determine the threshold based on the current group
            let threshold = dynamic_threshold(&current_group);
            let last_ratio = current_group.last().unwrap().mass_charge_ratio;
            if (peptide.mass_charge_ratio - last_ratio).abs() < threshold {
                current_group.push(peptide);
            } else {
                let peptide_group = create_peptide_group(&current_group);
                separated_groups.push(NAGroup {
                    groups: vec![peptide_group.clone()],
                    na_columns: peptide_group.na_columns.clone()
                });
                current_group = vec![peptide];
            }
        } else {
            // Different name, finalize current group and start a new one
            groups.push(create_peptide_group(&current_group));
            current_group = vec![peptide];
        }
    }

    if !current_group.is_empty() {
        groups.push(create_peptide_group(&current_group));
    }

    (groups, separated_groups)
}


fn dynamic_threshold(peptides: &[Peptide]) -> f64 {
    if peptides.len() <= 1 {
        return f64::INFINITY; // If there's only one peptide, no need for a threshold.
    }

    let mean: f64 = peptides.iter().map(|p| p.mass_charge_ratio).sum::<f64>() / peptides.len() as f64;
    let variance: f64 = peptides.iter()
        .map(|p| (p.mass_charge_ratio - mean).powi(2))
        .sum::<f64>() / (peptides.len() - 1) as f64;
    let std_deviation = variance.sqrt();

    std_deviation // You might want to scale this value based on your data.
}


// Helper function to create a PeptideGroup from a vector of peptides
fn create_peptide_group(peptides: &[Peptide]) -> PeptideGroup {
    let num_columns = peptides[0].intensities.len();
    let mut na_columns = vec![false; num_columns];

    // Check each column for NA values
    for peptide in peptides {
        for (i, intensity) in peptide.intensities.iter().enumerate() {
            if intensity.is_none() {
                na_columns[i] = true;
            }
        }
    }

    PeptideGroup { peptides: peptides.to_vec(), na_columns }
}

#[derive(Debug, Clone)]
pub struct NAGroup {
    pub groups: Vec<PeptideGroup>,
    pub na_columns: Vec<bool>,
}

pub fn group_by_na_columns(groups: Vec<PeptideGroup>) -> Vec<NAGroup> {
    let mut na_groups: HashMap<Vec<bool>, Vec<PeptideGroup>> = HashMap::new();

    // Group peptide groups by similar na_columns
    for group in groups {
        na_groups.entry(group.na_columns.clone())
            .or_insert_with(Vec::new)
            .push(group);
    }

    // Convert each group into a NAGroup with na_columns computed
    na_groups.into_iter().map(|(na_columns, groups)| {
        NAGroup { groups, na_columns }
    }).collect()
}