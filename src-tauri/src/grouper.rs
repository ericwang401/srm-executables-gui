use std::collections::HashMap;

use crate::parser::Peptide;

#[derive(Debug, Clone)]
pub struct PeptideGroup {
    pub peptides: Vec<Peptide>,
    pub na_columns: Vec<bool>,
}

pub fn group_by_peptides(tolerance_multiplier: f64, peptides: Vec<Peptide>) -> Vec<PeptideGroup> {
    let mut sorted_peptides = peptides;
    sorted_peptides.sort_by(|a, b| {
        a.name.cmp(&b.name).then_with(|| {
            a.mass_charge_ratio.partial_cmp(&b.mass_charge_ratio).unwrap()
        })
    });

    let mut groups: Vec<PeptideGroup> = Vec::new();
    let mut current_group: Vec<Peptide> = Vec::new();

    for peptide in sorted_peptides {
        if current_group.is_empty() {
            current_group.push(peptide);
        } else if peptide.name == current_group.last().unwrap().name {
            // Dynamically determine the threshold based on the current group
            let threshold = calc_std_deviation(&current_group) * tolerance_multiplier;
            let last_ratio = current_group.last().unwrap().mass_charge_ratio;
            if (peptide.mass_charge_ratio - last_ratio).abs() < threshold {
                current_group.push(peptide);
            } else {
                groups.push(create_peptide_group(&current_group));
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

    groups
}


fn calc_std_deviation(peptides: &[Peptide]) -> f64 {
    if peptides.len() <= 1 {
        return f64::INFINITY; // If there's only one peptide, no need for a threshold.
    }

    let mean: f64 = peptides.iter().map(|p| p.mass_charge_ratio).sum::<f64>() / peptides.len() as f64;
    let variance: f64 = peptides.iter()
        .map(|p| (p.mass_charge_ratio - mean).powi(2))
        .sum::<f64>() / (peptides.len() - 1) as f64;
    let std_deviation = variance.sqrt();

    std_deviation * 2.0 // You might want to scale this value based on your data.
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
    pub peptides: Vec<Peptide>,
    pub na_columns: Vec<bool>,
}

pub fn group_by_na_columns(groups: Vec<PeptideGroup>) -> Vec<NAGroup> {
    let mut na_groups: HashMap<(Vec<bool>, u64), Vec<Peptide>> = HashMap::new();

    for group in groups {
        let mut count = 1;
        loop {
            let key = (group.na_columns.clone(), count);
            if let Some(peptide_group) = na_groups.get(&key) {
                let name = group.peptides[0].name.clone();

                if let Some(_) = peptide_group.iter().find(|&x| x.name == name) {
                    count += 1;
                } else {
                    na_groups.entry(key).or_insert_with(Vec::new).extend(group.peptides);
                    break;
                }
            } else {
                na_groups.insert(key, group.peptides);
                break;
            }
        }
    }

    // Convert each group into a NAGroup with na_columns computed
    na_groups.iter().map(|((na_cols, _), peptides)| {
        NAGroup {
            peptides: peptides.to_vec(),
            na_columns: na_cols.to_vec(),
        }
    }).collect()
}